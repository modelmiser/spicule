"""Build-and-break the seam — inject a builder-authored spec, red-team it cold.

`run.py` drives the cold PROPOSER to GENERATE proposals; this driver does NOT. The
seam spec is authored by the stateful builder (the operator, or a context-holding
Claude) and only the cold attack+judge gauntlet runs against it. That preserves the
build/break separation the constructive phase requires: cold instances are the red
team, never the author (using them to build reproduces the `seamful` contamination —
"sound in spirit, defective as written"). After each gauntlet a cold HORN-CLASSIFIER
maps the outcome onto the Seam Trilemma horns.

It runs up to N CONSECUTIVE gauntlets on a FIXED spec and stops on the first FAILED,
handing control back to the (stateful) builder to revise the spec and re-run — the
human/stateful side owns revision; the cold side only demolishes.

Usage (from newlang/):
  python3 -m orchestrator.seam_phase --spec design_seam/SEAM_SPEC_spsc.md --gauntlets 1
  python3 -m orchestrator.seam_phase --spec design_seam/SEAM_SPEC_spsc.md --gauntlets 3 --prior-art-web
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

from . import roles
from .claude_cli import call
from .historian import Historian
from .round import _gauntlet, extract_json_block

ROOT = Path(__file__).resolve().parent.parent
DESIGN_DIR = ROOT / "design_seam"
RUNS_DIR = ROOT / "runs_seam"


def _log(m: str) -> None:
    print(m, file=sys.stderr, flush=True)


def classify_horn(spec_text: str, rec: dict, runs_dir: Path) -> tuple[dict, str]:
    """Cold horn-classifier over the SAVED transcripts of one gauntlet (no re-spend)."""
    tag = f"r{rec['round']}"
    attacks = []
    for a in rec["attacks"]:
        f = runs_dir / f"{tag}_{a['role']}.md"
        attacks.append({**a, "text": f.read_text() if f.exists() else ""})
    judge_f = runs_dir / f"{tag}_judge.md"
    judge_text = (judge_f.read_text() if judge_f.exists()
                  else rec["judge"].get("rationale", ""))
    r = call(roles.build_horn_classifier_prompt(spec_text, attacks, judge_text),
             roles.ROLE_SYSTEM["horn-classifier"], label=f"{tag} horn-classifier")
    parsed = extract_json_block(r.text) or {}
    (runs_dir / f"{tag}_horn.md").write_text(r.text)
    return parsed, r.text


def run_seam_phase(spec_path: Path, gauntlets: int, prior_art_web: bool,
                   design_dir: Path, runs_dir: Path) -> tuple[list[dict], int]:
    roles.set_program("seamful2")  # frame any properties-aware role in the seam program
    spec_path = Path(spec_path)
    spec_text = spec_path.read_text()
    runs_dir.mkdir(parents=True, exist_ok=True)
    design_dir.mkdir(parents=True, exist_ok=True)
    hist = Historian(design_dir)
    question = (f"Attack ONLY the re-import rule of this seam spec ({spec_path.name}); "
                "the NLL floor is stipulated sound and out of scope.")

    streak = 0
    classifications: list[dict] = []
    for g in range(1, gauntlets + 1):
        _log(f"\n=== seam gauntlet {g}/{gauntlets} on {spec_path.name} ===")
        rec = _gauntlet(hist, question, spec_text, 0, runs_dir, prior_art_web)
        hist.save()
        horn, _ = classify_horn(spec_text, rec, runs_dir)
        classifications.append({"gauntlet": g, "round": rec["round"],
                                "outcome": rec["outcome"], "horn": horn})
        _log(f"  HORN-CLASSIFIER → {horn.get('outcome','?')} "
             f"[{horn.get('horn_threatened','?')}] {horn.get('headline','')}")
        if rec["outcome"] == "FAILED":
            _log(f"  ✋ gauntlet FAILED at streak {streak}; stopping for builder revision.")
            break
        streak += 1
        _log(f"  survival {streak}/{gauntlets}")

    _write_report(spec_path, design_dir, classifications, streak, gauntlets)
    return classifications, streak


def _write_report(spec_path: Path, design_dir: Path, classifications: list[dict],
                  streak: int, gauntlets: int) -> None:
    out = design_dir / f"REPORT_{spec_path.stem}.md"
    # A TRADEOFF is a survival-with-recorded-cost (no fatal flaw), so it counts toward the
    # streak; only a FAILED stops early. "Held" = ran the full streak with no FAILED.
    failed_early = streak < gauntlets
    any_cost = any(c["outcome"] == "TRADEOFF" for c in classifications)
    status = ("stopped early (a gauntlet FAILED)" if failed_early
              else "SEAM HELD with recorded cost" if any_cost
              else "SEAM HELD")
    lines = [f"# Seam phase report — {spec_path.name}", "",
             f"*{streak}/{gauntlets} consecutive cold survivals. {status}.*", ""]
    for c in classifications:
        h = c["horn"]
        mark = {"SURVIVED": "✅", "FAILED": "❌", "TRADEOFF": "⚖️"}.get(c["outcome"], "•")
        lines += [f"## Gauntlet {c['gauntlet']} (round {c['round']}) — {mark} {c['outcome']}",
                  "",
                  f"- **horn classifier:** {h.get('outcome','?')} "
                  f"[{h.get('horn_threatened','?')}]",
                  f"- **obstruction sits at:** {h.get('obstruction_location','?')} "
                  f"(stayed decidable: {h.get('stayed_decidable','?')})",
                  f"- **headline:** {h.get('headline','')}",
                  f"- **rationale:** {h.get('rationale','')}"]
        if (h.get("horn1_ub_path") or "").strip():
            lines += ["", "- **⚠️ HORN-1 UB LAUNDERING PATH:**", "  ```",
                      "  " + h["horn1_ub_path"].replace("\n", "\n  "), "  ```"]
        lines.append("")
    out.write_text("\n".join(lines))
    _log(f"\nReport → {out}")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Build-and-break a seam spec (cold red team)")
    ap.add_argument("--spec", required=True, help="path to the builder-authored seam spec")
    ap.add_argument("--gauntlets", type=int, default=1,
                    help="max consecutive cold gauntlets (1 = Phase A, 3 = Phase B)")
    ap.add_argument("--prior-art-web", action="store_true",
                    help="let ATTACKER-PRIOR-ART use WebSearch/WebFetch")
    ap.add_argument("--design-dir", default=str(DESIGN_DIR))
    ap.add_argument("--runs-dir", default=str(RUNS_DIR))
    args = ap.parse_args(argv)

    classifications, streak = run_seam_phase(
        Path(args.spec), args.gauntlets, args.prior_art_web,
        Path(args.design_dir), Path(args.runs_dir))
    print(f"\nStreak: {streak}/{args.gauntlets} survivals on {Path(args.spec).name}")
    for c in classifications:
        print(f"  G{c['gauntlet']}: {c['outcome']} → "
              f"{c['horn'].get('outcome','?')} [{c['horn'].get('horn_threatened','?')}]")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
