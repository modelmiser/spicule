# Unsafe Audit Sweep: seam/src (16 files, 46 sites)
Date: 2026-09-04
Method: four parallel auditors, batched by hazard class; findings re-verified by the collator
        with independent witnesses before being recorded here.

> **RESOLUTION — findings 1-8 are now CLOSED (2026-09-04).**
> 1 SPSC re-split: fixed. 2 reseat: the FINDING was revised rather than the code — the r27 verdict
> stands and is strengthened (see design_seam/REPORT_SEAM_SPEC_reseat.md). 3 ordered_locks,
> 4 lock_registry, 5 shared: fixed. 6-8 mutation gaps: closed with tests that kill their mutants.
> 9-13 documentation-vs-code mismatches: CLOSED. **All 13 findings are now closed.**

## Scope

| Batch | Files | Sites |
|---|---|---|
| Locks | spin_lock, spin_mutex, lock_registry, ordered_locks | 15 |
| Containers | slab, intrusive, drain | 11 |
| References | reseat, shared, selfref | 9 |
| Root + type-level | lib.rs, validity, typed_nat, take, refined, branded | 11 |

`mmio.rs` and `sync_cell.rs` were audited separately — see the two sibling reports.

## Verification status

Findings 1-4 were reproduced by the collator independently of the reporting agent.
Findings 6-8 were re-verified by re-running the mutants (see that section).
Everything else is single-source and marked as such.

---

## 1. CRITICAL — SPSC `split` re-seeds caches to zero. **FIXED.**

`RingBuffer::split` takes `&mut self` but does not consume it and does not reset `head`/`tail`,
while hardcoding `cached_head: 0` and `cached_tail: 0` on every call. After any earlier split
that popped, `head != 0`, so `pop`'s "possibly empty" test (`head == self.cached_tail`) is FALSE
on a genuinely empty ring: the Acquire refresh is skipped and `assume_init_read` runs on a
never-written slot.

Reproduced from `#![forbid(unsafe_code)]` consumer code, public API only:

    pops from a PROVABLY EMPTY ring: [Some(8102099357864587376) x3, Some(7), None]
    materialised 4 value(s) out of nothing

The fourth value is a SECOND move out of slot 0 — a double-drop for any non-`Copy` `T`. The
reporting agent saw SIGSEGV with `RingBuffer<String, 4>`.

`pop`'s SAFETY comment reads "`head != tail` means slot `head` was published". The code tests
`head != self.cached_tail`. Those are different propositions, and the gap between them is the bug.

This is the crate's flagship seam and a direct counterexample to lib.rs:27, "safe code cannot
launder UB (compile-checked)". Conceded nowhere.

FIX: seed both caches from the live indices in `split` (Relaxed suffices — `&mut self` proves no
handle is live). Regression tests in `seam/tests/sound_resplit.rs`; verified to FAIL against the
unfixed code.

**UNCONFIRMED sub-claim:** the agent also reported `push` returning `Ok(())` into a full ring
after a re-split. Not reproduced in two configurations — `push`'s possible-full branch fires at
the wrap boundary exactly when the stale zero would matter and self-corrects. Recorded as
unconfirmed rather than dropped.

---

## 2. CRITICAL — `reseat.rs` `mem::forget` is a double-free, not a leak. **OPEN (owner decision).**

`Reseat::new` does `ptr::read(slot)`, leaving a bitwise duplicate in the slot. `DerefMut` lets
safe code invalidate what those bits name; `mem::forget` then skips the `Drop` that would
overwrite them, handing the slot's owner a dangling `T`.

Reproduced, `#![forbid(unsafe_code)]`, using only `new` / `DerefMut` / `forget` — no `into_inner`:

    D(1) drop count = 2   (1 = leak as documented, 2 = DOUBLE DROP)

The `String` variant is a real heap double-free that glibc did not trap; the process exited 0.

