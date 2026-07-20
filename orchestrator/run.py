"""Entry point: drive the adversarial design process.

Usage (from the spicule/ directory):
  python3 -m orchestrator.run                  # run the next unsettled question
  python3 -m orchestrator.run --question N     # run question index N (0-based)
  python3 -m orchestrator.run --all            # run every unsettled question once
  python3 -m orchestrator.run --status         # print state, run nothing
  python3 -m orchestrator.run --synthesize-failed  # backfill payload from failed Qs
  python3 -m orchestrator.run --survivals 2 --prior-art-web

Each question runs PROPOSER → (must survive `--survivals` consecutive cold
gauntlets of 5 ATTACKERs + JUDGE) → settle, revising on any FAILED up to
`--max-revisions`. On exhaustion the SYNTHESIZER records the convergent failure.
Output: design/state.json + design/DESIGN.md; raw transcripts in runs/.
"""
from __future__ import annotations

import argparse
import re
import sys
import tempfile
from pathlib import Path

from . import roles
from .claude_cli import call
from .historian import Historian
from .round import extract_json_block, run_round, synthesize_failed_question

# Design questions in dependency order. The original Section-7 memory-model
# question bundled four concerns and proved unsettleable (round 1); it is split:
# the sequential core ([0], judged sound every round) from the interior-mutability
# / weak-memory corner ([4], where the real incompatibility lives).
QUESTIONS = [
    "Sequential ownership core — for SINGLE-THREADED code only (no concurrency, "
    "atomics, or interior mutability), what move/borrow/lifetime discipline gives "
    "memory safety without GC, derived from the proof layer rather than a hardcoded "
    "borrow checker?",
    "Decidability boundary — precisely which fragment of dependent types is "
    "admitted, with the formal argument that it is decidable?",
    "Proof erasure — how are proof witnesses eliminated at compile time with a "
    "formal guarantee they leave no runtime trace?",
    "Unsafe boundary — what does 'unsafe' mean here, and which invariants does "
    "crossing it break and then restore?",
    "Interior mutability & weak memory — how are shared-mutable cells, atomics, "
    "MMIO/device registers (read-to-clear, write-1-to-clear), and ARM/POWER/RISC-V "
    "memory ordering typed, given the sequential ownership core cannot express them?",
]

# SEAMFUL program — the constructive flip of the impossibility result. The unified
# sweep proved keeping all 7 properties is impossible; this asks what the best design
# is once #2 (no-seam) is deliberately conceded to a first-class, machine-checked,
# concentrated seam. A coherent design program, in dependency order.
SEAMFUL_QUESTIONS = [
    "The SEAM construct — give the formal syntax and typing of the seam: how ordinary "
    "(safe) code enters proof-land, what machine-checked obligation it discharges there, "
    "and how it re-exports a CHECKED SAFE INTERFACE that ordinary code then uses WITHOUT "
    "itself crossing the seam. Give formation / introduction / elimination rules.",
    "The SAFE FRAGMENT — the decidable, proof-free, GC-free ownership discipline for "
    "ordinary single-threaded code that needs NO seam and NO SMT, derived from the proof "
    "layer. State it precisely (move/borrow/lifetime rules).",
    "BEHIND THE SEAM — concretely, what obligation does a foundation author discharge, "
    "in-language, to implement (a) a lock-free SPSC queue, (b) an MMIO read-to-clear "
    "register, (c) an intrusive doubly-linked list — and how does each re-export a safe "
    "interface the safe fragment can use without proofs?",
    "SEAM SOUNDNESS / re-import — the rule that makes crossing BACK sound: how does the "
    "safe fragment trust a seam-crossed primitive without re-verifying it, and what "
    "structurally stops an unsound obligation from laundering UB into safe code?",
    "SEAM CONCENTRATION & ergonomics — the language mechanism and the argument that keep "
    "ordinary code from ever crossing the seam (the checked-interface boundary), and why "
    "the obligation stays with a handful of foundation authors. Exhibit a realistic "
    "program where ~99% of code is seam-free.",
    "ZERO-COST SEAM ERASURE — how every seam obligation/witness erases at compile time "
    "with a formal guarantee it leaves no runtime trace.",
]

