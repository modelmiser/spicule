"""Role definitions for the adversarial design process (Section 5 of the seed).

Each role is (a) a system prompt that fixes its stance for the cold instance,
and (b) a builder that assembles the user message. ATTACKER builders receive the
proposal ONLY — no design history, no sibling attacks — enforcing coldness at
the prompt level in addition to the process level (claude_cli strips ambient
context). PROPOSER and JUDGE get exactly the context their job requires.
"""
from __future__ import annotations

import textwrap

# --- Program UNIFIED: the seed's original target (keep all 7, no seam) ---
UNIFIED_PROPERTIES = [
    "Executable specification (spec IS the type system IS the compiler)",
    "Unified proof/systems layer (no FFI seam between proof-land and systems-land)",
    "Ownership semantics without GC, derived from the proof layer (not a hardcoded borrow checker)",
    "Decidable type checking (common case needs no user-written proofs)",
    "Bare-metal targeting (no runtime, no RC spikes, no GC pauses; hard-real-time)",
    "Zero-cost abstractions (the proof layer erases at compile time)",
    "AI-legible semantics (a model trained on the corpus can reconstruct the spec)",
]
UNIFIED_CONSTRAINTS = [
    "Decidability: type checking must terminate; identify the decidable fragment.",
    "Zero runtime cost: proofs erase; no proof witnesses or unrequested allocations at runtime.",
    "Weak memory models: must express/verify ARM/POWER/RISC-V weak ordering.",
    "Ergonomics cliff: common case must cost no more annotation than Rust (ATS2 is the cautionary tale).",
]

# --- Program SEAMFUL: deliberately concede #2 (no-seam). A first-class, in-language,
#     machine-checked seam is REQUIRED and GOOD; the prior sweep proved keeping all 7
#     is impossible, so this program asks what the best seamful design is instead. ---
SEAMFUL_PROPERTIES = [
    "Executable specification — one toolchain, no external prover; spec IS type system IS compiler",
    "FIRST-CLASS MACHINE-CHECKED SEAM (deliberate concession of no-seam): proof-land and "
    "systems-land are ONE language separated by an explicit, in-language, machine-checked seam "
    "construct — NOT an FFI to a separate prover, NOT seamless. A seam is REQUIRED and good; do "
    "NOT penalize a proposal for HAVING a seam — only for the seam being informal, unsound, or pervasive",
    "Ownership without GC, derived from the proof layer — holds in the SAFE FRAGMENT",
    "Decidable AND proof-free in the SAFE FRAGMENT (ordinary code); user-written proofs and SMT "
    "appear ONLY at seam crossings, never in ordinary code",
    "Bare-metal targeting (no runtime, no GC)",
    "Zero-cost — all proofs, INCLUDING seam obligations, erase at compile time",
    "AI-legible semantics — the corpus reconstructs the spec including the seam discipline",
]
SEAMFUL_CONSTRAINTS = [
    "Seam CONCENTRATION (the make-or-break): seam crossings must concentrate in a small set of "
    "foundational primitives (allocators, atomics, lock-free structures, MMIO) re-exported as "
    "checked safe interfaces; ORDINARY code must compose them WITHOUT itself crossing the seam. A "
    "design where ordinary code constantly crosses the seam is the ATS2 cliff and FAILS.",
    "Decidable safe fragment: checking ordinary (non-seam) code terminates with no user proofs and no SMT/timeout.",
    "Sound re-import: crossing BACK from a seam must restore the safe fragment's invariants soundly — "
    "no unsound obligation may launder UB into safe code (the unsafe-boundary failure mode).",
    "Zero runtime cost: seam obligations/witnesses erase; nothing at runtime.",
]

# --- Program SEAMFUL2: concede #2 AND relax #3 (decontaminated seam test). The first
#     seamful run kept '#3 derived', forcing the proposer to re-derive a borrow checker
#     from scratch — a solved sub-problem (the Rust-NLL control settled it) that only
#     added formalization burden. Here the safe fragment MAY be a hardcoded Rust-style
#     checker, so the entire gauntlet bears on the SEAM itself. ---
SEAMFUL2_PROPERTIES = list(SEAMFUL_PROPERTIES)
SEAMFUL2_PROPERTIES[2] = (
    "Ownership without GC for the SAFE FRAGMENT — it MAY be a HARDCODED, decidable, "
    "Rust-style borrow checker (NLL as shipped is acceptable); it need NOT be derived "
    "from the proof layer. Do not re-derive or re-litigate the safe checker — adopt a "
    "known-good one; the design effort and the attacks belong on the SEAM (construct, "
    "sound re-import, concentration, erasure), not on the safe fragment."
)

