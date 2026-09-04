# Unsafe Audit: seam/src/sync_cell.rs
Date: 2026-09-04
Mode: standard

> **RESOLUTION — one finding still OPEN as of 2026-09-04.**
>
> SITES 1-3 are the seam's deliberate, conceded break; no action intended or taken.
>
> **SITE 4 (`unsafe impl<T> Send`, unbounded) is OPEN.** `Copy` does not imply `Send`, so
> `SyncCell<*const u8>` is constructible and unsoundly `Send` — a hole the module prose does
> not mention and that appears unintended, unlike the `Sync` break. Either bound it
> `unsafe impl<T: Send> Send` (which leaves the intended break entirely intact) or document
> it as a second deliberate hole. Silence is the one option that is wrong.

## Summary

| Metric | Count |
|---|---|
| Unsafe sites | 4 |
| SOUND | 0 |
| REVIEW | 0 |
| UNSOUND | 4 |
| UNPROVEN | 0 |

Blocks: 2   Impls: 2   Functions: 0   Traits: 0

**Framing.** This file is a deliberately unsound research artifact — the module prose
(lines 31-73) states the break, concedes it as pre-conceded folklore, and records the
refuted contribution. An audit that "discovers" the `Sync` unsoundness is telling the
author what they wrote down. The findings worth their ink are SITE 4 (an unsoundness the
prose does NOT mention) and the structural anti-pattern in SITES 1-2.

## Findings

### SITE 3: sync_cell.rs:128 — UNSOUND (root cause)
Kind: `unsafe impl`
Code: `unsafe impl<T> Sync for SyncCell<T> {}`

INVARIANT: thread safety — `&SyncCell<T>` may be shared across threads only if all
`&self` operations are race-free. `set` (line 117) performs a NON-ATOMIC write through
`UnsafeCell::get()`. The obligation is not discharged; it is deleted. The impl is
unconditional — no `T: Sync` bound, no atomics.
Evidence: `set` at :117-122 writes through `*self.inner.get()`; nothing serialises it.
Documentation: PROVEN — and unusually so. The comment at :125-127 states the invariant
AND states that it is false. Correct documentation of unsound code.

BLAST RADIUS: UB-memory (a data race is UB)
Rating: HIGH
Multipliers: library — but `publish = false` (seam/Cargo.toml:6) caps downstream reach.
Not reachable from untrusted input. Not a hot path.

SAFE ALTERNATIVE: UNNECESSARY for the engineering goal — `AtomicI32`/`Mutex` do this
soundly, and the crate's own test asserts it (`the_floor_already_vends_the_sound_version`,
tests/sync_cell_use.rs:40). NECESSARY for the artifact's purpose: the unsoundness IS the
experiment. Do not "fix" this without retiring the seam.

