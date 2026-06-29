# Seam spec — runtime lock-order registry (re-import rule under attack)

This is a **design proposal** for a sixteenth *seam*, and the **second in the liveness/progress domain**.
The fifteenth (`SEAM_SPEC_ordered_locks.md`, a *static* type-level lock order) **broke** [horn 1, liveness
sense], and the break **graded** the floor-domain match: it failed on **granularity** (re-entrant
`Floor::root` mints a second per-token lineage, but the precondition is per-thread-global) and **coverage**
(the *dynamic* fragment — runtime-ordered / same-rank locks — escapes a *static* floor). This seam tests the
falsifiable question the graded thesis implies: **does *upgrading the floor* lift the match — partial →
full — and at what price?** Same domain, same danger (deadlock by lock-order inversion); a different,
stronger floor: a **runtime** registry.

You are attacking a **text specification** backed by real `rustc`-checked code
(`seam/src/lock_registry.rs`, built with `--features lockdep`).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker as shipped, **plus OS thread-local storage** (`std::thread_local!`) and `std` (the
seam links `std`; it is *not* `no_std` — that is itself a recorded cost, §6). The underlying
mutual-exclusion primitive (a spinlock: `AtomicBool` + `UnsafeCell`) is **trusted**, as in every prior
seam. Do not attack the spinlock, the borrow checker, or thread-local storage's correctness; attack the
**re-import rule** (the runtime ordering registry) and the §6 floor-upgrade claim.

## 1. The primitive

A **lock acquired under a runtime-checked global order**. The danger is identical to the fifteenth seam:
**deadlock by lock-order inversion** (a liveness hazard, not UB, not a leak). The discipline is the same
(a global rank order, acquired monotonically), but the floor is **runtime**: each lock carries a `u64`
rank, and a per-thread held-set is consulted on every acquire.

## 2. The true precondition (cathedral side)

Lock acquisition is **deadlock-free by lock ordering** iff, on every thread, the set of locks held at any
instant is totally ordered by a single global rank and each new acquisition's rank is **strictly greater
than every rank currently held** (acquire monotonically increasing). The hold-and-wait graph is then
acyclic. (Scope, as in #15: this forecloses *lock-order-inversion* deadlock; condvar / channel / join-cycle
deadlocks are out of scope, §5.)

## 3. The witness — a thread-local registry of runtime ranks

`OrderedMutex<T>` carries a runtime `rank: u64`. A `thread_local!` `HELD: RefCell<Vec<u64>>` records the
ranks the calling thread currently holds. `OrderedMutex::lock(&self)`:
1. reads `HELD`; if `self.rank <= max(HELD)`, **panics** — a detected lock-order violation (the
   `lockdep`/`RefCell` modality: trap the bug loudly on the path that executes it, instead of deadlocking);
2. otherwise acquires the spinlock, registers `self.rank` in `HELD`, and returns a `Guard`;
3. the `Guard` removes `self.rank` from `HELD` (by value — order-independent) and releases on drop.

**This is the floor upgrade.** *Thread-local* fixes the fifteenth seam's **granularity** break: there is
exactly one held-set per thread, *un-mintable twice*, so the nested-`root` inversion has **no analog**.
*Runtime ranks* fix the **coverage** break: two locks of the same kind take distinct ranks (`accounts[i]`
→ rank `i`), so **bank-transfer `min/max`** ordering and **hand-over-hand** coupling — inexpressible under
#15's static, irreflexive, same-rank tower — are now ordinary safe code.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** An `OrderedMutex` / `lock` interface vends rank-tagged locks. The safe fragment may take
> multiple locks, in any *runtime*-chosen order, with no `unsafe` and no hand-written lock-order reasoning,
> because every `lock` call **checks the runtime order against the thread-local held-set** and **traps a
> violation as a panic** before it can deadlock. A lock-order-inversion deadlock is therefore
> **converted into a loud, immediate failure** on any path that exercises it.

Structural stoppers: the held-set is `thread_local` (one per thread, not mintable twice — granularity); the
check runs on every acquire (coverage); `Guard::drop` removes by value (never corrupts the registry).

## 5. Threat model