# Active program (UNIFIED by default; run.py flips it via set_program).
TARGET_PROPERTIES = UNIFIED_PROPERTIES        # back-compat alias
HARD_CONSTRAINTS = UNIFIED_CONSTRAINTS
_PROGRAMS = {
    "unified": (UNIFIED_PROPERTIES, UNIFIED_CONSTRAINTS,
                "UNIFIED — keep all 7 properties, no seam (the seed's original ask)"),
    "seamful": (SEAMFUL_PROPERTIES, SEAMFUL_CONSTRAINTS,
                "SEAMFUL — a first-class machine-checked seam is REQUIRED and good; "
                "design the best seamful language, not a seamless one"),
    "seamful2": (SEAMFUL2_PROPERTIES, SEAMFUL_CONSTRAINTS,
                 "SEAMFUL2 — concede #2 AND #3: safe fragment may be a hardcoded "
                 "Rust-style checker; the entire gauntlet bears on the SEAM itself"),
}
_ACTIVE = {"name": "unified", "properties": UNIFIED_PROPERTIES,
           "constraints": UNIFIED_CONSTRAINTS, "banner": _PROGRAMS["unified"][2]}


def set_program(name: str) -> None:
    props, cons, banner = _PROGRAMS[name]
    _ACTIVE.update(name=name, properties=props, constraints=cons, banner=banner)

_ATTACKER_OUTPUT_CONTRACT = textwrap.dedent("""\
    After your analysis, end your response with EXACTLY one fenced json block:
    ```json
    {"verdict": "BREAKS|SURVIVES",
     "severity": "fatal|serious|minor|none",
     "summary": "<= 20 words capturing your finding",
     "strongest_point": "<one sentence: your single most damaging argument>"}
    ```
    'BREAKS' means you found a real defect the proposal must answer. 'SURVIVES'
    means you tried hard and the proposal holds against your angle. Do not flatter.
""")

