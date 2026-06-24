"""Entry point: drive the adversarial design process.

Usage (from the newlang/ directory):
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
import sys
from pathlib import Path

from .historian import Historian
from .round import run_round, synthesize_failed_question

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

ROOT = Path(__file__).resolve().parent.parent
DESIGN_DIR = ROOT / "design"
RUNS_DIR = ROOT / "runs"


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
    ap.add_argument("--max-revisions", type=int, default=3)
    ap.add_argument("--survivals", type=int, default=3,
                    help="consecutive cold survivals required to settle (seed: 3)")
    ap.add_argument("--prior-art-web", action="store_true",
                    help="let ATTACKER-PRIOR-ART use WebSearch/WebFetch")
    args = ap.parse_args(argv)

    hist = Historian(DESIGN_DIR)

    if args.status:
        print_status(hist)
        return 0

    if args.synthesize_failed:
        qs = _failed_unsettled_questions(hist)
        if not qs:
            print("No un-synthesized failed questions.")
        for q in qs:
            print(f"\n>>> Synthesizing failure: {q[:70]}", file=sys.stderr)
            synthesize_failed_question(hist, q, RUNS_DIR)
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
        run_round(hist, q, RUNS_DIR, max_revisions=args.max_revisions,
                  survivals=args.survivals, prior_art_web=args.prior_art_web)

    print_status(hist)
    print(f"\nDesign document: {DESIGN_DIR / 'DESIGN.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
