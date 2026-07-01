# Seam spec — the correctly-bounded spinlock (the salvage of #36) (re-import rule under attack)

This is a **design proposal** for a thirty-seventh *seam*, the **salvage of `spin_mutex` #36** — mirroring #26→#27,
#29→#30, #31→#32, #33→#34. #36 was itself the salvage of #35 (`sync_cell`), but it **broke** (r43, SEAM-BREAKS
[horn1], tally 9→10): it discharged the LOCK obligation (the CAS witnesses one live guard) but **omitted the GUARD's**
— with no manual guard impl, `SpinGuard` auto-derived `Sync ⟺ SpinMutex<T>: Sync ⟺ T: Send` (too weak), so sharing
one `&guard` across `thread::scope` over `T = Cell<i32>` (`Send + !Sync`) raced the interior cell — UB from safe code.
**This seam adds the one missing bound** and stakes that the re-import is now sound in full. The thesis under attack:
a mutex has exactly **four** auto-trait obligations, #36 discharged three, this discharges the fourth, and the result
is the sound, value-adding, `core`-only concurrency seam #35 claimed could not exist — settling the #35→#36→#37 chain
and confirming data-race freedom is a **concentrate-behind-a-seam** domain.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/spin_lock.rs`, default
`no_std` build — `core` only; the seam concentrates `unsafe` in its three `unsafe impl`s + the two guard `Deref`s, the
consumer uses none). Tests pass (default + `--features lockdep`), including a `compile_fail` doctest (CF1: the r43
break is now rejected — a guard over a `Send + !Sync` `T` is not `Sync`) and a `no_run` doctest (the sound witness:
`thread::scope` mutating a shared `&SpinLock` race-free).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; the `Send`/`Sync` **auto-trait** system (`unsafe impl` is the manual escape; an
explicit `unsafe impl` of an auto trait **opts the type out** of the auto-derivation for that trait); `UnsafeCell`;
`core::sync::atomic::*` (`AtomicBool` + `Acquire`/`Release`/`Relaxed`), with the C11-style memory model stipulated
sound (RustBelt); `core::hint::spin_loop`; `Drop`; `Deref`/`DerefMut`. **`core` vends atomics but NO mutex.** Do not
attack the atomics, the memory model, `UnsafeCell` semantics, or the auto-trait system. Attack the **`SpinLock`
re-import rule** — specifically whether the **four-obligation enumeration is complete and correctly discharged** — and
the §6 deliverable.

## 1. The primitive

**A mutual-exclusion lock over shared mutable state, with a correctly-bounded guard.** `SpinLock<T>` wraps
`UnsafeCell<T>` beside an `AtomicBool`. `lock(&self) -> SpinLockGuard<'_, T>` spins on `compare_exchange_weak(false,
true, Acquire, Relaxed)`; the guard vends `&T`/`&mut T` via `Deref`/`DerefMut` and releases (`Release`) on `Drop`.
`try_lock` is the non-spinning variant. The `unsafe` surface is **three** `unsafe impl`s:
`unsafe impl<T: Send> Sync for SpinLock<T>`, `unsafe impl<T: Send> Send for SpinLock<T>`, and — the r43 fix —
`unsafe impl<T: Sync> Sync for SpinLockGuard<'_, T>`. Safe consumer code composes `new`/`lock`/`*guard` with **no
`unsafe`**.

## 2. The true precondition (cathedral side) — the COMPLETE four-obligation set

Data-race freedom requires that no two threads concurrently access the non-atomic storage with ≥1 write. Discharging
it for a shared `&SpinLock` requires **all four** of the mutex's auto-trait obligations, enumerated in full (the
discipline #36 lacked — it enumerated only the first three):

1. **`SpinLock<T>: Send where T: Send`** — moving the whole lock moves the `T`; sound iff `T: Send`. (auto)
2. **`SpinLock<T>: Sync where T: Send`** — a shared `&SpinLock` lets threads take turns; `lock()` gives *exclusive*
   access, so the `T` only **migrates** between threads (never shared `&T`), hence `T: Send`, not `T: Sync` (matches
   `std::Mutex<T>: Sync where T: Send`).
