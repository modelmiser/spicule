# refined escalation — line-by-line re-examination + out-of-family adjudication

**Date:** 2026-08-12 · **Context:** Gemini 3.1 Pro's full-sample run escalated the recorded
SEAM-COSTS [horn2-decidability] on the refined-index seam to SEAM-BREAKS [horn1].
This record closes the residual flagged in FOREIGN_REPLICATION.md.

## Line-by-line finding (in-family, disclosed as such)

Gemini's `horn1_ub_path` requires a configuration the spec does not vend: an
unparameterized `Refined` AND no runtime check. The spec's two disclosed
configurations are (a) the bare-NLL model — runtime `bound == slice.len()` compare
inside `at` (SEAM_SPEC_refined.md §3), which rejects `refine(5,10)` against a
length-2 slice, and (b) the refinement floor — R2 is *definitional* via the shared
length variable `n` (§2), making the mismatched call ill-typed. Its own
`stayed_decidable: false` and headline are horn-2-shaped; the horn-1 label is a
classification of an unvended strawman.

## Out-of-family adjudication (GPT-5.6 Sol, blind, single ask-mode call)

Adjudicator was shown the spec excerpt and both rulings, not this analysis.
Transport: cursor-cli, same disclosed deltas as the replication runs. GPT-5.6 is
foreign to both the recorded (Claude) and challenger (Gemini) families, though it
produced the other full-sample replication. Prompt md5 `577d84524c2552f9ead8ee095c6d288d`,
reply md5 `337ba4ce654f6628de6b41f4e823fe0b`. Verbatim reply:

> 1. No. In the bare-NLL model, `at` detects `10 != 2` and panics before `get_unchecked`. On the refinement floor, `r` is tied to length `10`, while the slice has length `2`; the call cannot establish the shared `n` and is rejected. B instead assumes an unchecked, existentially erased configuration the spec does not vend.
> 
> 2. B’s conditional design-space claim is correct: static erasure requires either a runtime equality check or a static type-level relation. The floor supplies the latter through shared `n`, so there is no contradiction. This concedes that some type-level relation is necessary, but `n` is not a brand as defined here: it denotes structural length, not generative slice identity. It permits same-length cross-slice use and does not impose lifetime branding, closure scoping, or non-storability. It may still add type-signature propagation, but that is distinct from identity-brand virality.
> 
> 3. Final ruling (≤150 words):
> 
> **SEAM-COSTS [horn2].** Ruling B supplies no UB path through either vended configuration. The bare-NLL implementation checks R2 dynamically and panics on the proposed mismatch; the refinement floor represents R2 statically through the shared length variable `n`, making the mismatched call ill-typed. The design therefore soundly re-imports unchecked indexing under the stated threat model. Its actual cost is the trusted external SMT solver and resulting one-toolchain dependency. Strictly, consumer proof-freedom remains intact—the consumer writes no proof—so horn 2 is better understood as unbundled into decidability/proof automation, which the design satisfies, and solver-freedom, which it sacrifices.

## Disposition

**Recorded SEAM-COSTS [horn2] SUSTAINED.** Durable contributions from the
escalation, kept: (1) the explicit dichotomy — static erasure of a relational
bound requires a runtime equality check XOR a static type-level relation; (2) the
axis-#5 clarification it forced — a shared structural length variable is not a
brand (it denotes length, not generative slice identity; permits same-length
cross-slice use; imposes no lifetime/closure/storability constraint), though it
does add type-signature propagation, a distinct cost; (3) the adjudicator's
unbundling of horn 2 into proof-automation (satisfied) vs solver-freedom
(sacrificed) — which is precisely the recorded verdict's [solver-freedom] label.
