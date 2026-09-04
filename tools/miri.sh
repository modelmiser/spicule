#!/bin/bash
# Run the seam crate under Miri. From the repo root: tools/miri.sh [extra cargo args]
#
# WHY THE FLAGS:
#   -Zmiri-ignore-leaks   Several seams leak DELIBERATELY and assert that they do
#                         (binary_session::forgotten_peer_starves_its_dual_forever,
#                         drain::leaking_the_guard_is_sound..., reseat::forget_is_a_leak...).
#                         With leak checking on, Miri fails on those by design, and an
#                         auditor learns to ignore its output — the cry-wolf failure.
#                         Leak behaviour is checked separately below.
#   --features test-helpers   mmio's helpers are gated; without this, mmio_use and
#                         mmio_witness compile to nothing and their sites go unexercised.
#
# COST: the whole suite is a multi-minute job under interpretation — spin_lock_use alone
# takes ~5 minutes (8 threads x 1000 iterations). This is not a per-commit check.
#
# COVERAGE LIMIT worth knowing: Miri reports UB on paths the tests EXECUTE. It cannot
# invent a call. A guard no test exercises stays unverified no matter how long this runs —
# see .crystal/audits/UNSAFE-sweep-2026-09-04.md findings 6-8.
set -u
cd "$(dirname "$0")/../seam" || exit 2

echo "== Miri, Stacked Borrows (default model) =="
MIRIFLAGS=-Zmiri-ignore-leaks rustup run nightly cargo miri test --features test-helpers "$@"
sb=$?

echo
echo "== Miri, Tree Borrows (the other candidate model) =="
MIRIFLAGS="-Zmiri-ignore-leaks -Zmiri-tree-borrows" rustup run nightly cargo miri test --features test-helpers "$@"
tb=$?

echo
echo "== Deliberate leaks still leak (leak checking ON, expected to FAIL) =="
echo "   A pass here would mean a seam stopped leaking where the module says it must."
MIRIFLAGS= rustup run nightly cargo miri test --features test-helpers --test drain_use 2>&1 \
  | tail -3

echo
echo "stacked-borrows exit=$sb  tree-borrows exit=$tb"
exit $(( sb | tb ))