3. **`SpinLockGuard<T>: Sync where T: Sync`** — a *shared* `&guard` derefs to `&T` reachable from many threads, sound
   iff `T: Sync`. **This is #36's omitted bound.** The explicit `unsafe impl<T: Sync> Sync for SpinLockGuard` opts the
   guard out of the too-weak auto-`Sync` (`T: Send`, inherited from the `&SpinLock<T>` field) and pins it to `T: Sync`.
4. **`SpinLockGuard<T>: Send where T: Send`** — moving a guard *migrates* exclusive access; sound for a spinlock
   (the `Release` store is valid from any thread), so left **auto-derived** (`= T: Send`). Deliberately **not
   `!Send`**: `std::MutexGuard` is `!Send` because an OS futex must be unlocked on the acquiring thread, but that is
   an OS-specific constraint a pure spinlock does not have — `spin::MutexGuard` is `Send`. This is a *reasoned* choice,
   not an omission.

The discharge: the `AtomicBool` CAS is a runtime witness that at most one guard is live (obligation 3's exclusion
across the *lock*), and the `T: Sync` bound handles the guard's *sharing*; together they meet the data-race-freedom
obligation in full.

## 3. The witness — the r43 break is now CLOSED

CF1: `assert_sync::<SpinLockGuard<Cell<i32>>>()` does **not** compile — `Cell<i32>: !Sync` ⇒ (by obligation 3's
explicit bound) `SpinLockGuard<Cell<i32>>: !Sync`, so `&guard` is not `Send` and the r43 `thread::scope` witness fails
to type-check. The SOUND witness (`no_run`): `assert_sync::<SpinLockGuard<i32>>()` compiles, and two threads mutating
a shared `&SpinLock<i32>` through `lock()` is data-race-free (mutual exclusion serializes the writes). A single-thread
integration test observes the invariant (`try_lock` returns `None` while a guard is live) and confirms the bounds
(guard `Sync` at `T: Sync`, `Send` at `T: Send`; lock `Sync`/`Send` at `T: Send`).

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `SpinLock` vends `new`/`lock`/`try_lock` and, via the three `unsafe impl`s, the capability to
> share `&SpinLock` across threads and to hand out guarded `&T`/`&mut T`. Safe code composes them with no `unsafe`.
> The type system + the lock invariant are claimed to guarantee **data-race freedom** — every `&`/`&mut T` a guard
> vends is exclusive (mutual exclusion), and a `&guard` is shareable across threads only when `T: Sync` — for **every**
> `T`, with the four-obligation set discharged in full.
>
> This claim is staked as **TRUE** (the salvage). The honest residue (§5) is **liveness**, never soundness.

## 5. Threat model

