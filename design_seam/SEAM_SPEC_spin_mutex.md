# Seam spec — a core-only spinlock mutex (the sound concurrency seam; the salvage of #35) (re-import rule under attack)

This is a **design proposal** for a thirty-sixth *seam*, the **salvage of `sync_cell` #35** — mirroring #26→#27,
#29→#30, #31→#32, #33→#34. #35 (`sync_cell`) was a deliberately-unsound strawman — an **unconditional** `unsafe impl<T>
Sync` whose data-race-freedom obligation was simply **deleted** — built as the *vehicle* for a domain probe. It BROKE
(the broken-seam tally rose 8→9), but its **contribution FAILED**: I staked *"no value-adding concurrency seam exists;
data-race freedom is **floor-closed**; a re-import is **categorically** an unchecked axiom,"* and the r42 panel refuted
it with one obvious omitted seam — a sound, `core`-only **`SpinMutex<T>`** whose `unsafe impl<T: Send> Sync` is
**DISCHARGED by the lock's mutual-exclusion invariant**, not asserted false. **This seam builds exactly that.** The
thesis under attack is the *corrected* classification: data-race freedom (DRF) is a **concentrate-behind-a-seam**
domain (locks/channels/atomics are its seams — the concurrency analog of SPSC #1 / the slab #4), a **discharged**
`unsafe impl` is categorically distinct from a **false** one, and DRF is **not a fourth soundness domain** (a data
race *is* UB).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/spin_mutex.rs`, default
`no_std` build — `core` only; the seam concentrates `unsafe` in its `unsafe impl Sync`/`Send` + the two guard
`Deref`s, the consumer uses none). Tests pass (default + `--features lockdep`), including a `compile_fail` doctest
(CF1: the floor rejects a mutex over a non-`Send` payload) and a `no_run` doctest (the SOUND witness: `thread::scope`
runs two threads mutating a shared `&SpinMutex` race-free — the exact shape that was UB in #35).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; the `Send`/`Sync` **auto-trait** system (`unsafe impl` is the manual escape);
`UnsafeCell` as the interior-mutability primitive; `core::sync::atomic::*` (here `AtomicBool` + `Acquire`/`Release`/
`Relaxed`) as the sound cross-thread primitives, with the C11-style memory model stipulated sound (RustBelt).
`core::hint::spin_loop`, `Drop` glue, `Deref`/`DerefMut`. **Crucially: `core` vends atomics but NO mutex** —
`std::sync::Mutex` needs `std` (OS futex/parking). Do not attack the atomics, the memory model, `UnsafeCell`
semantics, or the auto-trait system. Attack the **`SpinMutex` re-import rule** and the §6 deliverable: *is the
concentrated `unsafe impl Sync`/`Send` a **discharged** (sound) re-import; and — the point — does it establish that
DRF is a concentrate-behind-a-seam domain, settling #35's "floor-closed / no value-adding seam" over-claim?*

## 1. The primitive

**A mutual-exclusion lock over shared mutable state.** `SpinMutex<T>` wraps `UnsafeCell<T>` beside an `AtomicBool`
flag. `lock(&self) -> SpinGuard<'_, T>` spins on `compare_exchange_weak(false, true, Acquire, Relaxed)` until it wins
the flag; the returned guard vends `&T`/`&mut T` via `Deref`/`DerefMut`, and its `Drop` stores `false` (`Release`).
`try_lock(&self) -> Option<SpinGuard>` is the non-spinning variant. The dangerous re-import is the pair
**`unsafe impl<T: Send> Sync for SpinMutex<T>`** and **`unsafe impl<T: Send> Send`** — the foundation author's
assertion that a `&SpinMutex` is safe to share/send across threads. Safe consumer code composes `new`/`lock`/`*guard`
with **no `unsafe`**.

## 2. The true precondition (cathedral side)

- **Data-race freedom (the property at stake):** no two threads concurrently access the cell's non-atomic storage
  with ≥1 write. This is exactly what `Sync` must certify — and for `SpinMutex` it is **TRUE**, discharged by the lock
  invariant: **at most one `SpinGuard` is live at any time** (the CAS admits exactly one winner until its `Drop`
  releases), so every `&`/`&mut` the guard vends is exclusive. The `Acquire`/`Release` pairing on the flag publishes a
  happens-before edge, so the serialized critical sections are properly ordered.
- **Why the bound is `T: Send` (not `T: Sync`), and why that is #35's missing conjunct:** the lock lets a `T` be
  touched from whichever thread holds it — the value effectively migrates across threads — so `Sync` requires `T:
  Send`. `SyncCell` #35's `unsafe impl<T> Sync` was **unconditional** (no bound) *and* undischarged; `SpinMutex`
  restores **both** the bound and the discharge. CF1 shows the floor enforces the bound (`SpinMutex<Rc<i32>>: !Sync`).
- **What the floor does NOT vend:** `core` has atomics but no mutex, and an `AtomicI32` gives a sound cell for *one
  machine word* only. `SpinMutex<T>` gives sound shared mutation for an **arbitrary `Send` `T`** (`[i64; 4]`, a
  struct, a `String` under `std`) — a genuine capability the floor's atomics cannot express. This is the value-add
  #35 claimed did not exist.

## 3. The witness — a DISCHARGED axiom vs. #35's FALSE one

CF1: `assert_sync::<SpinMutex<Rc<i32>>>()` does **not** compile — `Rc<i32>: !Send`, so the bounded impl does not
apply. The SOUND witness (`no_run`): `assert_sync::<SpinMutex<i32>>()` compiles, and `std::thread::scope(|s| {
s.spawn(|| *m.lock() += 1); s.spawn(|| *m.lock() += 1); })` type-checks **and is data-race-free** — the two non-atomic
writes are *serialized* by the flag. This is the identical shape that was UB for `SyncCell` #35; the difference is the
lock. The `unsafe impl Sync` here is a **discharged** obligation (the CAS witnesses exclusion), not an unchecked TCB
axiom. A single-threaded integration test observes the invariant directly: while a guard is live, `try_lock()`
returns `None`.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `SpinMutex` vends `new`/`lock`/`try_lock` and, via `unsafe impl<T: Send> Sync`/`Send`, the
> capability to share `&SpinMutex` across threads. Safe code composes them with no `unsafe`. The type system + the
> lock invariant are claimed to guarantee:
> 1. **Data-race freedom** — every `&`/`&mut T` a `SpinGuard` vends is exclusive (mutual exclusion), so a
>    `&SpinMutex` shared across threads never races.
>
> This claim is **TRUE** for every `T: Send` (the SOUND witness), because the guarantee rests on the **discharged**
> lock invariant, not on an unchecked axiom. The honest residue (§5) is **liveness**, never soundness.

## 5. Threat model

In scope: any **safe** path that (a) reaches a **data race / UB** through the vended API + the `unsafe impl` (the
soundness attack — the claim is there is none: exclusion is discharged, ordering is `Acquire`/`Release`); (b) argues
the discharge is **incomplete** — e.g. a memory-ordering gap (`Relaxed` where `Acquire`/`Release` is needed), a `T`
whose own interior mutability escapes the lock, or a re-entrant/`&mut`-aliasing path that mints two live guards; (c)
argues the seam is **vacuous / a strawman** — that `SpinMutex` merely restates the floor (it does not: `core` has no
mutex) or that a spinlock is textbook (granted — see §6.4); (d) argues DRF **is** a genuine fourth soundness domain
(rather than memory-safety — a data race is UB — closed by a *concentrating seam*). **Out of scope / conceded
up front (the honest residue, NOT soundness):** a `mem::forget`-ten guard never releases ⇒ permanent **deadlock**
(liveness — the #10/#17/#29 leak/completion coordinate on the lock face); no **poisoning** (a panic mid-critical-
section: `Drop` *does* run on unwind and releases, so no deadlock there, but no poison flag marks the possibly-broken
invariant — `std::Mutex` poisons, this does not); no **reentrancy** (a same-thread double-`lock` self-deadlocks);
**busy-wait** (a spinlock burns CPU under contention — a performance property). None of these is UB; all are liveness/
performance, and naming them is the point (conservation of hardness: the concurrency danger relocates from *data race*
to *deadlock*, and I make the relocation visible rather than hide it).

## 6. The scoring rubric — soundness, then the domain deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** Does safe code + the `unsafe impl` reach a data race (UB)? The claim: **no — it
  HOLDS.** The lock invariant discharges exclusion; `Acquire`/`Release` discharges ordering; `T: Send` is the correct
  bound. Confirm (or exhibit a laundering path — a memory-ordering hole, a double-guard, an escaping interior).
- **Horn 2 — decidable & proof-free.** The consumer writes plain `lock`/`*guard` — no SMT, no user proof, no external
  solver. Trait resolution (`T: Send`) is the floor's own decidable auto-trait. Confirm decidable & zero-ceremony.
- **Horn 3 — concentration.** The `unsafe` is concentrated in the `unsafe impl` + the two guard `Deref`s; the
  consumer touches none, and seam width is **O(1) per critical section** (one `lock()`/guard). The honest cost lands
  here as a **liveness residue** (§5), not a soundness break. Rule whether concentration HOLDS (predicted) and whether
  the liveness residue is correctly a horn-3 cost rather than a horn-1 break.

Then **the domain deliverable** — the point of the phase. Rule on each:

1. **Does the seam HOLD [horn 1] — is the discharged `unsafe impl` genuinely sound (no data race reachable from safe
   code)?** Confirm (or refute with a laundering path). If sound, the broken-seam tally **STAYS 9** (a HOLD, not a
   break), and this is the **first concurrency-domain close**.
2. **Does `SpinMutex` establish that DRF is a *concentrate-behind-a-seam* domain — settling #35's over-claim?** The
   claim: **yes** — a sound, value-adding, `core`-only concurrency seam exists (the thing #35 said could not);
   data-race freedom is **"floor-PROPAGATES-but-does-not-close"** (the auto-trait propagates `Send`/`Sync`; closure
   comes from concentrating seams like locks), the **same bucket as memory-aliasing**, NOT a floor-closed special
   case. Confirm — or argue DRF really is floor-closed (that `SpinMutex` adds nothing the floor's atomics don't).
3. **Is a *discharged* `unsafe impl` categorically distinct from a *false* one (SyncCell #35)?** The claim: **yes** —
   the distinction is *the obligation is met (a runtime lock witness) vs. deleted*. Confirm the distinction is real
   and load-bearing — or argue it is a distinction without a difference (both are `unsafe impl`s the compiler cannot
   check, so the seam is "just trust" either way).
4. **Prior-art deflation (pre-registered).** This is `spin::Mutex` / `parking_lot` / a textbook spinlock; the
   `Acquire`/`Release` discipline is the Rustonomicon's; `T: Send ⇒ Mutex<T>: Sync` is `std`'s own bound. **No novel
   mechanism.** Confirm the contribution is the **domain classification** (DRF is concentrate-behind-a-seam; discharged
   ≠ false; settling the #35 over-claim) — **not** the lock. Or rule it a **redundant** restatement with no domain-map
   contribution.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn) / **SEAM-BREAKS** (give the safe sequence). And
state, explicitly: **does the discharged `unsafe impl<T: Send> Sync` HOLD [horn 1] (no data race from safe code, tally
STAYS 9); does `SpinMutex` establish DRF as a concentrate-behind-a-seam domain (settling #35's "floor-closed / no
value-adding seam" over-claim); is a discharged `unsafe impl` categorically distinct from a false one; is the honest
liveness residue (forget→deadlock, no poison, no reentrancy, busy-wait) correctly a horn-3/liveness cost rather than a
horn-1 break; and does the contribution reduce to the textbook spinlock, or is the *classification* the genuine finding
— with no new verdict mode (N = 9; the broken-seam tally STAYING 9 if this HOLDS, the #35→#36 salvage pair settling as
the fifth)?**
