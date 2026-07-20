"""Tier-3 foreign-model replication — answer the monoculture threat.

The whole build-and-break instrument (proposer, attackers, judge, horn-classifier)
ran on ONE model family (Claude). A cold reviewer's sharpest objection: a single
model family's shared corpus folklore could emit the same verdicts regardless of
ground truth. This script re-runs the *adjudication* half of the gauntlet — the five
cold ATTACKERS + JUDGE + HORN-CLASSIFIER — against the SAME frozen seam specs, but
with a FOREIGN model (OpenAI / Gemini / Groq-hosted open weights) as every role, and
compares the foreign verdict to the recorded Claude verdict.

It deliberately reuses `roles.py`'s prompt builders verbatim, so the ONLY variable
changed is the model weights behind each role. It does NOT touch `claude_cli.py` or
the Claude driver — the original instrument stays pristine and comparable.

Coldness is automatic here: a stateless API call carries only the role system prompt
+ the proposal, no ambient context at all — strictly colder than the Claude subprocess.

Config via env (no secrets in code):
  FOREIGN_PROVIDER = openai | gemini | openai-compatible   (default openai-compatible)
  FOREIGN_MODEL    = e.g. gpt-4o / gemini-2.0-flash / llama-3.3-70b-versatile
  FOREIGN_API_KEY  = the key (required for a live run)
  FOREIGN_BASE_URL = for openai-compatible (Groq: https://api.groq.com/openai/v1,
                     OpenRouter: https://openrouter.ai/api/v1, default OpenAI)

Usage (from spicule/):
  python3 -m orchestrator.foreign_replication --dry-run          # mock, no key, prove wiring
  python3 -m orchestrator.foreign_replication --list             # show the stratified sample + est. calls
  FOREIGN_PROVIDER=openai-compatible FOREIGN_BASE_URL=https://api.groq.com/openai/v1 \
    FOREIGN_MODEL=llama-3.3-70b-versatile FOREIGN_API_KEY=... \
    python3 -m orchestrator.foreign_replication --seams spsc,slab,intrusive,secret,spin_lock
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
import urllib.request
import urllib.error
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

from . import roles
from .round import extract_json_block

ROOT = Path(__file__).resolve().parent.parent
DESIGN_DIR = ROOT / "design_seam"
OUT_DIR = ROOT / "runs_foreign"

# A stratified sample spanning all three verdict classes AND all three soundness
# domains, including the pivotal concurrency and IFC seams. (spec-stem -> note)
STRATIFIED_SAMPLE = {
    "spsc":         "HOLDS  · memory · the flagship fully-encapsulable hold (only 3/3 seam)",
    "spin_lock":    "HOLDS  · concurrency · the pivotal discharged-unsafe-impl hold (#37)",
    "slab":         "COSTS  · memory · [#6 zero-cost] runtime witness",
    "refined":      "COSTS  · memory · [horn2 solver-freedom]",
    "intrusive":    "BREAKS · memory · [horn1] whole-heap aliasing+liveness escapes",
    "secret":       "BREAKS · IFC · [horn1 IFC] declassification is etiquette",
    "sync_cell":    "BREAKS · concurrency · [horn1] the by-design data-race probe",
    "ordered_locks":"BREAKS · liveness · [horn1] per-token vs per-thread-global",
}

# Recorded Claude verdict, parsed live from each REPORT file's horn-classifier line;
# this dict is only a fallback/label for seams whose report is unusual.
VERDICT_RE = re.compile(r"horn classifier:\*\*\s*(SEAM-\w+)\s*(\[[^\]]*\])?", re.I)


# ----------------------------------------------------------------------------- foreign client

def _http_post(url: str, headers: dict, payload: dict, timeout: int) -> dict:
    body = json.dumps(payload).encode()
    req = urllib.request.Request(url, data=body, headers=headers, method="POST")
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.loads(resp.read().decode())


def foreign_call(prompt: str, system: str, *, label: str = "", timeout: int = 300,
                 retries: int = 2, dry_run: bool = False) -> str:
    """One cold foreign-model role call. Returns raw text. Mock in --dry-run."""
    if dry_run:
        return _MOCK.pop(0) if _MOCK else _mock_reply(system)
    provider = os.environ.get("FOREIGN_PROVIDER", "openai-compatible").lower()
    model = os.environ.get("FOREIGN_MODEL", "")
    key = os.environ.get("FOREIGN_API_KEY", "")
    if not model or not key:
        raise SystemExit("FOREIGN_MODEL and FOREIGN_API_KEY must be set for a live run "
                         "(or pass --dry-run).")
    last: Exception | None = None
    for attempt in range(retries + 1):
        try:
            if provider == "gemini":
                base = os.environ.get("FOREIGN_BASE_URL",
                                      "https://generativelanguage.googleapis.com/v1beta")
                url = f"{base}/models/{model}:generateContent?key={key}"
                payload = {
                    "systemInstruction": {"parts": [{"text": system}]},
                    "contents": [{"role": "user", "parts": [{"text": prompt}]}],
                    "generationConfig": {"temperature": 0.0},
                }
                data = _http_post(url, {"Content-Type": "application/json"}, payload, timeout)
                return data["candidates"][0]["content"]["parts"][0]["text"]
            else:  # openai | openai-compatible (OpenAI, Groq, OpenRouter, DeepSeek, xAI, Together)
                base = os.environ.get("FOREIGN_BASE_URL", "https://api.openai.com/v1")
                url = f"{base}/chat/completions"
                payload = {
                    "model": model,
                    "messages": [{"role": "system", "content": system},
                                 {"role": "user", "content": prompt}],
                    "temperature": 0.0,
                }
                headers = {"Content-Type": "application/json",
                           "Authorization": f"Bearer {key}"}
                data = _http_post(url, headers, payload, timeout)
                return data["choices"][0]["message"]["content"]
        except (urllib.error.HTTPError, urllib.error.URLError, KeyError, TimeoutError) as e:
            last = e
            detail = ""
            if isinstance(e, urllib.error.HTTPError):
                try: detail = e.read().decode()[:300]
                except Exception: pass
            if attempt < retries:
                wait = 8 * (attempt + 1)
                print(f"  [foreign] {label} failed ({str(e)[:120]} {detail[:120]}); "
                      f"retry in {wait}s", file=sys.stderr, flush=True)
                time.sleep(wait)
    raise RuntimeError(f"{label!r} failed after {retries+1} attempts: {last}")


# ----------------------------------------------------------------------------- mock (dry-run wiring)

_MOCK: list[str] = []

def _mock_reply(system: str) -> str:
    if "HORN-CLASSIFIER" in system:
        return ('mock classifier\n```json\n{"outcome":"SEAM-HOLDS",'
                '"horn_threatened":"none","horn1_ub_path":"","obstruction_location":"none",'
                '"stayed_decidable":true,"headline":"mock","rationale":"mock"}\n```')
    if "JUDGE" in system:
        return ('mock judge\n```json\n{"per_attack":{},"decision":"SURVIVED",'
                '"tradeoffs":[],"headline":"mock","rationale":"mock"}\n```')
    return ('mock attacker\n```json\n{"verdict":"survivable","severity":"low",'
            '"summary":"mock","strongest_point":"mock"}\n```')


# ----------------------------------------------------------------------------- the gauntlet

def recorded_verdict(stem: str) -> str:
    rpt = DESIGN_DIR / f"REPORT_SEAM_SPEC_{stem}.md"
    if not rpt.exists():
        return "?"
    # take the LAST horn-classifier line = the seam's settled verdict (SPSC ran 3
    # gauntlets: TRADEOFF then SURVIVED×2, so the final line is the HOLD, not gauntlet 1).
    ms = VERDICT_RE.findall(rpt.read_text())
    if not ms:
        return "?"
    verdict, horn = ms[-1]
    return f"{verdict.upper()} {horn or ''}".strip()


def run_one_seam(stem: str, *, prior_art_web: bool, dry_run: bool) -> dict:
    spec_path = DESIGN_DIR / f"SEAM_SPEC_{stem}.md"
    spec_text = spec_path.read_text()
    seam_out = OUT_DIR / stem
    seam_out.mkdir(parents=True, exist_ok=True)

    # 1) five cold attackers (foreign), in parallel — note: no web tool on the foreign
    # side, so the prior-art attacker reasons from parametric knowledge only (a recorded
    # delta vs the Claude run, which had WebSearch).
    def one_attacker(role: str) -> dict:
        txt = foreign_call(roles.build_attacker_prompt(spec_text, web=False),
                           roles.ROLE_SYSTEM[role], label=f"{stem} {role}", dry_run=dry_run)
        (seam_out / f"{role}.md").write_text(txt)
        p = extract_json_block(txt) or {}
        return {"role": role, "text": txt, "verdict": p.get("verdict", "?"),
                "severity": p.get("severity", "?"), "summary": p.get("summary", ""),
                "strongest_point": p.get("strongest_point", "")}
    with ThreadPoolExecutor(max_workers=len(roles.ATTACKER_ROLES)) as ex:
        attacks = list(ex.map(one_attacker, roles.ATTACKER_ROLES))

    # 2) judge (foreign)
    judge_sys = roles.ROLE_SYSTEM.get("judge", "You are the JUDGE. Rule SURVIVED/FAILED/TRADEOFF.")
    judge_txt = foreign_call(
        roles.build_judge_prompt(f"Does the {stem} seam re-import soundly?", spec_text, attacks),
        judge_sys, label=f"{stem} judge", dry_run=dry_run)
    (seam_out / "judge.md").write_text(judge_txt)

    # 3) horn-classifier (foreign)
    cls_txt = foreign_call(
        roles.build_horn_classifier_prompt(spec_text, attacks, judge_txt),
        roles.ROLE_SYSTEM["horn-classifier"], label=f"{stem} horn-classifier", dry_run=dry_run)
    (seam_out / "horn.md").write_text(cls_txt)
    cls = extract_json_block(cls_txt) or {}

    foreign_outcome = cls.get("outcome", "?")
    foreign_horn = cls.get("horn_threatened", "?")
    rec = recorded_verdict(stem)
    # agreement = same outcome class (HOLDS/COSTS/BREAKS); horn compared secondarily
    rec_class = rec.split()[0] if rec != "?" else "?"
    agree = (foreign_outcome.upper() == rec_class.upper()) if foreign_outcome != "?" else None
    return {"seam": stem, "recorded": rec, "foreign_outcome": foreign_outcome,
            "foreign_horn": foreign_horn, "agree": agree,
            "foreign_headline": cls.get("headline", "")}


def main() -> None:
    ap = argparse.ArgumentParser(description="Foreign-model replication of seam verdicts")
    ap.add_argument("--seams", default="", help="comma-separated spec stems; default = stratified sample")
    ap.add_argument("--dry-run", action="store_true", help="mock foreign calls; prove wiring, no key/spend")
    ap.add_argument("--list", action="store_true", help="list the sample + call estimate and exit")
    ap.add_argument("--prior-art-web", action="store_true", help="(no-op on foreign side; recorded delta)")
    args = ap.parse_args()

    stems = [s.strip() for s in args.seams.split(",") if s.strip()] or list(STRATIFIED_SAMPLE)
    calls_per_seam = len(roles.ATTACKER_ROLES) + 2  # attackers + judge + classifier
    if args.list:
        print(f"Stratified sample ({len(stems)} seams, {calls_per_seam} foreign calls each = "
              f"{len(stems)*calls_per_seam} calls):")
        for s in stems:
            note = STRATIFIED_SAMPLE.get(s, "(custom)")
            print(f"  {s:16s} recorded={recorded_verdict(s):22s} {note}")
        return

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    if args.dry_run:
        print("[dry-run] mock foreign calls — proving wiring only.\n")
    else:
        prov = os.environ.get("FOREIGN_PROVIDER", "openai-compatible")
        print(f"[live] provider={prov} model={os.environ.get('FOREIGN_MODEL','?')} "
              f"— {len(stems)*calls_per_seam} calls on YOUR key.\n")

    results = []
    for stem in stems:
        r = run_one_seam(stem, prior_art_web=args.prior_art_web, dry_run=args.dry_run)
        results.append(r)
        mark = {True: "✓ agree", False: "✗ DIVERGE", None: "? unparsed"}[r["agree"]]
        print(f"  {stem:16s} recorded={r['recorded']:22s} "
              f"foreign={r['foreign_outcome']:11s}[{r['foreign_horn']}]  {mark}")

    agreed = sum(1 for r in results if r["agree"] is True)
    total = sum(1 for r in results if r["agree"] is not None)
    print(f"\nAGREEMENT: {agreed}/{total} seams — a foreign panel reproduced the recorded "
          f"outcome class on {agreed} of {total} scored seams.")
    (OUT_DIR / "agreement.json").write_text(json.dumps(results, indent=2))
    print(f"Per-seam transcripts + agreement.json under {OUT_DIR.relative_to(ROOT)}/")


if __name__ == "__main__":
    main()