# SEAMFUL2: same program, but the safe fragment is a known-good hardcoded checker so
# the gauntlet bears entirely on the seam (question [1] adopts NLL instead of deriving).
SEAMFUL2_QUESTIONS = list(SEAMFUL_QUESTIONS)
SEAMFUL2_QUESTIONS[1] = (
    "The SAFE FRAGMENT — ADOPT Rust's Non-Lexical-Lifetimes borrow checker as shipped "
    "(hardcoded, decidable, proof-free) for ordinary single-threaded code. Do NOT "
    "re-derive it. State only the SAFE INTERFACE it presents to the seam: what a "
    "seam-crossed primitive must hand back for safe code to use it without proofs."
)

PROGRAMS = {"unified": QUESTIONS, "seamful": SEAMFUL_QUESTIONS,
            "seamful2": SEAMFUL2_QUESTIONS}

ROOT = Path(__file__).resolve().parent.parent
DESIGN_DIRS = {"unified": ROOT / "design", "seamful": ROOT / "design_seamful",
               "seamful2": ROOT / "design_seamful2"}
RUNS_DIRS = {"unified": ROOT / "runs", "seamful": ROOT / "runs_seamful",
             "seamful2": ROOT / "runs_seamful2"}
# Defaults (overridden in main() once --program is parsed).
DESIGN_DIR = DESIGN_DIRS["unified"]
RUNS_DIR = RUNS_DIRS["unified"]


def next_unsettled(hist: Historian) -> int | None:
    for i, q in enumerate(QUESTIONS):
        if not hist.is_settled(q):
            return i
    return None


def _failed_unsettled_questions(hist: Historian) -> list[str]:
    """Distinct questions with FAILED rounds, not settled, not yet synthesized."""
    already = {i.get("from_question") for i in hist.state["incompatibilities"]}
    seen, out = set(), []
    for r in hist.state["rounds"]:
        q = r["question"]
        if (r["outcome"] == "FAILED" and not hist.is_settled(q)
                and q not in already and q not in seen):
            seen.add(q)
            out.append(q)
    return out


def meta_synthesize(hist: Historian) -> None:
    """Ask whether the recorded incompatibilities are facets of one theorem."""
    incs = hist.state["incompatibilities"]
    if not incs:
        print("No incompatibilities to meta-synthesize.")
        return
    r = call(roles.build_meta_synthesizer_prompt(incs),
             roles.ROLE_SYSTEM["meta-synthesizer"], label="meta-synthesizer")
    parsed = extract_json_block(r.text) or {}
    out = DESIGN_DIR / "THEOREM.md"
    body = ["# Meta-Synthesis — are the incompatibilities one theorem?", "",
            f"*Over {len(incs)} recorded incompatibilities.*", ""]
    if parsed.get("one_theorem"):
        body += [f"## One theorem (universal property: {parsed.get('universal_property','?')})",
                 "", f"> {parsed.get('statement','')}", "", "## How each follows", ""]
        for f in parsed.get("facets", []):
            body.append(f"- **{f.get('incompat','?')}** — {f.get('how_it_follows','')}")
    else:
        body += ["## Not one theorem", "", parsed.get("statement", "(independent findings)")]
    body += ["", "---", "", "## Full reasoning", "", r.text.strip()]
    out.write_text("\n".join(body))
    print(f"\nMeta-synthesis → {out}")
    if parsed.get("one_theorem"):
        print(f"  ONE THEOREM ({parsed.get('universal_property','?')}): "
              f"{parsed.get('statement','')[:200]}")