In scope: any path by which **safe** consumer code holds two locks in a non-increasing rank order
*without the registry trapping it*; a way to corrupt, bypass, or desynchronize the `HELD` set (a leaked or
forgotten guard, a panic that leaves `HELD` inconsistent, a `mem::forget(guard)` that never removes its
rank, reentrancy through `Drop`); a way for the registry to give a *false* verdict (accept an inversion, or
reject a sound ordering). Out of scope: deadlock from non-lock-order causes (condvars, channels, joins);
the *correctness of the rank assignment* (a human/data judgment — MMIO-shaped); spinlock mutual exclusion;
and **whether an inversion on an un-executed path is caught** — by construction the registry is a *runtime*
check, so it can only trap orderings that *run* (this is the disclosed modality cost, §6, not a soundness
bug within the model).

## 6. The scoring rubric — soundness, then the FLOOR-UPGRADE deliverable

First, soundness *in the liveness sense* (no undetected lock-order-inversion deadlock):

- **Horn 1 — sound re-import.** Exhibit safe code that co-holds locks in non-increasing rank order *without*
  a panic, or that corrupts `HELD` so the check gives a false verdict (a forgotten guard via `mem::forget`
  leaving a stale rank; a panic-in-the-gap; a reentrancy hole). Or concede the per-acquire thread-local
  check traps every *executed* inversion.
- **Horn 2 — decidable & proof-free.** `lock`/`new` are ordinary safe calls; the check is an O(held)
  runtime comparison. Confirm — *and note this is a **runtime** decision, not a compile-time one* (the
  modality, below).
- **Horn 3 — concentration.** The order obligation is concentrated in the registry; the consumer writes no
  lock-order proof and no `unsafe`. Confirm — *or* argue the per-lock runtime `rank` argument is itself a
  per-call obligation the caller carries.

Then **the floor-upgrade deliverable** — the point of the phase. Rule on each:

1. **Does upgrading the floor LIFT the match?** The fifteenth seam (static floor) was a *partial* match,
   failing on granularity and coverage. The claim: the runtime registry **fixes both** — thread-local ⇒
   per-thread (granularity), runtime ranks ⇒ dynamic orderings expressible (coverage). Agree, or find a
   gap (e.g. is thread-local *really* un-defeatable? does the dynamic fragment *really* close, or does some
   sub-case still escape?).
2. **At what PRICE — and does the price COUPLE the graded match to the closed cost-basis?** The claim: the
   lift is **bought**, not free, with (a) **bare-metal #5** — needs OS thread-local, *cannot be `no_std`*
   (hence the feature gate); (b) **zero-cost #6** — a runtime check + registry update per acquire, never
   erased (the *slab's* place, in a new domain); (c) a **modality shift** — static *prevention* (#15: the
   inversion was a compile error) → runtime *detection* (#16: the inversion compiles and is trapped only on
   executed paths). Are (a)/(b) genuinely the memory-safety axes #5/#6 re-instanced in the liveness domain
   — i.e. is the graded floor-match **coupled** to the closed cost-basis (lifting a match spends basis
   axes)? Is (c) — *prevention vs detection* — a **genuinely new dimension** (a guarantee *modality*, not a
   cost magnitude), or does it reduce to something already on the map?
3. **Is the lifted match "FULL", or "FULL-BUT-DYNAMIC" (a weaker modality)?** The honest claim: detection is
   *path-coverage dependent* — an inversion on an un-run branch ships uncaught (the real `lockdep` caveat),
   so the seam does **not** carry the precondition *statically*; it carries it *dynamically and
   conditionally*. So is this a **full** floor-match (the slab is considered a closing seam, and it too is
   a runtime witness), or is "static prevention" vs "dynamic detection" a distinction that means the match
   is *full in a different modality*, never identical to memory-safety's static closure? Prior art:
   `lockdep` (runtime, coverage-limited), `parking_lot`/`tracing` lock ordering, witness/ghost-state
   runtime checks, and the project's own **slab** (runtime generation witness, pays #6) and **validity**
   (runtime tag check) — both ruled *closing* seams.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which axes — and whether the modality shift is a new
dimension) / **SEAM-BREAKS** (give the undetected-inversion or registry-corruption sequence). And state,
explicitly: **does upgrading the floor lift the liveness match from partial toward full — confirming the
graded floor-match is *coupled* to the closed cost-basis (you pay #5 + #6 + a prevention→detection
modality) — or does the dynamic-detection modality mean the match is never "full" in the static sense, or
is the runtime registry unsound (a corruptible held-set)?**
