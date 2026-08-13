# Foreign-model replication — the monoculture control (run report)

**Run date:** 2026-08-12 · **Foreign families:** `gpt-5.6-sol-high` (GPT-5.6 Sol, high
effort, OpenAI) and `gemini-3.1-pro` (Gemini 3.1 Pro, Google) · **Transport:** Cursor
CLI (`FOREIGN_PROVIDER=cursor-cli`, subscription auth — no raw API key; see
`orchestrator/foreign_replication.py` docstring for the disclosed transport deltas) ·
**Sample:** the 8-seam stratified sample, 7 foreign calls per seam (5 attackers ∥,
judge, horn-classifier), 56 calls per family, one gauntlet per seam. Raw transcripts
under `runs_foreign_gpt-5.6-sol-high/` and `runs_foreign_gemini-3.1-pro/` (untracked,
like `runs/`); machine-readable results in each dir's `agreement.json` + `run_meta.json`.

## Headline

Outcome-class agreement: **GPT-5.6 Sol 6/8, Gemini 3.1 Pro 5/8** — in both families,
both recorded HOLDS were demoted to COSTS, and Gemini escalated one recorded COSTS to
BREAKS. Within that: **both families reproduced all four recorded BREAKS exactly,
horn labels included (8/8 break-cells [horn1])** — the strongest cells in the table,
with the caveat that all four recorded breaks share the modal horn label. On the
full-sample runs no recorded HOLD was ruled BREAKS; an earlier single-seam run did
rule one BREAKS, and was later vindicated (see the provenance note and lineage).

| seam | recorded (Claude) | GPT-5.6 Sol | Gemini 3.1 Pro |
|---|---|---|---|
| spsc | SEAM-HOLDS [none] | COSTS [horn1] ✗ | COSTS [horn3] ✗ |
| spin_lock | SEAM-HOLDS [none] | COSTS [horn3] ✗ | COSTS [horn3] ✗ |
| slab | SEAM-COSTS [other] | COSTS [none] ✓ | COSTS [none] ✓ |
| refined | SEAM-COSTS [horn2] | COSTS [other] ✓ | **BREAKS [horn1] ✗** |
| intrusive | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |
| secret | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |
| sync_cell | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |
| ordered_locks | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |

## The divergences, read closely

**Both HOLDS demoted, in both families — mostly, but not entirely, boundary pricing.**
Three of the four demotion cells are horn-3: on spin_lock, GPT's own *judge* ruled
SURVIVED before its classifier emitted horn-3; Gemini emitted horn-3 on both HOLDS.
(The horn-3 rationales differ in substance — GPT spin_lock prices an implicit `Sized`
bound and misuse-resistance gaps, Gemini spsc prices allocation and global storage,
and only Gemini spin_lock actually cites proof concentration — so "rubric-boundary
pricing" is our gloss over heterogeneous cost objections, not the transcripts' shared
stated reason.) The fourth cell, GPT spsc, is horn-1 and spec-defect-driven (the P4
wording objection below). On the two full-sample runs, no verdict on either HOLDS
seam claimed a UB path (all four `horn1_ub_path` fields are empty) — **but the
earlier single-seam GPT run did claim one** (SEAM-BREAKS via covariance laundering),
and the lineage below vindicated that attack as real against the v1 text. So the
honest summary is: full-sample demotions were argued at the HOLDS/COSTS boundary,
while one foreign run — later vindicated in-family — claimed a genuine UB path
against the flagship HOLD. The transport deltas (prompt prepending, no temperature
pin) and single-draw sampling are unquantified contributors to all of this.

