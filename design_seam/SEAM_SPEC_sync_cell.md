# Seam spec — shared mutability across threads (the data-race / concurrency probe) (re-import rule under attack)

This is a **design proposal** for a thirty-fifth *seam*, the **fifth fourth-domain probe** — after `crdt` #21
(convergence → folded into application-correctness), `wcet` #22 (real-time → broke unsound), and `divisor` #33
(panic-freedom → folded into application-correctness). It probes **data-race freedom / concurrency** — the property
Rust's `Send`/`Sync` auto-traits enforce ("fearless concurrency"). The `secret` #13 panel named "concurrency
channels" as a distinct axis with no memory-safety analog; this asks whether data-race freedom is a genuine **fourth
soundness domain**, or memory-safety (a data race **is** UB) closed by a *different mechanism*. It is a deliberate
stress-test of the "3-domain map (memory / IFC / liveness) is complete" claim the three prior deflations left
standing.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/sync_cell.rs`, default
`no_std` build — `core` only; the seam concentrates `unsafe` in its `unsafe impl Sync`/`Send`, the consumer uses
none). Tests pass (default), including a `compile_fail` doctest (CF1: the floor rejects a shared `Cell`) and a
`no_run` doctest (the break: `thread::scope` accepts the racing `SyncCell` shares).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; the `Send`/`Sync` **auto-trait** system (auto-derived structurally; `Cell`/`Rc`
are `!Sync`/`!Send` by construction; `unsafe impl` is the manual escape); `UnsafeCell` as the interior-mutability
primitive; `core::sync::atomic::*` as the sound cross-thread mutable cells. Assume auto-`Send`/`Sync` is **sound**
(Rustbelt: RustBelt proves the auto-trait discipline data-race-free). Do not attack the auto-trait system, atomics,
or `UnsafeCell` semantics. Attack the **`SyncCell` re-import rule** and the §6 deliverable: *is the concentrated
`unsafe impl Sync`/`Send` a sound re-import or a break; and — the point — is data-race freedom a fourth **soundness
domain**, or memory-safety closed by the floor's auto-trait, admitting no value-adding seam?*

## 1. The primitive

**Shared mutable state across threads.** `SyncCell<T>` wraps `UnsafeCell<T>` with a **non-atomic `&self` get/set**
(`get(&self) -> T` for `T: Copy`; `set(&self, v: T)`), plus an **unconditional `unsafe impl<T> Sync for SyncCell<T>`
and `unsafe impl<T> Send`** — the foundation author's assertion that a `&SyncCell` is safe to share/send across
threads. Safe consumer code composes `new`/`get`/`set` with no `unsafe`.

## 2. The true precondition (cathedral side)

- **Data-race freedom (the property at stake):** no two threads concurrently access the cell's non-atomic storage
  with at least one write. This is exactly what `Sync` is supposed to certify — and it is **FALSE** for
  `SyncCell<T>` under the unconditional impl (non-atomic `set` racing across threads is UB).
- **The floor already closes this:** auto-derived `Send`/`Sync` structurally forbids sharing a `Cell`/`Rc` across
  threads (CF1). The **sound** cross-thread mutable cell is `AtomicI32` (or `Mutex`), which the floor vends
  directly. So the seam's `unsafe impl` adds nothing sound — it only *removes* the floor's protection.

## 3. The witness — an unchecked axiom vs. the floor's structural closure

CF1: `assert_sync::<Cell<i32>>()` does **not** compile — the floor rejects cross-thread sharing of a non-atomic
cell. The BREAK (`no_run`): `assert_sync::<SyncCell<i32>>()` **does** compile, and `std::thread::scope(|s| {
s.spawn(|| c.set(1)); s.spawn(|| c.set(2)); })` type-checks — two threads racing a non-atomic write through a shared
`&SyncCell`, a data race (UB) with **no consumer `unsafe`**. The `unsafe impl Sync` is an **unchecked TCB axiom**:
`rustc` does not (cannot, in general, without the auto-trait it is here overriding) verify it. The only sound
`SyncCell` bounds `T: Sync` and mutates through atomics — which is the floor's `AtomicI32`, i.e. a vacuous restating.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `SyncCell` vends `new`/`get`/`set` and, via `unsafe impl Sync`/`Send`, the capability to share
> `&SyncCell` across threads. Safe code composes them with no `unsafe`. The type system is claimed to guarantee:
> 1. **Data-race freedom** — a `&SyncCell` shared across threads never races.
>
> This claim is **FALSE** for non-atomic `T` (the module BREAK), which is the point: the guarantee rests on the
> `unsafe impl`, an unchecked axiom, not on any checked rule.

## 5. Threat model

In scope: any **safe** path that (a) **reaches a data race / UB** through the vended API + the `unsafe impl`
(the `thread::scope` break — no consumer `unsafe`); (b) argues the seam is a **strawman / trivially known** (the
`unsafe impl Send` footgun is textbook), i.e. the break adds nothing; (c) argues data-race freedom **is** a genuine
fourth soundness domain with its own basis (rather than memory-safety closed by the auto-trait); (d) argues the
seam is **vacuous** — that the only sound setting (`T: Sync` + atomics) merely restates the floor, so there is no
seam here at all. Out of scope: the auto-trait system, atomics, `UnsafeCell` semantics (stipulated sound); the fact
that a data race is UB (stipulated).

## 6. The scoring rubric — soundness, then the domain deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** Does safe code + the `unsafe impl` reach a data race (UB)? The claim: **yes, it
  BREAKS** — the `no_run` doctest shares `&SyncCell<i32>` across `thread::scope` and races a non-atomic write.
  Confirm the break (or show the impl is somehow sound, e.g. only for atomic `T`).
- **Horn 2 — decidable & proof-free.** N/A to a break, but note: the sound alternative (`T: Sync` + atomics) is the
  floor's own decidable auto-trait.
- **Horn 3 — concentration.** The `unsafe` is concentrated in the `unsafe impl` — but as an **unchecked axiom**, not
  a checked concentration (the MMIO/`secret` pattern).

Then **the domain deliverable** — the point of the phase. Rule on each:

1. **Does the seam BREAK [horn 1] — a data race reachable from safe code via the unchecked `unsafe impl`?** Confirm
   (or refute). If a genuine soundness break, the broken-seam tally rises 8→9 (the first concurrency-domain break).
2. **Is data-race freedom a FOURTH soundness domain, or memory-safety closed by the auto-trait?** The claim: it is
   **memory-safety (UB) closed by a *different mechanism*** (auto-derived `Send`/`Sync`, Rustbelt), NOT a fourth
   coordinate. So the 3-domain map (memory / IFC / liveness) + application-correctness basis survives this stress
   test. Confirm — or argue concurrency is a genuine fourth domain (and rule what its basis is).
3. **Is there any VALUE-ADDING seam for data-race freedom, or does the floor's auto-trait already close it?** The
   claim: **no value-adding seam** — the only sound interface (`T: Sync` + atomics) restates the floor, and the only
   non-trivial re-import (`unsafe impl`) is an unchecked axiom that breaks. Confirm the floor-closed classification
   — or exhibit a sound, non-vacuous concurrency seam the floor does not already give. (A *runtime-checked* thread
   affinity wrapper — `SendWrapper`/`fragile` — is the candidate, but it needs `std` thread-IDs (not this `no_std`
   floor) and demotes the data race to a **panic** — folding into the totality/`divisor` domain, not concurrency.)
4. **Prior-art deflation (pre-registered).** The Rustonomicon's `Send`/`Sync` chapter and the `unsafe impl` footgun;
   RustBelt (Jung et al., POPL 2018) proving the auto-trait discipline; `SendWrapper`/`fragile`; `loom`/`miri` for
   detecting the race. Confirm the contribution is the **domain classification** (data-race freedom is floor-closed,
   distinguishing it from IFC's floor-closes-nothing and memory-aliasing's floor-closes-via-borrows) — **not** the
   well-known break. Or rule it a **redundant** restatement of the textbook footgun with no domain-map contribution.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does the unconditional `unsafe impl Sync` BREAK [horn 1] (a data race reachable
from safe code, tally 8→9); is data-race freedom a fourth soundness domain or memory-safety closed by the floor's
auto-trait; is there any value-adding concurrency seam or does the floor already close it (leaving only a vacuous
restating or an unchecked-axiom break); and does the probe's contribution reduce to the textbook `unsafe impl`
footgun, or is the floor-closed *classification* the genuine finding — with no new verdict mode (N = 9; the
broken-seam tally rising 8→9 only if this is accepted as a genuine concurrency-domain soundness break)?**
