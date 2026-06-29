# Seam spec — type-level lock ordering (re-import rule under attack)

This is a **design proposal** for a fifteenth *seam*, and the **first in a third soundness domain —
liveness / progress**. Seams 1–12 live in memory safety (the basis is closed there, nine axes); seams
13–14 (`SEAM_SPEC_secret.md`, `SEAM_SPEC_labeled.md`) probed information flow and **both broke** [horn 1,
IFC sense], establishing the deepest result so far: *a closing seam exists only when the **floor** carries
the **domain's** precondition*, and Rust's affine floor carries none of IFC's. That was the **negative**
half of the floor-domain match. This seam tests the **positive** half: pick a *new* domain whose
precondition Rust's floor (or a thin typed discipline atop it) **does** carry, and ask whether a *closing*
seam reappears. If yes, the floor-domain match is confirmed from both sides, and the build-and-break method
is shown to map a *third* domain.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/ordered_locks.rs`).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL, **affinity**, **unique `&mut` borrows**), plus its
**trait resolver** (decidable for this well-founded relation) and **`PhantomData` rank markers**. The
underlying mutual-exclusion primitive (a `no_std` spinlock: `AtomicBool` + `UnsafeCell`) is **trusted** —
its mutual-exclusion correctness is foundation-discharged, exactly like SPSC's P1–P3, and is **not** under
attack. Do not attack the spinlock or the borrow checker; attack the **re-import rule** (the type-level
ordering discipline) and the §6 floor-domain-match claim.

## 1. The primitive

A **lock acquired under a global order**. The danger re-imported is **deadlock by lock-order inversion**:
two threads that take the same two locks in opposite orders enter a cyclic wait and never progress. This
is a **liveness/progress** hazard, *not* UB and *not* an information leak — a deadlocked program is
memory-safe and confidential. The classic discipline that forecloses it is a **lock hierarchy**: rank
every lock, require each thread to acquire ranks strictly decreasing. The "primitive" re-imported here is
**the act of acquiring a lock**, made order-checked at compile time.

## 2. The true precondition (cathedral side)

Lock acquisition is **deadlock-free *by lock ordering*** iff the set of locks held by any thread, at any
instant, forms a **strictly-decreasing chain in a single global rank order** — equivalently, the
hold-and-wait graph is acyclic because every edge points from a higher rank to a strictly lower one. (Note
the scope: this forecloses *lock-order-inversion* deadlock specifically. Deadlock also arises from
condition-variable misuse, channel/join cycles, and resource exhaustion — those are **out of scope**; see
§5. Whether enforcing the lock-order fragment may be *called* "deadlock-freedom" is itself a §6 question.)

## 3. The token — a type-level rank and an affine floor capability

Each `OrderedLock<T, L>` carries a **type-level rank** `L` (a sealed tower `Bottom` / `Up<L>`). A scoped
capability `Floor<F>` records *the lowest rank currently held* is `F`. You begin holding nothing at the
top via `Floor::root(|top| …)`. Acquiring is `lock.with(floor, |data, inner_floor| …)`, whose signature is
the entire discipline:

```text
fn with<F, R>(&self, floor: &mut Floor<F>, body: impl FnOnce(&mut T, &mut Floor<L>) -> R) -> R
where L: Below<F>
```

`Below<H>` is a sealed, **irreflexive** type-level strict order (`Bottom: Below<Up<H>>`; `Up<L>:
Below<Up<H>>` iff `L: Below<H>`; nothing is `Below<Bottom>`). Two facts make the held set a
strictly-decreasing chain *by construction*: (1) `L: Below<F>` forbids acquiring a rank ≥ the current
floor (no inversion, no same-rank co-hold); (2) `floor` is taken **`&mut`** for the closure's duration, so
the outer floor is unusable inside the body and the only capability available is the fresh `&mut Floor<L>`
at the lower rank — nested acquisitions march strictly downward, release is stack-ordered.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** An `OrderedLock` / `Floor` / `with` interface vends rank-tagged locks and a scoped
> acquire. The safe fragment may take multiple locks with no `unsafe` and no hand-written lock-order
> reasoning, because the **only** way to acquire a lock is `with`, which **requires a proof `L: Below<F>`**
> that this lock outranks-downward everything held — so a lock-order inversion is *unconstructible*. The
> hold-and-wait graph is acyclic by typing, hence no lock-order-inversion deadlock.

Structural stoppers: the rank tower is **sealed** (CF3 — no forged rank); `Below` is **irreflexive** (CF2
— equal ranks not co-holdable); the `&mut` floor borrow downgrades the held rank (CF1 — no higher-then-
lower acquisition). CF1/CF2 surface as an **E0308 floor type mismatch** (the `Below` bound pins `F`'s
constructor from `L`, so inference reports the floor conflict), CF3 as **E0277** (sealed supertrait).

## 5. Threat model

In scope: any path by which **safe** consumer code acquires two or more locks in an order that is *not* a
strictly-decreasing rank chain — a forged or out-of-tower rank, two same-rank locks co-held, a higher rank
acquired while holding a lower one, a `Floor` token duplicated or escaped to re-enter at a stale (higher)
rank, a way to obtain a `Floor<F>` other than `root`/`with`. Out of scope: **deadlock from non-lock-order
causes** (condition variables, channel cycles, thread-join cycles, resource starvation); the *correctness
of the rank assignment* (that the integers the author hands the locks reflect a sound real-world order is a
human judgment — an MMIO-shaped TCB axiom the type enforces *consistently* but does not *verify*);
mutual-exclusion correctness of the spinlock (trusted foundation); and progress *liveness* of the spinlock
itself (it may spin, but it cannot deadlock by ordering — the property under test).

## 6. The scoring rubric — soundness, then the FLOOR-DOMAIN-MATCH deliverable

First, soundness *in the liveness sense* (no lock-order-inversion deadlock typable):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that holds two locks in a non-decreasing rank
  order — a higher-then-lower acquisition, a same-rank co-hold, a forged rank, or a `Floor` obtained/reused
  to re-enter at a higher rank than currently held. Or concede the `Below`-gated, `&mut`-downgraded `with`
  forecloses it.
- **Horn 2 — decidable & proof-free.** `with`/`root`/`new` are ordinary safe calls; `L: Below<F>` is
  discharged by trait resolution (well-founded, depth = rank, ranks bounded). Confirm — *and note whether
  this re-introduces `typed_nat`'s checker-tractability cost (resolution depth) or stays trivially bounded.*
- **Horn 3 — concentration.** The lock-order obligation is concentrated in the rank types + the `Floor`
  threading; the consumer writes no lock-order proof and no `unsafe`. Confirm — *or* argue the `Floor`
  threading is itself a per-operation seam crossing (a seam-width cost).

Then **the floor-domain-match deliverable** — the point of the phase. Rule on each:

1. **Does this seam CLOSE** (all three horns hold) for the lock-order-inversion fragment, in a domain
   (liveness) where memory-safety reasoning is silent? The claim: yes — Rust's *type-level order* floor
   carries the domain's precondition (an acyclic global order **is** a strict partial order), so the
   inversion is unconstructible. Agree, or find the deadlockable safe program.
2. **Is "deadlock-freedom" the right name, or does the seam close only a SUB-PROPERTY?** The honest claim:
   it closes *lock-order-inversion* deadlock-freedom, **not** deadlock-freedom in general (condvars,
   channels, join cycles remain). Is this the **validity-seam pattern** (a real guarantee that names a
   *sub*-property and must not overclaim), and does the consumer risk reading "deadlock-free" too broadly?
3. **What does it COST, and is that a clean NEW (liveness-domain) axis, a PROJECTION onto an axis already
   on the memory-safety map, or a TRUSTED AXIOM?** Candidate costs: (a) the **rank-assignment axiom**
   (MMIO-shaped — human picks ranks; floor enforces consistency not correctness); (b) **API-locality** —
   the `Floor<F>` token threads through every signature that acquires locks (viral, like the branded
   `'id`); (c) a **static-order-only expressiveness limit** — no *data-dependent* acquisition order (e.g.
   hand-over-hand list traversal where lock identity is a runtime value), and no two same-rank locks
   co-held even when safe (cf. `typed_nat`'s static-index-only, ruled a *confounded* non-axis). Which is
   it? Does the genuine liveness coordinate (a *runtime* acyclicity, as `lockdep` checks dynamically)
   *escape the static floor* the way IFC's quantitative coordinate escaped the affine floor — making this
   a **partial** floor-match (carries the static fragment, not the dynamic one)?
4. **Does this confirm the floor-domain match from the POSITIVE side, and that the method transfers to a
   third domain?** The claim: memory safety closes on the *affine* floor; information flow *fails to close*
   on it; liveness/deadlock-freedom *closes* on a *type-level-order* floor — three domains, each matched
   (or not) to a different floor, the seam tension the same shape in each. Does the three-layer result
   (trilemma domain-general / cost-basis per-domain / closure gated by the floor-domain match) now stand
   on a *positive* instance, not only the IFC negative?

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which axis, and whether it is new / a projection /
a trusted axiom) / **SEAM-BREAKS** (give the deadlockable or mis-ordered safe sequence). And state,
explicitly: **does a closing seam reappear in the liveness domain on a type-level-order floor — confirming
the positive half of the floor-domain match — or does the genuine liveness coordinate escape the static
floor (a partial match), or does the seam overclaim "deadlock-freedom" for a sub-property, or is it
unsound?** Prior art to weigh: lock hierarchies / `lockdep` (runtime), Boyapati–Lee–Rinard "Ownership
Types for Safe Programming: Preventing Data Races *and Deadlocks*" (OOPSLA'02), session types &
deadlock-freedom (Honda et al.), the `tracing`/`parking_lot` ecosystems, typestate, and the project's own
`typed_nat` (static-vs-runtime) and `mmio` (TCB axiom) precedents.