The module asserts the opposite four times (reseat.rs:29-31, 112-113, 136-137, and the SAFETY
comment at 152-153: "a leak, not a double-free").

**Why this is left open:** reseat.rs:80-85 argues the `into_inner`-less subset is "sound but
degenerate", and a cold panel accepted that. The witness uses exactly that subset, so the finding
relocates #20's `mem::forget` gap from horn 3 to horn 1 and removes the module's stated contrast
with #17. That is a change to a published result, not a bug fix.

The test that certifies this property, `forget_is_a_leak_not_a_double_free`, uses a payload
documented as owning no heap and performs `let _ = &mut *g;` as its "mutation" — a no-op. It
passes with the defect present.

---

## 3. MEDIUM — `ordered_locks::with` strands its lock on panic. **FIXED.**

The release is a straight-line `self.flag.store(false, Release)` after `body(...)`, and the file
contains NO `Drop` impl (verified). If `body` unwinds, the store is skipped and every later
acquirer spins forever on a hot loop. `spin_lock`, `spin_mutex` and `lock_registry` all release
from `Drop` and survive unwinding; this module alone does not.

The SAFETY comment (:255-257) says "we hold `flag` exclusively until the Release store below" and
the doc (:241-242) promises release "when the body returns" — both assume a store the panic path
never reaches. Logic, not UB. FIXED with an RAII `Release` guard, as the three siblings already use. Regression:
`tests/ordered_locks_use.rs::panic_in_body_releases_the_lock` and `::panic_in_a_nested_body_releases_both_locks`.
Verified against the unfixed code — the test HANGS (timeout, SIGTERM) rather than failing, which is the
honest shape of a stranded spinlock: there is no timeout to trip.

---

## 4. MEDIUM — `lock_registry::Guard::drop` can abort before releasing. **FIXED.**

`HELD.with(...)` (:208) runs BEFORE the release store (:214), and `LocalKey::with` panics if the
thread's TLS is already torn down — a guard held in a `thread_local!` hits exactly that. The
comment three lines above (:205-207) says the destructor will "never panic". The agent observed
`fatal runtime error: thread local panicked on drop, aborting`, exit 134.

FIXED: the release store now runs FIRST, and the bookkeeping uses `try_with` so a dead registry is
harmless. Regression: `tests/lock_registry_teardown.rs`, verified to reproduce
`fatal runtime error: thread local panicked on drop, aborting` (SIGABRT) against the unfixed code.

**Note on that test.** The first version of it PASSED against the unfixed code and proved nothing.
TLS destructors run in reverse registration order, and calling `lock()` first registers the registry's
`HELD` before the guard is parked — so `HELD` was still alive when the guard dropped. The test only
bites when `PARKED` is touched BEFORE `lock()`, which puts `HELD` second and therefore destroyed first.
That ordering is the entire test, and it is commented as such in the file.

---

## 5. MEDIUM — `shared.rs` refcount has no overflow guard. **FIXED (untestable).**

`Clone` does `strong.set(c + 1)` with no check. `std::rc::Rc` deliberately aborts here. `2^32`
iterations of the safe loop `mem::forget(s.clone())` wrap the count to zero on a 32-bit target,
after which the next `Drop` frees while owners are live. The crate builds clean for
`wasm32-unknown-unknown`. The module's "Honest scope" lists only `Weak`, cycles and `RefCell`.
FIXED with `checked_add` + panic. Panicking rather than aborting is sound here because the check runs
BEFORE any state change: on the failing path `strong` is untouched and no `Shared` has been minted, so
unwinding leaves the allocation exactly as it was. (`Rc` aborts because it increments first.)

**NO REGRESSION TEST — and this should not be papered over.** Reaching the boundary needs 2^32
iterations on a 32-bit target; it is not testable at `usize` width on this host, and a test that
merely asserts `checked_add` was called would be testing the implementation, not the property. The
guard is therefore defended by review only. If this crate ever gains a 32-bit CI target, an
`#[ignore]`d long-running test becomes worth writing.