TEST COVERAGE: PARTIAL
Gaps: `synccell_is_unsoundly_sync_and_send` (tests/sync_cell_use.rs:29) witnesses the hole
at TYPE level only. The runtime race is a `no_run` doctest (:83-91) — it compiles, never
executes. No test ever runs the UB. No Miri, no sanitizers, no fuzzing anywhere in the
repo (grep over *.toml/*.yml/*.yaml: zero hits). For a deliberate-UB artifact this is
arguably correct, but it should be a stated decision, not an absence.

VERDICT: UNSOUND (intentional, documented, conceded)

---

### SITE 4: sync_cell.rs:129 — UNSOUND (not covered by the module prose)
Kind: `unsafe impl`
Code: `unsafe impl<T> Send for SyncCell<T> {}`

INVARIANT: thread safety — `SyncCell<T>` may be moved to another thread only if `T: Send`.
The impl is unconditional, so it also grants `Send` for `T: !Send`.
Evidence: no bound on the impl. Reachability is narrowed but NOT closed by `new` being in
`impl<T: Copy>` (:103) with `inner` private — external code cannot construct `SyncCell<T>`
for arbitrary `T`. However `Copy` does not imply `Send`: `SyncCell<*const u8>` is
constructible and is unsoundly `Send`.
Documentation: UNSTATED for this specific hole. The module's 60 lines of analysis are
about `Sync` and the racing write; `Send` appears only in passing (:16, :126). The
raw-pointer case is not mentioned anywhere.

BLAST RADIUS: UB-memory / UB-aliasing — a `*const u8` sent across threads can outlive or
alias what it points to.
Rating: HIGH
Multipliers: same as SITE 3 (`publish = false`).

SAFE ALTERNATIVE: REDUCIBLE at zero cost to the experiment. The probe is about `Sync` and
cross-thread SHARING; bounding the Send impl as `unsafe impl<T: Send> Send for SyncCell<T>`
would discharge it soundly and leave the intended break entirely intact. That is precisely
the discharged-vs-false distinction the module itself draws at :48-49.

TEST COVERAGE: PARTIAL — `assert_send::<SyncCell<i32>>()` covers only `T = i32`, which is
`Send` anyway. No test exercises a `Copy + !Send` T.
Gaps: `assert_send::<SyncCell<*const u8>>()` would witness this hole the way
`synccell_is_unsoundly_sync_and_send` witnesses the other.

VERDICT: UNSOUND (appears unintentional — the one finding here the author has not already
written down)

---

### SITE 1: sync_cell.rs:113 — UNSOUND (inherited)
Kind: `unsafe` block, in `pub fn get(&self) -> T`
Code: `unsafe { *self.inner.get() }`

INVARIANT: aliasing — dereferencing the `*mut T` from `UnsafeCell::get()` requires no
concurrent writer. Under `&self` alone this holds iff the type is `!Sync`.
Evidence chain: sound in isolation (identical to `core::cell::Cell`'s reasoning) and
UNSOUND only because SITE 3 grants `Sync`. The invariant is established nowhere once
`Sync` is asserted.
Documentation: WRONG (structurally). The comment reads `// SAFETY (single-threaded):` on a
SAFE `fn`. Per the audit reference's documentation anti-patterns, a safe fn cannot carry a
precondition — if callers must guarantee single-threadedness, this is an `unsafe fn`. The
comment is honest about intent but the signature contradicts it.

BLAST RADIUS: UB-memory (torn/racy read)
Rating: HIGH
Multipliers: library.

SAFE ALTERNATIVE: NECESSARY given the design (interior mutability requires `UnsafeCell`).
The unsafe surface is already minimal — one deref.

TEST COVERAGE: PARTIAL — `single_threaded_get_set_works` covers the happy path only.
Gaps: no Miri; no concurrent execution.

VERDICT: UNSOUND (inherited from SITE 3; would be SOUND if the `Sync` impl were removed)

---

### SITE 2: sync_cell.rs:119 — UNSOUND (inherited, the racing operation)
Kind: `unsafe` block, in `pub fn set(&self, v: T)`
Code: `unsafe { *self.inner.get() = v; }`

INVARIANT: aliasing — the write needs EXCLUSIVE access; `&self` provides it only if `!Sync`.
Evidence chain: same as SITE 1. This is the operation the module BREAK races (:88-89).
Documentation: WRONG (structurally) — same safe-fn-with-precondition shape as SITE 1.

BLAST RADIUS: UB-memory (data race on a non-atomic write)
Rating: HIGH
Multipliers: library.

SAFE ALTERNATIVE: NECESSARY given the design.

TEST COVERAGE: PARTIAL — same as SITE 1.

VERDICT: UNSOUND (inherited from SITE 3)

## Recommendations

1. **SITE 4 — bound the `Send` impl.** `unsafe impl<T: Send> Send for SyncCell<T> {}`
   discharges it against the lock-free equivalent of the module's own discharged/false
   distinction, at zero cost to the probe. This is the only finding that is plausibly a
   genuine slip rather than a designed property.
2. **SITE 4 — add `assert_send::<SyncCell<*const u8>>()`** to tests/sync_cell_use.rs, or
   drop the recommendation above and document the raw-pointer case as a second intended
   hole. Either resolution is fine; silence is not.
3. **SITES 1-2 — state the safe-fn/precondition contradiction in the prose.** The module
   analyses the `unsafe impl` axiom at length but does not note that `get`/`set` are safe
   fns carrying documented preconditions. That is a second, structurally distinct
   soundness defect and it strengthens the seam's own argument.
4. **SITES 3-4 — record the no-Miri decision.** For an artifact whose UB is deliberate,
   "we do not run this under Miri because the race is the point" is a legitimate and
   useful sentence. Its absence is indistinguishable from an oversight.

## Tooling Gaps

No Miri, no sanitizers, no fuzzing configured anywhere in the repo (zero hits across
*.toml, *.yml, *.yaml). For this file that is defensible; for the crate's SOUND seams
(spin_mutex.rs, the SPSC ring) it is the single highest-value addition — Miri is what
distinguishes a discharged `unsafe impl` from one that merely looks discharged, which is
the exact distinction this module's correction turns on.