ROLE_SYSTEM = {
    "proposer": textwrap.dedent("""\
        You are the PROPOSER in a rigorous adversarial language-design process.
        You produce ONE concrete, falsifiable, implementable design decision for a
        new systems language — not a direction, a specific choice. Where the
        decision is formal (a type rule, an operational-semantics step, an
        algorithm), state it formally: formation/introduction/elimination rules,
        judgement forms, or precise pseudocode. Vagueness is failure: an attacker
        must be able to point at a specific clause and break it.

        You will be told the exact design question, the language's target
        properties, the decisions already settled, and (if this is a revision) the
        attacks that defeated your previous attempt. Address those attacks head-on.

        Structure:
        1. THE DECISION — the concrete proposal, formal where possible.
        2. HOW IT SERVES THE TARGET — which target properties it advances and how.
        3. ANTICIPATED ATTACKS — the 2-3 strongest objections you expect, and your
           pre-emptive answer to each. Honesty here is rewarded; hiding a known
           weakness is not.
        Be precise and dense. No marketing language."""),

    "attacker-ergonomics": textwrap.dedent("""\
        You are ATTACKER-ERGONOMICS, a cold reviewer in an adversarial language
        design process. You see ONLY the proposal. Your single angle: will a
        working systems programmer actually use this? Take a REAL pattern — async
        I/O, arena/bump allocation, a lock-free MPSC queue, an intrusive linked
        list, a slab allocator, an interrupt handler — and write out what using it
        under this proposal concretely looks like. Find where annotation burden,
        proof obligations, or type ceremony make it unacceptable versus Rust. Your
        benchmark is the ATS2 failure: theoretically correct, practically unusable.
        A proposal that is sound but no one will tolerate writing has failed."""),

    "attacker-soundness": textwrap.dedent("""\
        You are ATTACKER-SOUNDNESS, a cold reviewer in an adversarial language
        design process. You see ONLY the proposal. Your single angle: formal
        correctness. Either (a) exhibit a concrete program the proposed rules
        ACCEPT that violates the invariant the proposal claims to enforce (a
        soundness hole — show the typing derivation and the bad execution), or
        (b) argue the type checking the proposal requires is undecidable or
        non-terminating, with the reduction. Be a hostile theorist. One real
        counterexample is worth more than ten vague worries."""),

    "attacker-implementation": textwrap.dedent("""\
        You are ATTACKER-IMPLEMENTATION, a cold reviewer in an adversarial language
        design process. You see ONLY the proposal. Your single angle: can this
        actually be COMPILED? Identify the specific compiler pass this proposal
        implies and show where it becomes undecidable, super-polynomial, or
        requires solving a known-open problem (e.g. higher-order unification,
        general SMT over the wrong theory, full alias analysis). Name the pass,
        name the blow-up, and give the input shape that triggers it."""),

    "attacker-expressiveness": textwrap.dedent("""\
        You are ATTACKER-EXPRESSIVENESS, a cold reviewer in an adversarial language
        design process. You see ONLY the proposal. Your single angle: find a
        program that SHOULD be valid under the language's stated philosophy but
        cannot be expressed under this proposal — or can be expressed only by
        escaping the system entirely. Probe the hard cases: unsafe hardware
        register access, FFI with a C library that aliases, a custom allocator
        handing out overlapping views, type-punning, a performance trick that
        deliberately defeats the abstraction. If the only way to write real code is
        to fall out of the type system constantly, the proposal has failed."""),

    "attacker-prior-art": textwrap.dedent("""\
        You are ATTACKER-PRIOR-ART, a cold reviewer in an adversarial language
        design process. You see ONLY the proposal. Your single angle: has this been
        tried? Name the specific paper, language, or project that attempted this
        exact decision (Cyclone, Mezzo, Linear Haskell, ATS, F*, Low*, Dafny,
        Liquid Haskell/Flux, Cogent, RustBelt/Iris, Vault, Plaid, Pony, Austral,
        Verona, separation-logic tools, region-inference work, etc.). State what it
        cost them, what they abandoned, and what the proposal must do differently to
        not repeat that history. If you genuinely know of no precedent, say so —
        but that itself is a flag worth recording."""),

    "judge": textwrap.dedent("""\
        You are the JUDGE in an adversarial language-design process. You do NOT
        propose. You receive a design proposal and the reports of five cold
        attackers (ergonomics, soundness, implementation, expressiveness,
        prior-art). For EACH attack, classify it as exactly one of:
          - "genuine-flaw": a real defect; the proposal must be revised.
          - "argued-past": the attacker overreached or the proposal already answers
            it; the attack fails.
          - "tradeoff": neither side simply wins; a real cost must be made explicit
            and recorded, but it does not by itself kill the proposal.
        Then render an overall decision:
          - "SURVIVED": no genuine-flaw; the decision enters the design document
            (possibly carrying recorded tradeoffs).
          - "FAILED": at least one genuine-flaw; the PROPOSER must revise.
          - "TRADEOFF": no fatal flaw, but the decision is only admissible alongside
            explicit tradeoffs that themselves need a future decision.
        Be impartial and specific. Cite the attacker's strongest point when you
        reject the proposal, and cite the proposal's answer when you reject an
        attack. Your verdict is itself challengeable, so make the reasoning legible."""),
}

ROLE_SYSTEM["reconciler"] = textwrap.dedent("""\
    You are the RECONCILER in an adversarial language-design process. You do NOT
    propose or attack. You are given TWO design decisions that have each already
    survived adversarial challenge, plus the language's target properties. Your job
    is the single most valuable output of the whole process: determine whether
    holding BOTH decisions at once FORCES a concrete incompatibility — i.e., to keep
    decision A and decision B together, which target property must be weakened or
    abandoned, and why, mechanically. Do not invent conflicts to look productive: if
    the two decisions are genuinely independent or mutually reinforcing, say so
    plainly. A false incompatibility is as harmful as a missed one.
    End with EXACTLY one fenced json block:
    ```json
    {"incompatible": true,
     "property_at_risk": "<the exact target property that must give, or 'none'>",
     "severity": "hard|soft|none",
     "description": "<one or two sentences: the mechanical reason they collide>"}
    ```
    'hard' = the two cannot both fully hold, ever. 'soft' = they coexist only with a
    runtime/ergonomic/scope cost. 'none' = no real conflict.""")

ATTACKER_ROLES = [
    "attacker-ergonomics",
    "attacker-soundness",
    "attacker-implementation",
    "attacker-expressiveness",
    "attacker-prior-art",
]