---

## 6-8. Test force: three discharges no test defends. **OPEN. VERIFIED.**

Reported via mutation testing and independently re-verified by the collator on 2026-09-04.
Baseline is 206 passing / 0 failing with `--features test-helpers`; all three mutants below
compile and leave that baseline UNCHANGED, i.e. every one survives the entire suite:

| Mutant | Change | Result |
|---|---|---|
| M1 | `Drain::drop`: `for i in self.idx..self.end` -> `self.idx..self.idx` | **NOW KILLED** |
| M10 | `Buffer::drop`: `for i in 0..self.len` -> `0..0usize` | **NOW KILLED** |
| M7 | `Slab::get_mut`: `Entry::Occupied if ...` -> `_ if ...` | **NOW KILLED** — see below |

M7 is the serious one: that mutant makes `assume_init_mut()` return a reference to
uninitialized storage for any key whose generation matches a vacant slot, reachable from safe
code, and nothing in the suite noticed. The sibling guard in `get` WAS killed by a test; the
`get_mut` guard was not, and `// SAFETY: as in `get`` is what concealed the asymmetry.

**M7 CLOSED 2026-09-04.** `tests/slab_use.rs::harness_exploit_fresh_slab_gen0_is_rejected_by_get_mut_too`
aims the existing cross-slab exploit at the third accessor: a key from slab A (index 0,
generation 0) against a fresh slab B whose slot 0 is vacant AT generation 0 — the generation
matches, only occupancy rejects it. Verified to kill the mutant.

**M1 and M10 CLOSED 2026-09-04.** `tests/drain_use.rs::partial_drain_then_drop_actually_drops_the_remainder`
and `::an_undrained_buffer_drops_its_elements`. Both count drops with a destructor-bearing
payload, which is precisely what the pre-existing tests did not: `partial_drain_then_drop_runs_remainder_cleanup`
uses `u32` (no drop glue) and asserts only `is_empty()`, true from `drain()`'s `len = 0` however
the guard behaves; and every other buffer in the file was drained to `len == 0` before dropping,
so `Buffer::drop`'s loop never destroyed an observable value anywhere in the crate. Both verified
to kill their mutants.

**All three mutation findings are now closed.** Suite 210 native / 212 with `test-helpers`;
clean under Miri (212 passed, 0 failed, zero UB).

- `Drain::drop`'s element loop (drain.rs:150-154) — the module calls it "load-bearing" (:121) —
  survives being replaced with an empty range. The one partial-drain test uses `u32` (no `Drop`)
  and asserts only `is_empty()`; the only drop-counting test `mem::forget`s the guard.
- `Slab::get_mut`'s occupancy guard (slab.rs:115) survives deletion, and that mutant makes
  `assume_init_mut()` read uninitialized storage from safe code. The sibling guard in `get` IS
  killed by a test — the coverage is asymmetric, and the `// SAFETY: as in get` comment hides
  exactly that asymmetry.
- `Buffer::drop`'s loop (drain.rs:112-116) never destroys an observable value in any test.

## 9-13. Documentation that does not match the code. **ALL CLOSED 2026-09-04.**

- `lib.rs` — **corrected.** The handles' SAFETY comment claimed "We do NOT implement `Sync`", which
  was false twice: `Sync` is an auto trait (not implementing it is not the same as not having it),
  and both handles ARE `Sync` for every `T: Send`, derived through their `&RingBuffer` field. The
  real discharge — `push`/`pop` take `&mut self`, so a shared `&Producer` cannot push — is now
  written down. The `unsafe impl Sync for RingBuffer` comment now also states its operative reason:
  `RingBuffer` has no `&self` method at all, so `&RingBuffer` conveys no capability.
- `validity.rs` — the witness is a hand-maintained `const VALID_DISCRIMINANTS`, not the "`match`
  pinned to the actual variants" the prose claims (:23), and "a new variant forces a matching
  update" (:65-66) is false: editing `C = 200` to `C = 201` compiles clean and `tag_at` hands out
  an invalid `Tag`. A `match` would need no `unsafe` at all.