**GPT's stable spsc objection was a specification-wording defect** (both GPT runs):
`Send` + `Arc<Mutex<Producer<T>>>` permits serialized migration across thread
identities, so thread-affinity does not discharge **literal** P4 ("one producer thread
*ever*") — explicitly "does not launder UB." This was repaired and re-adjudicated; see
the lineage section.

**Gemini's escalation on refined (COSTS[horn2] → BREAKS[horn1])** is the one two-way
divergence: its judge ruled the seam's central premise — a relational bound that is
simultaneously *brand-free* and *statically erased* — contradictory across function
boundaries ("an unparameterized value existentially hides its bound... impossible
without a runtime check or a viral type parameter"). That is a realizability claim
about the seam's central mechanism, not a severity dial; the recorded verdict priced
the same defect region as a horn-2 cost. **Re-examined line-by-line and sustained
(2026-08-12, `REFINED_ESCALATION_ADJUDICATION.md`): the recorded COSTS [horn2]
stands.** The checkable core is the in-family textual analysis: Gemini's UB path
requires a configuration the spec does not vend (no-runtime-check AND no type-level
relation; the spec discloses one of the two in each configuration — verifiable
against SEAM_SPEC_refined §2–§3). A blind out-of-family call (GPT-5.6, single
ask-mode draw) corroborated: the mismatched call panics in the model and is
ill-typed on the floor. Two independence caveats on that corroboration, disclosed:
it is one draw, and it comes from the family whose own replication run had already
ruled COSTS on this very seam (table above) — so it carries corroborative, not
independent, weight; the disposition rests on the spec-text analysis. The
escalation's durable content is kept: the erasure dichotomy (runtime check or
type-level relation — at least one) and the axis-#5 clarification that a shared
structural length variable is not a generative brand. The earlier "foreign panels
never upgrade" reading remains corrected — the upgrade happened; it did not
survive re-examination.

**Horn-label accounting, symmetrically.** The break agreement is horn-exact (8/8
cells [horn1]) — though all four recorded breaks share the modal label, so four-way
identity is weaker evidence than it looks. The COSTS agreements are horn-INEXACT
(slab: recorded [other] vs foreign [none]/[none]; refined: recorded [horn2] vs GPT
[other]) — the 6/8 and 5/8 figures score outcome class only. If horn concordance
counts as agreement depth on the breaks, horn discordance must be reported on the
agreements; it is, here.

## Foreign-side verdict instability (provenance note)

A first single-seam GPT run (same day, before the full run) returned
**SEAM-BREAKS[horn1]** on spsc, headlined by a covariance/lifetime-laundering attack.
On the full-sample re-run that attack was ruled argued-past and did not recur; the P4
objection appeared in both runs. **The first run's spsc transcripts were overwritten
by the full run before being staged** — an operator error, disclosed; the run-1
verdict line and judge JSON survive verbatim in the session record only, so every
run-1-dependent claim in this report (including "the P4 objection recurred in both"
and the two-family independence of the covariance find) rests on that session record,
not on disk. At the time this looked like foreign tier drift. **The lineage work
below vindicated the run-1 attack as real** — a strong corrective to treating a
non-recurring finding as noise. Note it is also direct evidence on sampling: the one
foreign cell ever drawn twice flipped tiers (BREAKS → COSTS), so foreign single-draw
instability is unquantified and plausibly higher than the recorded Claude-side
1-of-3 base rate.

## The spsc lineage (v1 → v6): repairing what the panels surfaced

Post-replication, the flagship seam's spec was amended and re-entered the Claude-side
gauntlet as new spec files (frozen v1 untouched). Each step killed one named defect
and surfaced the next:

| step | change | gauntlet verdict | what it surfaced |
|---|---|---|---|
| v1 (frozen) | — | HOLDS (3-streak, recorded) | — |
| v2 | P4 restated as role exclusivity (the GPT wording objection) | COSTS [horn1] | **P4-v2 held under all five attacks**; new: `N` unbounded+erased (P3 vacuous at N=0), teardown clause missing |
| v3 | `N >= 2` const bound at split; P6 teardown | **BREAKS [horn1]** | **unspecified VARIANCE**: covariant handles launder a dangling reference — the run-1 GPT attack, independently rediscovered in-family |
| v4 | invariance pinned in `T` | **BREAKS [horn1]** | **dropck ownership**: v4's own `PhantomData<*mut T>` marker gives invariance but not drop-ownership; under `#[may_dangle]`, P6's glue drops a payload whose borrow died |
| v5 | plain `Drop` (no `#[may_dangle]`) + `PhantomData<T>` ownership marker | **SEAM-COSTS [horn2-decidability]** — panel 1 of three; panels 2–3 drew COSTS [horn1] and BREAKS [horn1], see the stability run below (a first attempt was stopped externally at the judge stage BEFORE any adjudication existed — five attacker reports, no judge, so no outcome selection was possible; the rerun reused the round slot, so the stopped attempt is attested by the session record) | "All five attackers concede no UB from safe code; v5's five-tuple closes variance and dropck." Surviving price: `where [(); N-2]:` is off-floor, fires post-monomorphization, and syntactic abstract-const unification makes generic-over-N consumers restate the obligation per boundary. (Also: P6's stated invariant over-promises under `mem::forget`/cycles — a wording defect, ruled not-UB.) |
| v6 | input ELIMINATED (`channel<T,N>()`; RingBuffer unnameable; no-forgeable-input stopper) + stable-floor N-assert + P6 leak exclusion | **BREAKS [horn1] · COSTS [horn3] · BREAKS [horn1]** (three pre-registered panels) | panels 1+3 converge: §3's `Consumer<T>: Send` is UNBOUNDED while §4 bounds Producer only — latent since v1 (`SEAM_SPEC_spsc.md:71-72`); `Rc` payload → refcount race (RUSTSEC-2020-0150 class). Panel 2: no UB; horn-3 — no stable `where N >= 2`, so consumers re-paste the (stable, post-mono) const assert per boundary: repetition, not proof. |

**v5 three-panel stability run (2026-08-12 evening, user-ordered).** Two further
independent fresh panels (isolated scratch dirs; reports preserved as
`REPORT_SEAM_SPEC_spsc_v5_panel2.md` / `_panel3.md`). Distribution across all
three: **COSTS [horn2] · COSTS [horn1] · BREAKS [horn1]** — the tier is UNSTABLE
at one-panel resolution. But the instability has structure: panels 2 and 3 found
the SAME defect and differ only on an unwritten default. §3 says the seam "emits
exactly this, and nothing more" yet enumerates only the handles — never the
`RingBuffer` input, whose §1 representation is a plain record. Panel 2 presumes
field privacy (no UB derivable → COSTS, with privacy silently load-bearing for
horn 1); panel 3 does not (`head=1/tail=0` with an uninitialized slot satisfies
P3, so correct atomics hand safe code a `Box<u8>` of never-written bytes →
BREAKS). Both name the identical decidable, unwritten repair: pin the input's
construction (private fields, or a `channel<T,N>()` constructor). Panel 2 also
falsifies "decidable const bound" as stated: `generic_const_exprs` is unshipped,
so the `N >= 2` obligation degrades to post-monomorphization errors. **Net: the
declaration-surface enumeration is demonstrably open — a FOURTH component
(input-type provenance) surfaced exactly as the bidirectional caveat allowed —
and the one-panel "sound-but-priced" characterization is withdrawn as a stable
description; the input-provenance repair was subsequently authored and run as v6
(next section).**

**v6 run (2026-08-12 late, lineage reopened by owner order).** The v6 respec
(`SEAM_SPEC_spsc_v6.md`) applied the v5 panels' shared repair in its strongest
form — the input is ELIMINATED (`channel<T,N>()` constructs the buffer inside the
seam; `RingBuffer` unnameable; a "no forgeable input" stopper added) — plus the
stable-floor N-bound restatement and the P6 leak exclusion. Protocol pre-registered
in the spec header before any panel ran — verifiable at commit granularity: the
spec with its protocol landed in `1d76363` and the panel reports in `4452299`,
the spec byte-identical across the two. Three independent fresh panels, the
distribution is the result, no v7 without another reopen. (No comparable
in-artifact pre-registration exists for the earlier v5-era stopping decision;
that rule lived only in the session record, and this report no longer cites
it.) **Distribution: BREAKS
[horn1] · COSTS [horn3] · BREAKS [horn1]** (reports:
`REPORT_SEAM_SPEC_spsc_v6_panel{1,2,3}.md`). Panels 1 and 3 CONVERGE on a fifth
surface defect, present verbatim since v1 (`SEAM_SPEC_spsc.md:71-72`): §3's normative
enumeration asserts `Producer<T>: Send`, `Consumer<T>: Send` **unbounded**, while
§4's stopper supplies `T: Send` for Producer only — safe code sends the consumer
holding `Rc` payloads and races the non-atomic refcount (panel 1 cites
RUSTSEC-2020-0150 as the bug class). Repair: one decidable bound — unwritten, per
the protocol. Panel 2 found no UB and priced the `N` obligation as horn-3
concentration: stable Rust has no `where N >= 2`, so generic-over-N consumers
**re-paste the const assert at each boundary — "repetition, not proof"** (the
`const { assert!(N >= 2) }` spelling itself is stable and fires
post-monomorphization, exactly as the spec disclosed; the dent is the
per-boundary repetition, not the assert's spellability). Cross-panel horn
instability on this same price, for the record: v5 panel 1 filed it
[horn2-decidability], v6 panel 2 [horn3-concentration], and PAPER §6 records
the API-locality reading — three filings for one cost. **Net: the surface enumeration has yielded five
distinct discoveries in six specs — the `N` bound + missing teardown (v2's
gauntlet), variance (v3), dropck ownership (v4), input provenance (v5), and the
§3/§4 Send-bound inconsistency (v6; a consistency defect in a component class
already named, not a new kind) — and TWELVE post-v1 panels have never rated the
flagship clean (in-family: the v2, v3, v4 gauntlets + three v5 panels + three
v6 panels = 9; foreign: the GPT single-seam run + the GPT and Gemini
full-sample spsc cells = 3). One partial exhaustion signal is on record: v6
panel 2 surfaced no new component. The flagship sits between COSTS and BREAKS
and the lineage is closed under its stop rule (owner-reopenable).**

**The v6 horn-1 verdicts, adjudicated under the refined vend standard.** The
refined escalation was demoted because its UB path required a configuration the
spec does not vend. A hostile reader will ask whether the v6 breaks survive the
same standard — a configuration in which §4's stopper is honored blocks the
`Rc` path too. They survive, and the distinction is worth stating: refined's
attack *assumed absent* a protection the spec affirmatively disclosed (the
runtime check / the shared length variable), whereas §3 *affirmatively asserts*
the unsound unbounded impl as the seam's normative emission ("emits exactly
this, and nothing more"), with §4's bound appearing only in a Producer-side
impl example — the spec vends the unbounded reading on its face. An internal
§3-vs-§4 inconsistency whose normative side is the unsound one is a vended
defect; the repair (one bound) is a change to the vended text, not a disclosed
protection the attacker ignored. We also name the scrutiny asymmetry plainly:
the one verdict that threatened a recorded COSTS (refined) received a
line-by-line re-exam plus out-of-family corroboration, while the v6 breaks —
which support this report's closing reading — received only this paragraph;
the asymmetry is listed in the residuals.

**Two consequences.**

1. **The recorded v1 HOLD is contradicted as stated, and the lineage closed at
   the v6 distribution (BREAKS · COSTS · BREAKS), not back at HOLDS.** The
   covariance attack applies to the v1 text (same token section, variance never
   fixed), was found by two model families (GPT run 1 on v1 — per the disclosed
   session record — and the Claude gauntlet on v3, on disk), and no stopper in §4
   fires against it. Real implementations dodge it via `UnsafeCell`-forced
   invariance in the shared core. The v5 respec has now run THREE independent
   fresh panels (stability run above): **COSTS [horn2] · COSTS [horn1] · BREAKS
   [horn1]** — the tier is unstable at one-panel resolution, the panel-1
   "sound-but-priced" characterization is withdrawn as a stable description, and
   the bidirectional bound stated after panel 1 (a further surface component
   might exist, toward BREAKS) was realized by panel 3, with panels 2+3
   unanimously naming the missing component (input-type provenance). What IS
   stable across all panels and both families: **the recorded "HOLDS [none]"
   tier is not recovered — no panel of any family has rated the flagship clean
   since the surface enumeration began.** The defined "settled" bar (three
   consecutive survivals) is unreachable for v5 as written (FAILED verdicts
   occur); the operative description is the three-panel distribution itself; the
   input-provenance repair became v6, whose own three-panel distribution (above)
   surfaced the fifth component.
2. **The lineage's meta-finding, five distinct discoveries deep:** the seam token
   must pin the ENTIRE declaration surface — the **`N` bound and teardown** (v2's
   gauntlet), **variance** (v3), **dropck ownership** (v4), **input-type
   provenance** (v5), and **auto-trait bounds stated consistently between the
   enumeration and its stoppers** (v6 — a consistency defect in a component
   class the lineage had already named, not a new kind). Each is floor-checkable
   and decidable once named; each was invisible until an adversarial panel
   needed it. Five discoveries in six specs, the latest latent in the frozen v1
   text throughout, is strong *internal* evidence that any such enumeration
   should be presumed open — bounded by two confounds this lineage cannot
   separate: a panel disposition that returned zero clean ratings on anything
   in the lineage (a finds-something prior would produce this signature too),
   and attention reallocation (v6's input elimination made §3 the principal
   remaining attack surface, so the fifth discovery's location partly tracks
   where the panels were pushed, not only what the seam hides). n = one seam,
   one lineage.

## Disclosed deltas vs the recorded runs

1. No web tool on the foreign side (prior-art attacker reasons parametrically).
2. No temperature pin (HTTP providers pin 0.0; Cursor CLI exposes none).
3. Role system prompt prepended to the user prompt (Cursor CLI has no system channel).
4. One gauntlet per seam-per-family. The recorded Claude-side 1-of-3 tier-drift base
   rate is a lower-bound proxy only: the one repeated foreign cell (spsc, GPT) flipped
   tiers, so foreign single-draw instability is unquantified and plausibly higher.

## What this licenses (and what it doesn't)

The monoculture threat named in §6 has two components, and this control addresses
exactly one of them. **(a) Family-specific judge bias** (self-preference, one
family's idiosyncratic priors): sharply weakened — two foreign families reproducing
all four breaks with horn-exact attributions is strong evidence the breaks are not
*family* folklore. **(b) Corpus-shared consensus prior** ("a seamless dependent
systems language is impossible" as folklore common to every model trained on
overlapping corpora): **in principle unaddressable by any LLM-family replication,
including this one** — cross-family agreement is what a shared-corpus prior predicts
too, and the shared role prompts are additionally a shared instrument. Component (b)
remains open; only non-LLM evidence (the Lean skeleton, human adversaries, or a
mechanized counterexample) can bear on it. What the control also does *not*
establish: the flagship HOLDS at their recorded tiers. On this evidence the
HOLDS/COSTS boundary is rubric-relative in every family tested, and the flagship
HOLD itself carried a spec defect found by two families (one attested only by the
disclosed session record); the v5 respec drew COSTS [horn2] · COSTS [horn1] ·
BREAKS [horn1], and the v6 respec (input eliminated) drew BREAKS [horn1] ·
COSTS [horn3] · BREAKS [horn1] with panels 1+3 converging on a fifth surface
defect latent since v1 (§3's unbounded Send enumeration) — so on present
evidence the recorded HOLDS tier is not recovered, the flagship sits between
COSTS and BREAKS, and the lineage is CLOSED per its pre-registered protocol
(both three-panel records in the lineage section). Residuals: one gauntlet per seam per
family on the 8-seam sample; three transport deltas; the v5 tier distribution
itself; the refined re-examination's out-of-family corroboration being a
single draw from a family with a prior COSTS commitment on that seam — the
sustained disposition rests on the checkable spec-text analysis
(`REFINED_ESCALATION_ADJUDICATION.md`); and the scrutiny asymmetry named in the
v6 adjudication paragraph (re-exam effort was spent where a recorded verdict
was threatened, not where the closing reading was supported).