ROLE_SYSTEM["synthesizer"] = textwrap.dedent("""\
    You are the SYNTHESIZER in an adversarial language-design process. A design
    question has just FAILED every revision attempt: no proposal could satisfy all
    five attack angles at once. You are given the question, the target properties,
    and the attacks that LANDED (judged genuine flaws) across every revision. Your
    job is NOT to summarize round-by-round. It is to find the INVARIANT: the single
    recurring reason the question is unsettleable, expressed as a concrete
    incompatibility among the target properties — i.e. 'to get property X and
    property Y, property Z must give, mechanically, because <reason>'.

    This is the highest-value output of the process (the seed's Section 8 payload):
    a property-incompatibility PROVEN by survived adversarial challenge rather than
    asserted. Be precise about the mechanism. If the failures were actually
    scattered and unrelated (no real convergence — the proposer was just bad), say
    so honestly with converged=false; do not manufacture a clean story.
    End with EXACTLY one fenced json block:
    ```json
    {"converged": true,
     "properties_in_tension": ["<exact target-property phrases, 2-3>"],
     "severity": "hard|soft",
     "headline": "<= 14 words: the two properties that collide>",
     "description": "<2-4 sentences: the mechanical reason, with the concrete pattern that exposes it>"}
    ```""")


def build_synthesizer_prompt(question: str, failed_rounds: list[dict]) -> str:
    blocks = []
    for r in failed_rounds:
        landed = [
            a for a in r.get("attacks", [])
            if r.get("judge", {}).get("per_attack", {}).get(a["role"]) == "genuine-flaw"
        ]
        lines = [f"  Revision {r.get('revision','?')} (round {r.get('round','?')}):"]
        for a in landed:
            lines.append(f"    - {a['role']} [{a.get('severity','?')}]: "
                         f"{a.get('strongest_point') or a.get('summary','')}")
        lines.append(f"    JUDGE: {r.get('judge',{}).get('rationale','')}")
        blocks.append("\n".join(lines))
    return (
        _properties_block()
        + f"\n\nFAILED QUESTION:\n  {question}\n\n"
        "ATTACKS THAT LANDED ACROSS ALL REVISIONS:\n"
        + "\n\n".join(blocks)
        + "\n\nFind the invariant incompatibility among the target properties above."
    )


def build_reconciler_prompt(decision_a: dict, decision_b: dict) -> str:
    def fmt(d):
        return (
            f"DECISION [{d['question']}]\n"
            f"Commits to: {d.get('headline','(unstated)')}\n"
            f"Full proposal:\n{d.get('proposal_text','(text unavailable)')}"
        )
    return (
        _properties_block()
        + "\n\nTwo settled decisions follow. Determine whether holding both forces "
        "an incompatibility among the target properties above.\n\n"
        "=== DECISION A ===\n" + fmt(decision_a) + "\n=== END A ===\n\n"
        "=== DECISION B ===\n" + fmt(decision_b) + "\n=== END B ===\n"
    )


def _properties_block() -> str:
    props = "\n".join(f"  {i+1}. {p}" for i, p in enumerate(_ACTIVE["properties"]))
    cons = "\n".join(f"  - {c}" for c in _ACTIVE["constraints"])
    return (f"DESIGN PROGRAM: {_ACTIVE['banner']}\n\n"
            f"TARGET PROPERTIES (all must eventually hold simultaneously):\n{props}\n\n"
            f"HARD CONSTRAINTS:\n{cons}")


def build_proposer_prompt(
    question: str,
    survived_decisions: list[dict],
    prior_attacks: list[dict] | None = None,
) -> str:
    parts = [_properties_block(), ""]
    parts.append(f"THE DESIGN QUESTION FOR THIS ROUND:\n  {question}\n")

    if survived_decisions:
        lines = []
        for d in survived_decisions:
            lines.append(f"  - [{d['question']}] => {d.get('headline','(settled)')}")
            for t in d.get("tradeoffs", []):
                lines.append(f"      tradeoff carried: {t}")
        parts.append("ALREADY-SETTLED DECISIONS you must stay consistent with:\n" + "\n".join(lines) + "\n")
    else:
        parts.append("ALREADY-SETTLED DECISIONS: none yet — this is an early round.\n")

    if prior_attacks:
        lines = []
        for a in prior_attacks:
            lines.append(f"  - {a['role']} [{a.get('severity','?')}]: {a.get('strongest_point') or a.get('summary','')}")
        parts.append(
            "YOUR PREVIOUS PROPOSAL WAS DEFEATED. The attacks that landed:\n"
            + "\n".join(lines)
            + "\nProduce a REVISED decision that directly answers these. Do not hand-wave.\n"
        )

    parts.append("Now produce your design decision.")
    return "\n".join(parts)


