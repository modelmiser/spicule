# Dissociation-witness re-derivation against the amended SPSC row

2026-08-12 (late) · Owed by PAPER App B since the flagship's row changed; re-derived
from `REALIZABILITY.md`'s axis table (twelve memory-safety rows) under the v5
three-panel distribution and the v6 respec (input eliminated; panels pending).

## What each witness actually requires

**zero-cost ⟂ concentration via slab-vs-SPSC.** The dissociation claim needs only
two columns: #6 varies (slab ✗, SPSC ✓) while concentration is constant (both ✓).
Under the amended SPSC row both load-bearing cells SURVIVE every panel reading that
leaves the row defined: the `N >= 2` bound is compile-time (`const { assert! }` in
v6) with zero runtime residue, so **#6 stays ✓**, and the obligation is
once-per-buffer(-type), so **concentration stays ✓**. The cell the lineage changed
is the decidable/proof-free cell (post-monomorphization obligation for
generic-over-N consumers), which this witness never used. **Verdict: SURVIVES as
the two-column claim the table actually states** — with the caveat that the pair
was never a single-variable isolation anyway (the slab row carries two ✗ cells).

**unwind-safety ⟂ all-seven via take-vs-SPSC.** The strict "⟂ all-seven via one
pair" form used SPSC as an all-✓ baseline: unwind varies (take ◐, SPSC ✓) while
*all seven* other axes hold on both sides. The unwind cell itself is untouched by
the lineage (v3–v6 changed variance/dropck/N-bound/input cells, not panic-path
behavior) — but SPSC's decidable/proof-free cell is now ◐, so the pair varies in
two-plus columns and the single-pair universal form is **DEGRADED**. The weaker
pairwise coverage survives without SPSC: take-vs-drain witnesses unwind ⟂
leak-freedom (already in the table), take-vs-slab witnesses unwind ⟂ zero-cost.
**Verdict: unwind's independence stands pairwise; the one-pair "⟂ all-seven"
phrasing is retired.**

## Conditions

- Under panel 3's BREAKS reading of v5, the SPSC row is void and both witnesses
  are gated on the v6 panel distribution (v6 eliminates the forged-input path; its
  three pre-registered panels were launched before this note was written and their
  outcome does not change the analysis above except by voiding or restoring the
  row wholesale).
- Meta-finding, worth stating because it generalizes: **the all-holds row was
  load-bearing as a universal baseline for single-pair dissociations.** Without
  one, axis-independence must be argued pairwise-per-axis — which the remaining
  rows mostly support — and any future "⟂ all-N via one pair" claim should be
  read as contingent on its baseline row staying clean.

## Addendum (same day, post-v6 panels)

The gate resolved AGAINST a defined SPSC row: v6 drew BREAKS · COSTS · BREAKS
(fifth latent defect, §3's unbounded Send enumeration). Both SPSC-involving
witnesses therefore rest on substitutes now: **slab-vs-drain** carries
zero-cost ⟂ concentration (drain: #6 ✓, concentration ✓, pays only leak-freedom;
slab: #6 ✗, concentration ✓), and unwind independence stands pairwise
(take-vs-drain, take-vs-slab) as derived above. App B updated accordingly.