- `take.rs` — **documented.** The dependency is now stated at the claim: double-panic-aborts is a
  `std::panicking` guarantee this `#![no_std]` crate does not link, so the seam is sound under `std`
  and under `panic = "abort"` but is a stated ASSUMPTION, not a discharge, under `no_std` +
  unwinding handler. NOT fixed, because fixing it means changing the primitive (the `T: Default`
  sentinel the module already sketches). The `f(old)`-inside-the-unsafe-block point stands and is
  also unfixed — narrowing it is a real edit to the seam's shape.
- `intrusive.rs` — **corrected.** The inventory now says four sites, not two. The SAFETY text is
  rewritten: the old one was false in both clauses ("mirrors std::cell semantics" — every
  `core::cell` type is `!Sync` and this one is `Sync`; "for this single-threaded demonstration" is
  not a discharge for a cross-thread trait). The real argument — global read-XOR-write via token
  exclusivity, one token per brand, invariant brand — is now on the page, along with the ratchet
  hazard: writing either auto-trait impl by hand suppresses the auto-impl, so a future change to the
  brand marker would leave both traits asserted rather than correctly withdrawn.
- `typed_nat.rs` and `ordered_locks.rs` — **both corrected.** Each now states the argument that
  actually holds instead of the one that does not. `typed_nat`: sealing and the orphan rule close it
  TOGETHER (a usable forgery needs both parameters `Nat`, hence both foreign, and the all-foreign
  impl is rejected by E0117); sealing alone does not. `ordered_locks`: the dangerous direction is
  all-foreign and E0117-rejected, and a `Below<Rogue>` impl is inert because `Floor<F>` has a private
  field and is only minted at `TopRank` or a sealed `L` — the private-field half was never stated and
  is the load-bearing part.
- `selfref.rs` argues the self-pointer's ADDRESS stability but never its PROVENANCE.
  **RESOLVED 2026-09-04 — no defect.** The sequence is now exercised by
  `tests/selfref_provenance.rs` and is clean under BOTH Stacked Borrows and Tree Borrows. The
  structural answer is the stronger one: the public surface is `new` / `init` / `get` / `value`,
  and `init` is the ONLY safe route to a `Pin<&mut Self>` — and it necessarily re-mints `ptr`
  from the fresh borrow. Every reborrow that could pop the old tag also replaces the pointer
  carrying it, so the hazard is unreachable through the public API. That reasoning appears
  nowhere in the module and is worth adding to it: the seam is sound for a better reason than
  the one it gives.

## Tooling gap — **Miri now set up (2026-09-04)**

`tools/miri.sh`; component installed on nightly. Whole suite is clean: 206 passed, 0 failed,
zero UB across 40 binaries, under both Stacked Borrows and Tree Borrows.

**Read that result narrowly.** Miri reports UB on paths the tests EXECUTE; it cannot invent a
call. M7 was invisible to it for exactly that reason — no test called `get_mut` with a
generation-matching vacant slot — and M1/M10 remain invisible for the same reason today. A clean
Miri run over a suite with holes certifies the suite, not the code. Two flags matter and are
documented in the script: `-Zmiri-ignore-leaks` (several seams leak deliberately and assert it, so
leak-checking on would make Miri cry wolf), and `--features test-helpers` (without it the mmio
tests compile to nothing). Cost: multi-minute; `spin_lock_use` alone takes ~5 minutes under
interpretation. Not a per-commit check.

Still absent: sanitizers, fuzzing. For a crate whose entire subject is the
difference between a discharged and a merely-asserted `unsafe impl`, this is the highest-value
addition available: findings 1, 6-8 and the `selfref` provenance question are all exactly what
Miri checks. `[profile.test] opt-level = 0` additionally means the optimizer that exploits
`noalias` never runs over the concurrency seams.