def print_status(hist: Historian) -> None:
    s = hist.state
    print(f"\nDesign state: {len(s['settled'])}/{len(QUESTIONS)} questions settled, "
          f"{len(s['rounds'])} rounds, {len(s['incompatibilities'])} "
          f"incompatibilities, notional cost ${s['total_cost_usd']:.2f}.")
    for i, q in enumerate(QUESTIONS):
        mark = "✅" if hist.is_settled(q) else "⬚"
        print(f"  {mark} [{i}] {q[:78]}")
    if s["incompatibilities"]:
        print("\nForced incompatibilities (the payload):")
        for ic in s["incompatibilities"]:
            print(f"  🔗 [{ic['severity']}/{ic.get('source','?')}] "
                  f"{ic['between'][0][:36]} ✕ {ic['between'][1][:36]}")
    if s["blind_spot_flags"]:
        print(f"\n⚠️  {len(s['blind_spot_flags'])} coldness/blind-spot flag(s).")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Adversarial language-design driver")
    ap.add_argument("--question", type=int, default=None,
                    help="run a specific question by 0-based index")
    ap.add_argument("--all", action="store_true",
                    help="run every unsettled question once, in order")
    ap.add_argument("--status", action="store_true", help="print state and exit")
    ap.add_argument("--synthesize-failed", action="store_true",
                    help="backfill payload incompatibilities from failed questions")
    ap.add_argument("--meta-synthesize", action="store_true",
                    help="ask whether the recorded incompatibilities are one theorem")
    ap.add_argument("--max-revisions", type=int, default=3)
    ap.add_argument("--survivals", type=int, default=3,
                    help="consecutive cold survivals required to settle (seed: 3)")
    ap.add_argument("--proposer-shots", type=int, default=1,
                    help="self-strengthening passes before each gauntlet (stronger proposer)")
    ap.add_argument("--prior-art-web", action="store_true",
                    help="let ATTACKER-PRIOR-ART use WebSearch/WebFetch")
    ap.add_argument("--scratch", action="store_true",
                    help="use a throwaway design dir (don't touch design/state.json)")
    ap.add_argument("--program", choices=list(PROGRAMS), default="unified",
                    help="which design program: 'unified' (keep all 7) or 'seamful'")
    args = ap.parse_args(argv)

    # Select the active program: question set, design dir, and role target-properties.
    global QUESTIONS, DESIGN_DIR, RUNS_DIR
    QUESTIONS = PROGRAMS[args.program]
    DESIGN_DIR = DESIGN_DIRS[args.program]
    RUNS_DIR = RUNS_DIRS[args.program]
    roles.set_program(args.program)

    if args.scratch:
        d = Path(tempfile.mkdtemp(prefix="spicule_scratch_design_"))
        runs_dir = Path(tempfile.mkdtemp(prefix="spicule_scratch_runs_"))
        print(f"[scratch] design={d} runs={runs_dir}", file=sys.stderr)
        hist = Historian(d)
    else:
        runs_dir = RUNS_DIR
        hist = Historian(DESIGN_DIR)

    if args.status:
        print_status(hist)
        return 0

    if args.meta_synthesize:
        meta_synthesize(hist)
        return 0

    if args.synthesize_failed:
        qs = _failed_unsettled_questions(hist)
        if not qs:
            print("No un-synthesized failed questions.")
        for q in qs:
            print(f"\n>>> Synthesizing failure: {q[:70]}", file=sys.stderr)
            synthesize_failed_question(hist, q, runs_dir)
        print_status(hist)
        return 0

    if args.all:
        targets = [i for i, q in enumerate(QUESTIONS) if not hist.is_settled(q)]
    elif args.question is not None:
        if not (0 <= args.question < len(QUESTIONS)):
            ap.error(f"--question must be in [0, {len(QUESTIONS)-1}]")
        targets = [args.question]
    else:
        nxt = next_unsettled(hist)
        if nxt is None:
            print("All questions settled. Use --question N to re-open one.")
            print_status(hist)
            return 0
        targets = [nxt]

    for idx in targets:
        q = QUESTIONS[idx]
        print(f"\n>>> Driving question [{idx}]: {q[:70]}", file=sys.stderr)
        run_round(hist, q, runs_dir, max_revisions=args.max_revisions,
                  survivals=args.survivals, prior_art_web=args.prior_art_web,
                  proposer_shots=args.proposer_shots)

    print_status(hist)
    print(f"\nDesign document: {DESIGN_DIR / 'DESIGN.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