def build_proposer_strengthen_prompt(question: str, current_proposal: str) -> str:
    """Self-critique pass: let the proposer harden its draft before attackers see it.
    Tests whether a STRONGER proposer settles questions a one-shot proposer cannot."""
    return (
        f"DESIGN QUESTION:\n  {question}\n\n"
        "Here is your current draft proposal. Five cold expert attackers are about to "
        "try to break it on soundness, decidability, implementability, expressiveness, "
        "and prior art. BEFORE they see it, find its 2-3 most dangerous holes yourself "
        "and produce a STRENGTHENED proposal that closes them — add the missing rule, "
        "tighten the fragment, fix the unsound case, or honestly narrow the claim. Do "
        "not merely restate it; make it materially harder to break. Keep it concrete "
        "and formal.\n\n"
        "=== CURRENT DRAFT ===\n" + current_proposal + "\n=== END DRAFT ===\n\n"
        "Output the full strengthened proposal (same structure: THE DECISION / HOW IT "
        "SERVES THE TARGET / ANTICIPATED ATTACKS)."
    )


ROLE_SYSTEM["meta-synthesizer"] = textwrap.dedent("""\
    You are the META-SYNTHESIZER. You are given several property-incompatibilities,
    each already proven by survived adversarial challenge, found while trying to
    design one language. Your job: decide whether they are N independent findings or
    N facets of ONE underlying theorem. If one theorem, state it precisely — the
    single structural reason, the property that is universally implicated, and the
    minimal claim — and show how each listed incompatibility follows from it. If they
    are genuinely independent, say so and do not force a false unification.
    End with EXACTLY one fenced json block:
    ```json
    {"one_theorem": true,
     "universal_property": "<the target property implicated in all of them>",
     "statement": "<the single theorem, stated precisely in one or two sentences>",
     "facets": [{"incompat": "<short label>", "how_it_follows": "<one sentence>"}]}
    ```""")


ROLE_SYSTEM["horn-classifier"] = textwrap.dedent("""\
    You are the HORN-CLASSIFIER in the constructive ("build-and-break the seam") phase
    of a language-design process. You do NOT propose or attack. You are given a seam
    specification (a dangerous primitive re-exported as a checked safe interface), the
    reports of the cold attackers who tried to break its re-import rule, and the judge's
    verdict. The seam claims three things at once — the SEAM TRILEMMA:
      - HORN 1, sound re-import: safe code using only the interface cannot reach UB.
      - HORN 2, decidable & proof-free safe fragment: ordinary code needs no user proof,
        no ghost/branded token threaded through it, no SMT/external prover.
      - HORN 3, seam concentration: ordinary code touches the seam rarely (ideally once at
        construction), not once per use.
    Your job: determine the OUTCOME for THIS seam and, if it is not fully sound,
    which horn the strongest surviving objection threatens — judged on the MERITS of the
    arguments, not by counting votes. A genuine UB-laundering path (horn 1) outranks
    everything; quote it exactly. If the obstruction was neutralized but only by pushing a
    cost onto the re-import boundary (auto-trait bounds, drop/storage ownership, a T-bound),
    say so and name whether that cost stays decidable (seam still holds, horn intact) or
    forces a proof/repetition (a horn drops). Distinguish "the obstruction MOVED onto the
    re-import boundary but stayed decidable" (the seam holds, with a marked price) from
    "a horn genuinely dropped." Be precise and do not flatter the design.
    End with EXACTLY one fenced json block:
    ```json
    {"outcome": "SEAM-HOLDS|SEAM-BREAKS|SEAM-COSTS",
     "horn_threatened": "horn1-sound-reimport|horn2-decidability|horn3-concentration|none|other",
     "horn1_ub_path": "<the exact safe-code sequence that launders UB, verbatim, or empty>",
     "obstruction_location": "<where the surviving difficulty sits: 'reimport-boundary' | 'inside-seam' | 'consumer-code' | 'none'>",
     "stayed_decidable": true,
     "headline": "<= 16 words: the verdict on this seam>",
     "rationale": "<= 70 words: the mechanical reason, citing the strongest attack>"}
    ```
    'SEAM-HOLDS' = all three horns intact (sound, decidable-proof-free, concentrated).
    'SEAM-BREAKS' = horn 1 dropped (a real UB path exists). 'SEAM-COSTS' = sound and
    decidable but a real, marked price (e.g. an explicit T-bound or a once-per-buffer
    crossing) must be recorded; no horn fully drops but the cost is named.""")