In scope: any **safe** path that (a) reaches a **data race / UB** — the claim is there is none; the four obligations
are complete and discharged; (b) exhibits a **FIFTH omitted obligation** — the sharpest attack, given #36 broke on an
omitted conjunct: is there any auto-trait / variance / `Drop` / re-entrancy-into-`unsafe` obligation the four-item
enumeration misses? (e.g. a `T: ?Sized` interaction, a covariance/`PhantomData` subtlety, a guard-vs-lock lifetime
escape, an ordering gap where `Relaxed` should be `Acquire`/`Release`); (c) argues the guard's **`Send` (obligation
4)** is itself unsound for a spinlock (vs. `std`'s `!Send`) — the claim is it is sound (the atomic store is
thread-agnostic; `spin::MutexGuard` precedent); (d) argues the seam is **vacuous / a textbook restatement** (granted
— see §6.4). **Out of scope / conceded up front (the honest residue, NOT soundness):** a `mem::forget`-ten guard
never releases ⇒ permanent **deadlock**; a same-thread re-`lock` **self-deadlocks**; in **IRQ/interrupt context** a
lock held across a re-entering interrupt self-deadlocks (needs an `irqsave` variant); under contention, **unbounded
priority inversion**; **no poisoning** (a panic mid-section releases via `Drop`-on-unwind — no deadlock — but leaves
no poison flag on a possibly-broken invariant, unlike `std::Mutex`); **busy-wait** (burns CPU). All are
liveness/performance (the #10/#17/#29 leak/completion coordinate on the lock face). Conservation of hardness: the
concurrency danger **relocates** from *data race* to *deadlock/inversion*; §5 names the relocation rather than hiding
it. Out of scope: the atomics/memory model/`UnsafeCell` semantics (stipulated sound); that a data race is UB.

## 6. The scoring rubric — soundness, then the domain deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** Does safe code + the three `unsafe impl`s reach a data race (UB)? The claim: **no —
  it HOLDS.** All four obligations are discharged; the r43 break is closed (CF1). Confirm — or exhibit a **fifth**
  omitted obligation / laundering path (the pre-registered sharpest attack).
- **Horn 2 — decidable & proof-free.** The consumer writes plain `lock`/`*guard` — no SMT, no user proof. Trait
  resolution (`T: Send`/`T: Sync`) is the floor's own decidable auto-trait. Confirm decidable & zero-ceremony.
- **Horn 3 — concentration.** The `unsafe` is concentrated in the three `unsafe impl`s + the two guard `Deref`s; the
  consumer touches none; seam width is **O(1) per critical section**. The honest cost lands here as a **liveness
  residue** (§5). Rule whether concentration HOLDS and the residue is correctly a horn-3/liveness cost, not a break.

Then **the domain deliverable** — the point of the phase. Rule on each:

1. **Does the seam HOLD [horn 1] — is the four-obligation discharge genuinely complete (no data race, no fifth
   omitted conjunct)?** Confirm (or refute). If sound, the broken-seam tally **STAYS 10** (a HOLD), the **first
   concurrency-domain close**, and the r43 break is settled.
2. **Does `SpinLock` settle the #35→#36→#37 chain — establishing DRF as a concentrate-behind-a-seam domain?** The
   claim: **yes** — a sound, value-adding, `core`-only concurrency seam EXISTS (what #35 said could not); DRF is
   closed by concentrating seams (locks), the **memory-aliasing bucket** — with the caveat (from r43) that the bucket
   spans **static** discharge (SPSC/slab, compiler-verified) and **dynamic** discharge (`SpinLock`/`RefCell`, runtime
   witness), and `SpinLock` is the dynamic end. Confirm — or argue the chain does not settle (a residual gap).
3. **Is the guard's obligation-4 `Send` (vs. `std`'s `!Send`) correctly reasoned, and is discharged-vs-false a real
   distinction?** The claim: **yes** — `Send` is sound for a spinlock; and a discharged `unsafe impl` (obligation met
   by lock invariant + `T: Sync`) is categorically distinct from #35's false one. Confirm — or refute.
4. **Prior-art deflation (pre-registered).** This is `spin::Mutex` / `parking_lot` / a textbook spinlock; the bound
   set is `spin`'s own; discharged-vs-false is **RustBelt's semantic type soundness** (Jung et al., POPL 2018) applied
   to `Mutex`. **No novel mechanism.** Confirm the contribution is the **classification** (DRF concentrate-behind-a-
   seam; the complete four-obligation enumeration; closing the #36 break) — **not** the lock. Or rule it **redundant**.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn) / **SEAM-BREAKS** (give the safe sequence — and if
so, name the omitted obligation, as r43 did). And state, explicitly: **does the four-obligation discharge HOLD [horn
1] (no data race, no fifth omitted conjunct, tally STAYS 10); does `SpinLock` settle the #35→#36→#37 chain
(establishing DRF as a concentrate-behind-a-seam domain, the memory-aliasing bucket's dynamic end); is obligation-4's
`Send` guard correctly reasoned for a spinlock; is the honest liveness residue correctly a horn-3/liveness cost rather
than a horn-1 break; and does the contribution reduce to the textbook spinlock, or is the *complete-enumeration +
classification* the genuine finding — with no new verdict mode (N = 9; the broken-seam tally STAYING 10 if this
HOLDS, the #35→#36→#37 salvage chain settling)?**
