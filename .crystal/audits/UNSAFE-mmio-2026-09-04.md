# Unsafe Audit: seam/src/mmio.rs
Date: 2026-09-04
Mode: standard

> **RESOLUTION — all findings closed 2026-09-04.** This report describes the code as it stood
> BEFORE the fixes below; do not read its verdicts as claims about HEAD.
>
> | Finding | Fix |
> |---|---|
> | SITE 6 `__reset_for_test` revokes M3 | gated behind `test-helpers` (`38be142`) |
> | SITE 5 `__device_raise` races REGION | gated behind `test-helpers` (`38be142`) |
> | SITE 1 `unsafe impl Sync` undischarged | follows from the two gates (`38be142`) |
> | SITE 6 unsafe block had no `// SAFETY:` | added (`38be142`) |
> | M3 break unpinned | witness test `tests/mmio_witness.rs` (`ec510e8`) |
> | SITES 2-3 lost-event window | documented in module + site docs (`cd0cf73`) |
>
> Under default features SITES 1-4 are now SOUND and SITES 5-6 do not exist.

## Summary

| Metric | Count |
|---|---|
| Unsafe sites | 6 |
| SOUND | 0 |
| REVIEW | 3 |
| UNSOUND | 3 |
| UNPROVEN | 0 |

Blocks: 5   Impls: 1   Functions: 0   Traits: 0

**Headline.** Unlike `sync_cell.rs`, this seam is *meant* to be sound, and its core is: the
affine singleton handle, the encapsulated volatile access, and the honest naming of M1/M4 as
an axiom outside the formal system are all correct. The soundness argument is defeated not by
the register access but by the module's own two `#[doc(hidden)]` test helpers, which are
unconditionally `pub` and safe, and which between them dismantle the exact invariant (M3,
at-most-one-owner) that discharges the `unsafe impl Sync`.

## Findings

### SITE 6: mmio.rs:95-98 (`__reset_for_test`) — UNSOUND (root cause)
Kind: `pub fn` containing an `unsafe` block
Code: `TAKEN.store(false, Ordering::Release); ... unsafe { write_volatile(p, 0) }`

INVARIANT: thread safety / aliasing — M3 states at most one `StatusReg` exists at a time,
and M3 is the sole discharge for SITE 1's `unsafe impl Sync`.
Evidence chain: `take()` (:54) enforces M3 via `TAKEN.swap(true, AcqRel)` — a genuine runtime
singleton. `__reset_for_test` sets `TAKEN = false` while an existing `StatusReg` may still be
alive. A second `take()` then succeeds, yielding TWO live handles, each handing out `&mut self`
to the same `REGION`. M3 is false from that point on; every `&mut`-based exclusivity claim in
the module rests on it.
Reachability: `pub mod mmio;` (lib.rs:87), helper is `pub`, NOT `#[cfg(test)]`-gated.
`#[doc(hidden)]` hides it from rustdoc; it does not restrict access. Called from
tests/mmio_use.rs:15 — an integration test, i.e. an external crate, which is precisely why it
cannot be `#[cfg(test)]` and precisely why it is reachable by any other consumer.
Documentation: UNSTATED. The `unsafe` block at :98 has no `// SAFETY:` comment at all, and
nothing anywhere notes that this function invalidates M3.

BLAST RADIUS: UB-aliasing (two `&mut` paths to one `UnsafeCell<u32>`)
Rating: HIGH
Multipliers: library — capped by `publish = false` (seam/Cargo.toml:6) to in-repo consumers.

SAFE ALTERNATIVE: REDUCIBLE, cheaply. Put both helpers behind `#[cfg(feature = "test-helpers")]`
and have the integration test enable the feature. Cost: one Cargo feature. The seam's
demonstration is untouched.

TEST COVERAGE: UNTESTED for the failure mode. tests/mmio_use.rs uses the helper for its
intended purpose only; nothing exercises reset-while-held.
Gaps: a test taking a handle, calling `__reset_for_test`, and taking a second handle would
witness the double-owner in three lines.

VERDICT: UNSOUND

---

### SITE 5: mmio.rs:84-91 (`__device_raise`) — UNSOUND
Kind: `pub fn` containing an `unsafe` block
Code: `unsafe { let cur = read_volatile(p); write_volatile(p, cur | bits); }`

INVARIANT: thread safety — a non-atomic read-modify-write on `REGION`, which is `Sync` (SITE 1)
and therefore shareable across threads.
Evidence chain: nothing establishes exclusivity here. The function takes no `&mut StatusReg`
and consults no handle. Two concurrent `__device_raise` calls, or one concurrent with a
handle-holder's `read_status`, is a data race on a non-atomic `u32` — UB, reachable with zero
consumer `unsafe`.
Documentation: WRONG (structurally). `// SAFETY: single-threaded test helper standing in for a
hardware event` is a precondition stated on a SAFE `pub fn` — the anti-pattern named in the
audit reference (safe fns cannot carry preconditions). If single-threadedness is required, this
must be an `unsafe fn` or be feature-gated out of the public surface.

BLAST RADIUS: UB-memory (racing non-atomic RMW)
Rating: HIGH
Multipliers: library, `publish = false`.

SAFE ALTERNATIVE: REDUCIBLE — same feature gate as SITE 6. Alternatively make `REGION` an
`AtomicU32`, which would discharge SITES 1, 5 and 6 outright at the cost of blurring the
"volatile MMIO" demonstration the seam exists to show. The feature gate is the better fix here
because it preserves the point of the artifact.

TEST COVERAGE: PARTIAL — used once (tests/mmio_use.rs:25) single-threaded. No concurrency test,
no Miri, no sanitizers anywhere in the repo.

