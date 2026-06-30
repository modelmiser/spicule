# Seam spec — choice (offer/select, `&`/`⊕`) (re-import rule under attack)

This is a **design proposal** for a twenty-eighth *seam* — the last unprobed **core** session-type feature. It is
**NOT a fourth-domain bid**; it stays inside the **third confirmed domain (liveness)**. Every prior session seam
(#18, #23–#27) probed the *sequencing / progress* axis — order, delegation, n-party topology, priority,
projection. **Choice** is the orthogonal **branching** connective: external choice (`offer`, `&`) and internal
choice (`select`, `⊕`).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/choice.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, self-contained over #23's two-endpoint model). 158 tests pass
(default).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing + private fields; trait resolution (`HasDual`); **sum types +
exhaustive `match`** (the load-bearing capability here — an enum forces handling all variants); `Rc`/`RefCell`
with their runtime borrow checks; the leakpocalypse fact (`mem::forget` is safe, skips `Drop`, RFC 1066). The
transport's byte/label-moving is the trusted core (out of scope, like #23's). Do not attack `Rc`/`RefCell` or the
transport. Attack the **choice re-import rule** and the §6 deliverable.

## 1. The primitive

A **branching session**: over #23's `Rc<RefCell<Transport>>` dual endpoints, two markers extend `Tx`/`Rx`/`End`.
`Chan<Offer<L, R>>::offer() -> Result<Branch<L, R>, Self>` *receives* a choice (the `Branch<L, R>` enum carries
the correctly-typed continuation in each variant); its dual `Chan<Select<L, R>>` *sends* one via
`select_left() -> Chan<L>` / `select_right() -> Chan<R>`. `HasDual` threads choice: `Offer<L, R> ↔ Select<L::Dual,
R::Dual>`. The danger addressed is a **liveness** hazard (a starved/abandoned branch), not UB.

## 2. The true precondition (cathedral side)

- **Order:** `offer` only at an `Offer` state, `select_*` only at a `Select` state (affine typestate, as #18).
- **Handle every label:** the `offer`-er must handle *both* branches.
- **Label–continuation correlation:** the continuation taken matches the label (left ⇒ `L`, right ⇒ `R`).
- **Branch-set duality:** the selector's options are exactly the offerer's branches.
- **Completion (no abandonment):** every chosen branch runs to `End` — *as for every session seam since #18*.

## 3. The witness — sum types + exhaustive match + duality

`offer()` returns `Branch<L, R>`, an enum the consumer **must `match`** (Rust exhaustiveness ⇒ handle-all). Each
variant carries `Chan<L>` / `Chan<R>` (label–continuation correlation — taking `Left` and continuing as `R` is a
type error). `HasDual`'s `Offer<L, R> ↔ Select<L::Dual, R::Dual>` makes the option set the dualized branch set.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A branching session vends `offer`/`select_left`/`select_right`. Safe code composes them with no
> `unsafe`. The type system guarantees, **exactly**:
> 1. **Order** — an out-of-state op is a type error.
> 2. **Handle every label** — `offer` yields a `Branch<L, R>` enum; an exhaustive `match` must handle both.
> 3. **Label–continuation correlation** — each `Branch` variant carries the correctly-typed continuation.
> 4. **Branch-set duality** — `Offer<L, R>`'s peer is `Select<L::Dual, R::Dual>`; you cannot select an option
>    the peer does not offer, nor offer a branch the peer cannot select.
>
> The type system does **NOT** guarantee **completion**: a peer can `mem::forget` after selecting, or inside a
> branch, starving its dual — the same #17/#18/#23/#27 residue, **unchanged by branching**. Choice carries
> branch-*safety*, not branch-*completion*. Both failures are **liveness** (a starved/leaked session), not UB.

Structural stoppers (claimed): nominal/private typing forbids forging a `Chan` at a wrong state; the `Branch`
enum + exhaustiveness force handle-all; the variant types pin label↔continuation; `HasDual` pins the branch set.

## 5. Threat model

In scope: any **safe** path that (a) performs a choice op out of state and type-checks; (b) ignores a branch
(defeats exhaustiveness); (c) takes one label and continues as the other branch's protocol; (d) selects an
option the peer does not offer (defeats branch-set duality); (e) shows the re-import guarantees **completion**
(exhibit abandonment of a chosen branch — `tests/choice_use.rs` shows it). Out of scope: `Rc`/`RefCell`
internals; the trusted transport; async/threaded scheduling (single-threaded witness, as #23–#27); recursion
(no `Rec`/`Var` — a fixed-depth choice witness, as the arc).

## 6. The scoring rubric — soundness, then the no-new-coordinate deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the worst case is a **starved or leaked** (memory-safe) session,
  not UB. Confirm — or exhibit a real UB path (an out-of-state op that type-checks; a label taken with the wrong
  continuation; a branch-set mismatch the type system admits).
- **Horn 2 — decidable & proof-free.** `HasDual` over `Offer`/`Select` stays structural (Θ(depth), no SMT).
  Confirm — but note whether Horn 2 passes only because completion is never checked (the arc's pattern).
- **Horn 3 — concentration.** Order + branch-safety (handle-all, label–continuation, duality) are concentrated
  in the types (sum types + exhaustiveness). Confirm — or note the caller still carries the completion residue.

Then **the no-new-coordinate deliverable** — the point of the phase. Rule on each:

1. **Does choice add a NEW coordinate, or deflate onto the floor's sum types?** The hypothesis: no new axis —
   choice is labeled branching, and Rust's enums + exhaustive `match` carry handle-all + label–continuation,
   while `HasDual` threads the branch set. Confirm or refute (is there a branching-specific hazard the sum-type
   floor does not carry — e.g. a *mixed-choice* / nondeterminism gap, an exhaustiveness hole via wildcards?).
2. **Does completion stay exactly where it was (the #27 residue), unchanged by branching?** The claim: choice
   carries branch-*safety*, not branch-*completion*; a peer can abandon a chosen branch, starving its dual —
   the #17/#18/#23/#27 residue, untouched. Confirm — or argue branching introduces a *new* completion hazard
   (e.g. a branch that is *unreachable* yet must be handled, an asymmetry between the two branches' obligations).
3. **Does liveness stay the THIRD domain (N stays 9, FAILED tally stays 8)?** No new domain, coordinate, or
   mode; the seam HOLDS with the standard completion cost. Confirm — or argue choice is categorically new.
4. **Prior-art deflation (pre-registered).** External/internal choice is the founding session-type construct:
   **Honda 1993** (`&`/`⊕`); Honda–Vasconcelos–Kubo 1998; the `session_types` Rust crate (`offer!`/`choose!`
   macros over real channels). Confirm the contribution is **locating** choice on the lattice — the branching
   connective deflates onto the floor's sum types + exhaustiveness, adds no axis, and leaves the completion
   residue exactly where #27 left it.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does choice add a new coordinate or deflate onto the floor's sum types +
exhaustive `match`; does it carry branch-safety (handle-all, label–continuation, branch-set duality) but NOT
completion (the #17/#18/#23/#27 residue, unchanged by branching); does liveness stay the third domain with N = 9
and the FAILED tally at 8; and does choice deflate onto known folklore (Honda 1993's `&`/`⊕`; the `session_types`
crate), confirming the branching connective opens no new ground?**
