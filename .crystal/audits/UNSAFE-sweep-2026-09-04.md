# Unsafe Audit Sweep: seam/src (16 files, 46 sites)
Date: 2026-09-04
Method: four parallel auditors, batched by hazard class; findings re-verified by the collator
        with independent witnesses before being recorded here.

> **RESOLUTION.** Only finding 1 (SPSC re-split) is FIXED, in the commit that adds this file.
> Findings 2-5 are left OPEN **deliberately**: each changes what this artifact CLAIMS, not just
> what its code does, and finding 2 falsifies a conclusion a cold panel already blessed. Those
> are research decisions for the owner. Findings 6-13 are hygiene, also open.

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

## 3. MEDIUM — `ordered_locks::with` strands its lock on panic. **OPEN.**

The release is a straight-line `self.flag.store(false, Release)` after `body(...)`, and the file
contains NO `Drop` impl (verified). If `body` unwinds, the store is skipped and every later
acquirer spins forever on a hot loop. `spin_lock`, `spin_mutex` and `lock_registry` all release
from `Drop` and survive unwinding; this module alone does not.

The SAFETY comment (:255-257) says "we hold `flag` exclusively until the Release store below" and
the doc (:241-242) promises release "when the body returns" — both assume a store the panic path
never reaches. Logic, not UB. Fix: an RAII guard, as the three siblings already use.

---

## 4. MEDIUM — `lock_registry::Guard::drop` can abort before releasing. **OPEN.**

`HELD.with(...)` (:208) runs BEFORE the release store (:214), and `LocalKey::with` panics if the
thread's TLS is already torn down — a guard held in a `thread_local!` hits exactly that. The
comment three lines above (:205-207) says the destructor will "never panic". The agent observed
`fatal runtime error: thread local panicked on drop, aborting`, exit 134.

Fix: release the flag first, then `try_with` for the bookkeeping.

---

## 5. MEDIUM — `shared.rs` refcount has no overflow guard. **OPEN.**

`Clone` does `strong.set(c + 1)` with no check. `std::rc::Rc` deliberately aborts here. `2^32`
iterations of the safe loop `mem::forget(s.clone())` wrap the count to zero on a 32-bit target,
after which the next `Drop` frees while owners are live. The crate builds clean for
`wasm32-unknown-unknown`. The module's "Honest scope" lists only `Weak`, cycles and `RefCell`.
Fix is safe code: `checked_add` + abort, as `Rc` does.

---

## 6-8. Test force: three discharges no test defends. **OPEN. Single-source.**

Reported via mutation testing; NOT independently re-verified by the collator (an attempted
re-run measured nothing because `cargo test -q` suppresses the lines the check counted).

- `Drain::drop`'s element loop (drain.rs:150-154) — the module calls it "load-bearing" (:121) —
  survives being replaced with an empty range. The one partial-drain test uses `u32` (no `Drop`)
  and asserts only `is_empty()`; the only drop-counting test `mem::forget`s the guard.
- `Slab::get_mut`'s occupancy guard (slab.rs:115) survives deletion, and that mutant makes
  `assume_init_mut()` read uninitialized storage from safe code. The sibling guard in `get` IS
  killed by a test — the coverage is asymmetric, and the `// SAFETY: as in get` comment hides
  exactly that asymmetry.
- `Buffer::drop`'s loop (drain.rs:112-116) never destroys an observable value in any test.

## 9-13. Documentation that does not match the code. **OPEN.**

- `lib.rs:804-806` asserts "We do NOT implement `Sync`" for the handles; `Sync` is an auto trait
  and both handles ARE `Sync` for every `T: Send`, via `unsafe impl Sync for RingBuffer` at :738.
  Harmless only because `push`/`pop` take `&mut self` — the real discharge, written nowhere.
- `validity.rs` — the witness is a hand-maintained `const VALID_DISCRIMINANTS`, not the "`match`
  pinned to the actual variants" the prose claims (:23), and "a new variant forces a matching
  update" (:65-66) is false: editing `C = 200` to `C = 201` compiles clean and `tag_at` hands out
  an invalid `Tag`. A `match` would need no `unsafe` at all.
- `take.rs` — the abort-bomb rests on double-panic-aborts, a `std::panicking` guarantee this
  `#![no_std]` crate does not link. Sound under `std` and under `panic = "abort"`; unproven in the
  `no_std` + unwinding-handler configuration the crate advertises. Also, `f(old)` — arbitrary
  consumer code — sits INSIDE the unsafe block.
- `intrusive.rs:13-14` says the unsafe is "concentrated in the foundation (two blocks)"; there are
  four sites. The two `unsafe impl`s are absent from that inventory, their only SAFETY text is
  false ("the bounds mirror std::cell semantics" — every `core::cell` type is `!Sync`; this one is
  `Sync`), and the real discharge (global read-XOR-write via token exclusivity, which does hold)
  is nowhere on the page.
- `typed_nat.rs:55-57` and `ordered_locks.rs:189-191` both claim the orphan rule stops a consumer
  from forging a trait impl. False as stated — a downstream crate can write `impl Lt<MyLocal> for
  S<Z> {}` (foreign `Self`, local parameter, RFC 2451). Both seams are still sound by a DIFFERENT
  argument (sealing forces both parameters foreign, and the all-foreign impl is then rejected by
  E0117) — the written argument is the wrong one.
- `selfref.rs` argues the self-pointer's ADDRESS stability but never its PROVENANCE. The tag is a
  shared reborrow of a transient `&mut Self`; `a.as_mut().init(); let _p = a.as_mut();
  a.as_ref().get();` invalidates it under Stacked Borrows. UNPROVEN rather than unsound — Rust has
  no ratified aliasing model, and the claim has never been machine-checked.

## Tooling gap

No Miri, no sanitizers, no fuzzing anywhere in the repo (zero hits across *.toml/*.yml/*.yaml),
and no `miri` component on any installed toolchain. For a crate whose entire subject is the
difference between a discharged and a merely-asserted `unsafe impl`, this is the highest-value
addition available: findings 1, 6-8 and the `selfref` provenance question are all exactly what
Miri checks. `[profile.test] opt-level = 0` additionally means the optimizer that exploits
`noalias` never runs over the concurrency seams.
