# Meta-Synthesis — are the incompatibilities one theorem?

*Over 6 recorded incompatibilities.*

## One theorem (universal property: Sound re-import (crossing back from a seam restores the safe fragment's invariants with no laundered UB))

> For any seam-vended capability whose soundness rests on a non-structural invariant (a separation-logic resource invariant with sharing, a runtime provenance/aliasing fact, or a memory-order/affinity/Send condition), sound re-import requires that invariant to be enforced at the use site, but the use site is ordinary code, which by Property 4 (decidable+proof-free) and Property 6 (zero-cost erasure) may carry only structural, inert artifacts that cannot represent or preserve such an invariant; therefore the invariant is either dropped (UB launders into safe code) or threaded explicitly (ordinary code crosses the seam, the ATS2 cliff), and sound re-import, seam concentration, and a decidable-proof-free-zero-cost safe fragment cannot hold together for exactly the non-separated foundations the seam exists to vend.

## How each follows

- **1: capability invariant has nowhere to live** — A resource invariant must be enforced at the point of use, but a proof-free decidable fragment enforces only linearity, so the invariant is dropped (token aliasing/laundering) or re-threaded (ordinary code crosses the seam).
- **2: reborrow/deref aliasing** — Whether a deref-chain reborrow creates a second live &mut is a semantic provenance fact invisible to syntactic/outlives rules, so soundness needs a per-crossing obligation that ordinary deref code generates pervasively — laundered if dropped, seam-crossing if discharged.
- **3: re-export of shared/affine foundations** — A structural affine check is sound only for separated owned sequential resources, so vending lock-free/thread-affine/DMA foundations either bridges the semantic gap unsoundly (UB) or carries the real per-structure proof (each composition crosses the seam).
- **4: SL-with-sharing interface undecidable to compose** — A re-importable interface must carry shared/persistent/relaxed resources, but matching them at a use site is frame inference (NP-to-undecidable), so the entailment confined to seam definitions reappears at every seam use, while the only decidable alternative — opaque tokens — cannot express the sharing the primitives need.
- **5: no relational vocabulary for cross-seam relations** — A decidable proof-free exterior cannot state inter-seam disjointness/aliasing/Send, so two individually sound seams on one resource race with no obligation able to fire, and enriching the exterior to state it reimports the SMT/undecidable obligations into ordinary code.
- **6: erasure deletes the gated effect** — Concentration fuses the effectful primitive to the witness that authorizes it into one object, so the relevance/erasure judgment that deletes the inert grade-0 witness also deletes the load-bearing effect — the runtime-regime shadow of the same use-site fusion.

---

## Full reasoning

## Verdict: ONE theorem. Six facets of it.

These are not six independent obstructions that happen to share a target. They are one theorem viewed through six mechanisms, and the giveaway is structural: **the same property is the load-bearing antagonist in every one of them — sound re-import.** Finding 1 says so outright ("Sound re-import is the property that mechanically gives"); 2's "per-crossing disjointness/provenance obligation," 3's "no unsound laundering," 4's "sound enough to re-import," 5's "sound composition," and 6's "restore invariants with no laundered UB" are all the *same* property under different names. When one property is the thing that breaks in all N proofs, you are not looking at N findings.

### The single structural reason: enforcement is local to the use site, but the use site is constitutionally inert

Every one of these capabilities — Held tokens, owned bytes, invariant-bearing atomics, reborrows, lock-free queues, MMIO words — is **sound only where its invariant is enforced at the point of use** (CSL soundness is local: a resource is safe exactly where its invariant holds at each access; drop enforcement at the use site and you get R0–R3, the data race, the use-after-free). 

But "the point of use" is *ordinary code*, and the design imposes two non-negotiable disciplines on ordinary code, each of which permits only **structural, inert artifacts**:

- **Property 4 (decidable + proof-free):** the safe fragment's vocabulary is affine/linear/points-to/structural — it can represent *separation* and *ownership*, but it **cannot represent** a resource invariant, a provenance fact, a memory-order condition, or a sharing fraction. These are semantic (Val×Heap→Prop with ghost state, frame inference, relaxed-memory order) and provably outside any decidable proof-free check.
- **Property 6 (zero-cost by erasure):** the authorizing witness is grade-0 and must vanish — it must be **inert at runtime**.

So the soundness-bearing invariant is forced to live in the one place that, by construction, only holds inert structural tokens. That is the whole theorem. The invariant has **nowhere to live**:

- **Leave it out** → the structural check passes while the semantic invariant is False → UB launders into safe code (existential laundering, double tokens, `spawn(try_push)‖spawn(try_pop)` typing affinely, two seams racing on one address, the erased witness that still fires a store). *Sound re-import fails.*
- **Thread it explicitly** so the use site can enforce it → ordinary code now manipulates the relational/separation obligation → ordinary code **crosses the seam at every deref-chain and every composition** → the ATS2 cliff → *concentration fails.*