def build_horn_classifier_prompt(spec_text: str, attacks: list[dict],
                                 judge_text: str) -> str:
    atk_blocks = []
    for a in attacks:
        atk_blocks.append(
            f"--- ATTACKER {a['role']} "
            f"(verdict={a.get('verdict','?')}, severity={a.get('severity','?')}) ---\n"
            f"strongest: {a.get('strongest_point') or a.get('summary','')}\n"
            f"{a.get('text','')[:2400]}"
        )
    return (
        "=== SEAM SPECIFICATION UNDER TEST ===\n"
        f"{spec_text}\n"
        "=== END SPEC ===\n\n"
        "=== COLD ATTACKER REPORTS ===\n"
        + "\n\n".join(atk_blocks)
        + "\n=== END ATTACKS ===\n\n"
        "=== JUDGE VERDICT ===\n"
        f"{judge_text[:3000]}\n"
        "=== END JUDGE ===\n\n"
        "Classify the outcome for this seam by the trilemma horns."
    )


def build_meta_synthesizer_prompt(incompatibilities: list[dict]) -> str:
    blocks = []
    for i, ic in enumerate(incompatibilities, 1):
        blocks.append(
            f"{i}. [{ic.get('severity','?')}] {ic['between'][0]} ✕ {ic['between'][1]}\n"
            f"   {ic.get('description','')}"
        )
    return (
        _properties_block()
        + "\n\nThe following incompatibilities were each proven by survived "
        "adversarial challenge while trying to design ONE language with all the "
        "target properties above:\n\n"
        + "\n\n".join(blocks)
        + "\n\nAre these one theorem or several? If one, state it."
    )


def build_attacker_prompt(proposal_text: str, web: bool = False) -> str:
    # Cold: the proposal and the output contract, nothing else.
    tool_line = (
        "You MAY use WebSearch and WebFetch to confirm a precedent or find the "
        "paper/project; cite the URLs you actually retrieved. Do not browse for "
        "anything beyond prior art.\n\n"
        if web else
        "Reason from your own knowledge alone; do not use any tools.\n\n"
    )
    return (
        "Here is the design proposal to attack. You have no other context.\n\n"
        "=== PROPOSAL ===\n"
        f"{proposal_text}\n"
        "=== END PROPOSAL ===\n\n"
        + tool_line
        + _ATTACKER_OUTPUT_CONTRACT
    )


def build_judge_prompt(question: str, proposal_text: str, attacks: list[dict]) -> str:
    atk_blocks = []
    for a in attacks:
        atk_blocks.append(
            f"--- ATTACKER: {a['role']} "
            f"(self-reported verdict={a.get('verdict','?')}, severity={a.get('severity','?')}) ---\n"
            f"{a['text']}"
        )
    contract = textwrap.dedent("""\
        End your response with EXACTLY one fenced json block:
        ```json
        {"per_attack": {"attacker-ergonomics": "genuine-flaw|argued-past|tradeoff",
                         "attacker-soundness": "...",
                         "attacker-implementation": "...",
                         "attacker-expressiveness": "...",
                         "attacker-prior-art": "..."},
         "decision": "SURVIVED|FAILED|TRADEOFF",
         "tradeoffs": ["<each explicit tradeoff to record>"],
         "headline": "<= 15 words naming what this decision actually commits to",
         "rationale": "<= 50 words"}
        ```
    """)
    return (
        f"DESIGN QUESTION THIS ROUND:\n  {question}\n\n"
        "=== PROPOSAL ===\n"
        f"{proposal_text}\n"
        "=== END PROPOSAL ===\n\n"
        "=== ATTACKS ===\n"
        + "\n\n".join(atk_blocks)
        + "\n=== END ATTACKS ===\n\n"
        + "Judge each attack, then render the overall decision.\n\n"
        + contract
    )
