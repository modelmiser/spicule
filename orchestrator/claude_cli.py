"""Cold, stateless Claude Code CLI wrapper.

Every role instance is a fresh `claude -p` subprocess with the ambient session
context stripped out. This is the load-bearing 'cold instance' guarantee the
adversarial process depends on: an ATTACKER must reason from the proposal alone,
not from the operator's CLAUDE.md, memory, or hooks.

Verified isolation recipe (smoke-tested 2026-06-24):
  --system-prompt <role>            replace the Claude Code identity
  --setting-sources ""              load NO user/project/local settings => no hooks
  --strict-mcp-config               ignore all ambient MCP config...
  --mcp-config '{"mcpServers":{}}'  ...and register none
  --exclude-dynamic-system-prompt-sections   demote cwd/env/git/memory
  (run from a fresh temp cwd that contains no CLAUDE.md)

Model is intentionally NOT pinned: omitting --model inherits the operator's
best configured model (Opus on a flat-rate plan). Pass model= only to override.
"""
from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass, field


@dataclass
class CallResult:
    text: str
    cost_usd: float
    input_tokens: int
    output_tokens: int
    is_error: bool
    permission_denials: list = field(default_factory=list)
    raw: dict = field(default_factory=dict)


def _emptyish_cwd() -> str:
    """A fresh temp dir guarantees no project CLAUDE.md leaks into the call."""
    return tempfile.mkdtemp(prefix="spicule_cold_")


def _one_call(prompt: str, system: str, model: str | None, timeout: int,
              allow_web: bool = False) -> CallResult:
    cwd = _emptyish_cwd()
    cmd = [
        "claude", "-p", prompt,
        "--system-prompt", system,
        "--setting-sources", "",
        "--strict-mcp-config", "--mcp-config", '{"mcpServers":{}}',
        "--exclude-dynamic-system-prompt-sections",
        "--output-format", "json",
    ]
    if allow_web:
        # Whitelist only the web tools so they run without a permission prompt;
        # everything else stays unavailable, preserving the cold-reasoning stance.
        cmd += ["--allowedTools", "WebSearch", "WebFetch"]
    if model:
        cmd += ["--model", model]
    try:
        proc = subprocess.run(
            cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout
        )
    finally:
        shutil.rmtree(cwd, ignore_errors=True)

    # claude -p writes its diagnostics to STDOUT (often a JSON envelope with
    # is_error=true), so a failure report MUST include stdout, not just stderr.
    if proc.returncode != 0:
        raise RuntimeError(
            f"claude CLI exited rc={proc.returncode}. "
            f"stdout={proc.stdout.strip()[:600]!r} stderr={proc.stderr.strip()[:300]!r}"
        )
    try:
        data = json.loads(proc.stdout)
    except json.JSONDecodeError as e:
        raise RuntimeError(
            f"claude CLI returned non-JSON: {proc.stdout[:400]!r}"
        ) from e

    # rc==0 with is_error==true is a real failure (in-band API error); a missing
    # result is also a failure. Either must raise, never settle silently.
    if data.get("is_error") or not (data.get("result") or "").strip():
        raise RuntimeError(
            f"claude CLI in-band error: is_error={data.get('is_error')} "
            f"api_status={data.get('api_error_status')} "
            f"result={str(data.get('result'))[:200]!r}"
        )

    return CallResult(
        text=data["result"],
        cost_usd=float(data.get("total_cost_usd", 0.0) or 0.0),
        input_tokens=data.get("usage", {}).get("input_tokens", 0),
        output_tokens=data.get("usage", {}).get("output_tokens", 0),
        is_error=False,
        permission_denials=data.get("permission_denials", []) or [],
        raw=data,
    )


def call(
    prompt: str,
    system: str,
    *,
    model: str | None = None,
    timeout: int = 1800,
    label: str = "",
    retries: int = 1,
    allow_web: bool = False,
) -> CallResult:
    """Run one cold role instance and return its parsed result.

    Retries transient failures (overload/rate-limit) with backoff. Raises
    RuntimeError once retries are exhausted so a round fails loudly rather than
    settling a decision on a degraded or empty answer.
    """
    if label:
        print(f"  [cold-call] {label} ...", file=sys.stderr, flush=True)
    last: Exception | None = None
    for attempt in range(retries + 1):
        try:
            res = _one_call(prompt, system, model, timeout, allow_web=allow_web)
            if label:
                print(
                    f"  [cold-call] {label} done "
                    f"(${res.cost_usd:.3f}, {res.output_tokens} out tok"
                    + (f", retry {attempt}" if attempt else "") + ")",
                    file=sys.stderr, flush=True,
                )
            return res
        except RuntimeError as e:
            last = e
            if attempt < retries:
                wait = 10 * (attempt + 1)
                print(f"  [cold-call] {label} failed ({str(e)[:160]}); "
                      f"retry in {wait}s", file=sys.stderr, flush=True)
                time.sleep(wait)
    raise RuntimeError(f"{label!r} failed after {retries+1} attempts: {last}")