The non-negotiable middle — a structural, inert artifact that nonetheless carries a live semantic invariant — does not exist, because the seam exists precisely to vend the **non-separated** foundations (shared, aliased, thread-affine, effectful), whose soundness is *definitionally* not structural. For separated, owned, sequential resources the theorem is vacuous (structure = soundness, and all properties hold — that's the safe fragment working as designed). It bites exactly on the foundations the seam was built to deliver.

### The minimal claim

> For any seam-vended capability whose soundness rests on a **non-structural** invariant (a separation-logic resource invariant with sharing, a runtime-provenance/aliasing fact, a memory-order/affinity/Send-closedness condition), you cannot simultaneously have (a) **sound re-import**, (b) **seam concentration** (ordinary code composes without crossing), and (c) a safe fragment that is **both decidable-proof-free and zero-cost-by-erasure**. Enforcement must be local to the use site; the use site is ordinary code; and ordinary code's two disciplines (c) admit only structural, inert artifacts that cannot carry the invariant — so the invariant is either dropped (launders UB, (a) fails) or threaded (crosses the seam, (b) fails).

Facets 1–5 are the **static** horn (the inert artifact can't be *represented* by a decidable proof-free checker). Facet 6 is the **runtime** horn of the identical fusion: when the witness and the effect it authorizes are one object — forced by concentrating effects at the seam — the relevance/erasure judgment that deletes the inert witness also deletes the load-bearing effect. Same object, same locality, two regimes (typecheck vs. runtime). That is why 6 reads differently (zero-cost, not decidability) yet is not independent: it is the erasure-side shadow of "the invariant must be live at the use site, but the use site only holds inert tokens."

### Why this is not a forced unification

A false unification would require ignoring a genuinely separate cause. There is none here: every facet's kill-shot reduces to *the soundness invariant cannot be enforced at the use site without either being invisible (unsound) or being handled by ordinary code (unconcentrated)*, and the property that mechanically yields is sound re-import in all six. The mechanisms differ (frame inference vs. provenance vs. relational vocabulary vs. erasure) because they are the different *ways* an inert structural fragment fails to hold a semantic invariant — not different reasons it must.

```json
{"one_theorem": true,
 "universal_property": "Sound re-import (crossing back from a seam restores the safe fragment's invariants with no laundered UB)",
 "statement": "For any seam-vended capability whose soundness rests on a non-structural invariant (a separation-logic resource invariant with sharing, a runtime provenance/aliasing fact, or a memory-order/affinity/Send condition), sound re-import requires that invariant to be enforced at the use site, but the use site is ordinary code, which by Property 4 (decidable+proof-free) and Property 6 (zero-cost erasure) may carry only structural, inert artifacts that cannot represent or preserve such an invariant; therefore the invariant is either dropped (UB launders into safe code) or threaded explicitly (ordinary code crosses the seam, the ATS2 cliff), and sound re-import, seam concentration, and a decidable-proof-free-zero-cost safe fragment cannot hold together for exactly the non-separated foundations the seam exists to vend.",
 "facets": [
   {"incompat": "1: capability invariant has nowhere to live", "how_it_follows": "A resource invariant must be enforced at the point of use, but a proof-free decidable fragment enforces only linearity, so the invariant is dropped (token aliasing/laundering) or re-threaded (ordinary code crosses the seam)."},
   {"incompat": "2: reborrow/deref aliasing", "how_it_follows": "Whether a deref-chain reborrow creates a second live &mut is a semantic provenance fact invisible to syntactic/outlives rules, so soundness needs a per-crossing obligation that ordinary deref code generates pervasively — laundered if dropped, seam-crossing if discharged."},
   {"incompat": "3: re-export of shared/affine foundations", "how_it_follows": "A structural affine check is sound only for separated owned sequential resources, so vending lock-free/thread-affine/DMA foundations either bridges the semantic gap unsoundly (UB) or carries the real per-structure proof (each composition crosses the seam)."},
   {"incompat": "4: SL-with-sharing interface undecidable to compose", "how_it_follows": "A re-importable interface must carry shared/persistent/relaxed resources, but matching them at a use site is frame inference (NP-to-undecidable), so the entailment confined to seam definitions reappears at every seam use, while the only decidable alternative — opaque tokens — cannot express the sharing the primitives need."},
   {"incompat": "5: no relational vocabulary for cross-seam relations", "how_it_follows": "A decidable proof-free exterior cannot state inter-seam disjointness/aliasing/Send, so two individually sound seams on one resource race with no obligation able to fire, and enriching the exterior to state it reimports the SMT/undecidable obligations into ordinary code."},
   {"incompat": "6: erasure deletes the gated effect", "how_it_follows": "Concentration fuses the effectful primitive to the witness that authorizes it into one object, so the relevance/erasure judgment that deletes the inert grade-0 witness also deletes the load-bearing effect — the runtime-regime shadow of the same use-site fusion."}
 ]}
```