VERDICT: UNSOUND

---

### SITE 1: mmio.rs:30 — UNSOUND (as written; SOUND once 5 and 6 are gated)
Kind: `unsafe impl`
Code: `unsafe impl Sync for Mmio {}`

INVARIANT: thread safety — `&Mmio` may be shared across threads only if all access to the inner
`UnsafeCell<u32>` is serialised.
Evidence chain: the comment at :28-29 discharges this against M3 (single-owner, enforced by the
affine singleton). That reasoning is CORRECT for `StatusReg`'s own methods — `read_status` and
`configure` both take `&mut self`, and `take()` genuinely enforces one live handle. It is false
for the module as a whole, because SITE 5 accesses `REGION` with no handle and SITE 6 revokes
the singleton.
Documentation: STATED. The comment names the right invariant and the right mechanism; it is
incomplete rather than wrong — it does not mention the two bypasses.

BLAST RADIUS: UB-memory
Rating: HIGH
Multipliers: library, `publish = false`.

SAFE ALTERNATIVE: NECESSARY. A `static` MMIO region genuinely requires this impl; the fix is to
close the bypasses, not to remove the impl.

TEST COVERAGE: UNTESTED for the thread-safety claim. No test shares the region across threads —
which is unsurprising in a `no_std` crate, and is the reason the gap is invisible.

VERDICT: UNSOUND (inherited from SITES 5-6 — the impl itself is the right call)

---

### SITE 2: mmio.rs:69 (`read_status`, volatile read) — REVIEW
### SITE 3: mmio.rs:70 (`read_status`, read-to-clear store) — REVIEW
Kind: `unsafe` blocks
Code: `let v = unsafe { read_volatile(p) };` / `unsafe { write_volatile(p, 0) }`

INVARIANT: pointer validity + aliasing — `p` from `UnsafeCell::get()` on a `static` is non-null,
aligned and initialized (`UnsafeCell::new(0)`), so validity holds unconditionally. Exclusivity
comes from `&mut self` plus the singleton.
Evidence chain: locally correct. The `// SAFETY (M2)` comment at :65-67 is the best in the file —
it names the mechanism, the exclusivity source, AND flags M1/M4 as the trusted axiom rather than
pretending to discharge it. That last part is exactly right: "this address really is the device"
is outside any checker, and saying so is more honest than a confident SAFETY comment would be.
Documentation: PROVEN.

BLAST RADIUS: UB-memory if exclusivity fails — but see below for a non-UB issue.
Rating: MEDIUM (the local reasoning is sound; the premise is falsifiable elsewhere)
Multipliers: library, `publish = false`.

**Separate non-UB finding (Logic).** The read-to-clear is modelled as read-then-write-zero, two
separate volatile operations. Any `__device_raise` landing between :69 and :70 has its bits
silently discarded by the store — a lost-interrupt window. Real read-to-clear hardware clears
atomically as a side effect of the read, so this is an artifact of the stand-in, not of the
design being demonstrated. The comment says "modelled by the store" and so is aware of the
substitution, but not of the dropped-event window it introduces. Worth one sentence in the
module prose, since a reader could carry the pattern to real hardware.

SAFE ALTERNATIVE: NECESSARY — volatile access is the point of the seam.

TEST COVERAGE: PARTIAL — happy path only, single-threaded. No Miri (which would exercise the
aliasing claim), no test for the lost-bits window.

VERDICT: REVIEW — sound in isolation, defeated only by SITES 5-6. These become SOUND the moment
the helpers are gated.

---

### SITE 4: mmio.rs:78 (`configure`, volatile write) — REVIEW
Kind: `unsafe` block
Code: `unsafe { write_volatile(p, cfg.0) }`

INVARIANT / Evidence / Documentation: identical to SITES 2-3. `// SAFETY (M2)` at :76 names
exclusivity via `&mut self` + singleton. PROVEN.

BLAST RADIUS: UB-memory if exclusivity fails. Rating: MEDIUM. Multipliers: library.

SAFE ALTERNATIVE: NECESSARY.

TEST COVERAGE: PARTIAL — same gaps.

VERDICT: REVIEW — same disposition as SITES 2-3.

## Recommendations

1. **Gate both helpers.** `#[cfg(feature = "test-helpers")]` on `__device_raise` and
   `__reset_for_test`, with `seam/tests/mmio_use.rs` enabling the feature. This single change
   moves SITES 1, 5 and 6 from UNSOUND and promotes SITES 2-4 from REVIEW to SOUND. It is the
   highest-value edit in the file by a wide margin.
2. **Add the witness test before fixing.** Take a handle, `__reset_for_test()`, take a second
   handle, assert both are `Some`. Three lines, and it makes the M3 violation a fact in CI
   rather than a claim in an audit.
3. **Add a `// SAFETY:` comment to `__reset_for_test`'s unsafe block (:98).** It is the only
   unsafe block in the file with no comment — and, as it turns out, the most dangerous one.
4. **Note the lost-interrupt window** introduced by modelling read-to-clear as two operations
   (SITES 2-3), so the pattern is not carried to real hardware unexamined.

## Tooling Gaps

No Miri, no sanitizers, no fuzzing anywhere in the repo (zero hits across *.toml, *.yml,
*.yaml). Miri is the single highest-value addition for this file specifically: SITES 2-4's
exclusivity claim and SITE 1's `Sync` discharge are exactly the class of property Miri checks
at runtime, and a Miri run over a test that reset-then-retakes the handle would flag the
double-owner immediately. For a crate whose entire subject is the difference between a
*discharged* and a *merely asserted* `unsafe impl`, having no UB-detection tooling is the
gap most worth closing.
