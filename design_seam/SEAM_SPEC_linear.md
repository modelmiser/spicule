# Seam spec — must-run cleanup guard (re-import rule under attack) — r24 (revised after r23)

This is a **design proposal** for a seventeenth *seam*, built **breadth-wise**: not a new domain and not a
new rung of the granularity ladder, but a new **precondition** — *"this finalizer MUST run"* — to be
**graded** against Rust's ambient **affine** floor. It is the *inverse* of the tenth seam
(`SEAM_SPEC_drain.md`): `drain` **closed** by making a skipped `Drop` *harmless* (leak-**tolerance**); this
seam needs a skipped `Drop` to be *impossible* (leak-**intolerance**) — the same ninth-axis danger
(leak-freedom) from the opposite side.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/linear.rs`, the
default `no_std` build — `core`-only).

**Revision note (r24).** The first round (r23) FAILED on a genuine spec incoherence the ergonomics attacker
caught — not on the break (which is the designed result): §1/§2 motivated the primitive with
*commit-or-rollback* (a **conditional** finalizer that must NOT run on the happy path) while §2's
precondition demanded the finalizer run on **every** discard path, and the only cancel was `mem::forget`
(the named soundness hole). This revision **scopes the primitive to *unconditional* finalizers** and adds a
**sanctioned `disarm()`** for the conditional case (distinct from `mem::forget`). It also downgrades the
"partial-via-lint" grading claim to a clean *none* (per the r23 prior-art finding) and corrects two factual
nits. The break itself is unchanged and pre-conceded; the panel is asked to confirm it is now the *only*
defect.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker as shipped: ownership is **affine** (a value may be used *at most* once — you may
drop, move, or `mem::forget` it without using it). Destructors run via `Drop` on a normal or unwinding scope
exit, **except** where suppressed by a *safe* operation: `core::mem::forget`, `ManuallyDrop`, `Box::leak`,
or a reference cycle. The `#[must_use]` lint is a **non-type advisory** that ships with the compiler — it is
*not* part of the type-checked floor proper (it is silenced by `let _ =`, does not propagate through
fields/containers, and is blind to `forget`/`ManuallyDrop`); treat it as advisory, not as a floor fragment.
Do not attack the borrow checker, the closure machinery, or the fact that `forget` is safe; attack the
**re-import rule** (the must-run promise) and the §6 grading deliverable.

## 1. The primitive

A **RAII scope-exit guard for an *unconditional* finalizer**: `MustRun::new(cleanup)` arms a one-shot
closure that must run *regardless of how the scope ends and regardless of success/failure* — release a lock,
delete a temp file, decrement a refcount, restore a global. The danger is a **liveness / correctness**
hazard, not UB and not a memory leak per se: the *finalizer does not run*, so an invariant the program
relies on is silently violated. A program that skips the cleanup is memory-safe, confidential, and may even
make progress — it is just *wrong*.

For the **conditional** idiom (commit-or-rollback: on a successful commit the rollback must *not* run),
`MustRun::disarm(self) -> Option<F>` is the **sanctioned, greppable cancel** — explicitly *not*
`mem::forget`. The unconditional primitive plus an explicit disarm covers both shapes without making the
happy-path cancel identical to the attack.

## 2. The true precondition (cathedral side)

The finalizer is **must-run** iff, on every control-flow path that abandons the guarded scope *without an
explicit `disarm`* (normal exit, early return, `?`, panic-unwind, *and* any path that drops the guard), the
cleanup closure runs **exactly once**. This is a **linearity** requirement: the guard must be *consumed*
(used — by running, or by an explicit disarm), not *silently discarded*. The precondition is about
**unintended** skips (`forget`/`ManuallyDrop`/leak), which it forbids; an **intended** cancel (`disarm`) is
a sanctioned consumption, not a violation. (Scope: a single, *infallible* one-shot finalizer; ordering of
multiple finalizers, async cancellation points, `panic`-during-`panic` abort, and a finalizer that *itself
fails* are out of scope, §5.)

## 3. The witness — affine ownership + `Drop` (and a cosmetic advisory)

`MustRun<F: FnOnce()>` owns the closure in an `Option<F>` (one-shot: run at most once, whichever path
consumes it). `Drop` runs the closure; `run_now(self)` is the explicit consume-and-run; `disarm(self)`
explicitly cancels (returns the un-run closure). The struct is `#[must_use]`, which — per §0 — is a
**cosmetic advisory**: it lints (and, under `#![deny(unused_must_use)]`, errors) the commonest *accidental*
misuse, a guard created and dropped at statement position. It is **not** a floor fragment and carries none
of the linearity (it is blind to `forget`/`ManuallyDrop`/leak). The deployed `MustRun` — the `scopeguard`
crate — ships exactly this `#[must_use]` warning, with `into_inner`/`ManuallyDrop` as its defuse.

**What the affine floor genuinely carries:** every *ordinary, non-suppressed* scope exit. The cleanup runs
on normal fall-through, on early `return`/`?`, and **while a panic unwinds** (`Drop` runs during unwinding)
— verified in `tests/linear_use.rs`. This guard is unwind-safe / leak-sensitive: the off-diagonal partner of
the ninth axis's `take` seam (unwind-sensitive / leak-safe).

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `MustRun` guard vends an *unconditional must-run finalizer*: the safe fragment binds the
> guard to a local, and the cleanup is guaranteed to run when the scope ends — on every non-suppressed
> path, with no `unsafe` and no hand-written finally-block — because `Drop` runs the closure on scope exit.
> An *unintended* forgotten finalizer is therefore **claimed to be prevented by the type**: bind it and it
> runs (and to *intentionally* cancel, call `disarm`, not `forget`).

Structural stoppers (claimed): the `Option<F>` one-shot prevents a double-run; `Drop` covers normal and
unwinding exits; `disarm` gives the conditional case a cancel that is not the attack; `#[must_use]` (under
`deny`) catches the accidental statement-position drop.

## 5. Threat model

In scope: any **safe** path on which a `MustRun` guard is created but its cleanup **never runs** *without an
explicit `disarm`* — most directly `core::mem::forget(guard)` and `ManuallyDrop::new(guard)` (both safe),
but also `Box::leak`, storing the guard in a leaked `Rc`/`Arc` cycle; any way to make the cleanup run
**twice**. Sanctioned (NOT an attack): `disarm()`, the explicit conditional cancel. A correction the panel
should hold the spec to: `let _ = MustRun::new(...)` is **not** a skip — it runs `Drop` *immediately* (the
wrong *time*, lint-caught), not zero times. Out of scope: ordering among *multiple* finalizers; async
cancellation / `.await`-point drops; `panic`-during-unwind process abort (the destructor genuinely cannot
run then — a floor fact, not a seam bug); and the *finalizer's own success/failure* — `FnOnce() -> ()` has
no error channel, so a finalizer that *runs but fails* (a failed `close`/`fsync`) cannot report it on
`?`/unwind paths; this is a recorded **expressiveness cost** (§6), and the primitive is scoped to infallible
finalizers.

## 6. The scoring rubric — soundness, then the FLOOR-GRADING deliverable

First, soundness *in the must-run sense* (the finalizer runs on every non-suppressed path that abandons the
scope):

- **Horn 1 — sound re-import.** Exhibit **safe** code that creates a `MustRun` guard whose cleanup never
  runs *without a `disarm`* (the `forget`/`ManuallyDrop`/leak skip), defeating the "bind it and it runs"
  promise. Or concede that on the affine floor no construction can compel the finalizer. *(Pre-conceded:
  `let g = MustRun::new(c); mem::forget(g);` runs it zero times.)*
- **Horn 2 — decidable & proof-free.** `new`/`run_now`/`disarm` are ordinary safe calls; `Drop` is
  automatic. Confirm.
- **Horn 3 — concentration.** The finalizer obligation is concentrated in the guard; the consumer writes no
  `finally` block and no `unsafe`. Confirm — *or* argue the caller still carries the obligation to *not
  forget* the guard (a per-use obligation the type cannot discharge).

Then **the floor-grading deliverable** — the point of the phase. Rule on each:

1. **The GRADE — confirm it is a clean NONE at the type level.** The r23 panel foreclosed "partial via
   escalatable lint" (the `scopeguard` `#[must_use]` is a cosmetic advisory over a non-guarantee, not a
   floor fragment). Confirm the floor-proper grade against must-run-cleanup is a **clean none**, and that
   stipulating "`must_use` ∈ floor" only manufactures a false partial — or re-open the texture if you
   disagree.
2. **The UNIT of the match.** Deadlock-freedom (#15/#16) is *also* a liveness precondition, graded
   *partial*. Must-run-cleanup is graded *none*. So the **same domain hosts two preconditions of different
   grades** — does this sharpen the unit of the floor-match from *domain* to *precondition* (refining the
   Phase-M "domain-parameterized" result)? Agree or refute.
3. **Axis bookkeeping (the deflation).** This is built **not** to claim a new axis: it is the *enforcement*
   direction of the **ninth axis** (leak-freedom), where the tenth seam `drain` populated the *tolerance*
   direction. Confirm the off-diagonal is genuine (drain *closes* leak-tolerant / this *breaks*
   leak-intolerant, same axis) and **N is unchanged** — or argue must-run-cleanup is a distinct coordinate.
4. **The VERDICT MODE — confirm `wrong-structural-logic / out-of-frame`, not `graded-match`.** The r23 panel
   ruled: closing the gap while keeping `forget` safe needs either an **undecidable** finalizer-reachability
   pass (`Rc`-cycle escape/may-alias) or a **linear** (not affine) type system — so *no decidable boundary
   price* buys soundness back. The obstruction is the floor being the wrong *structural logic* (affine ≠
   linear), not the wrong *domain* (the Phase-L IFC sense). Confirm this is a distinct verdict mode from the
   Phase-N graded-match, or collapse it. Prior art to weigh: the **leakpocalypse** (RFC 1066; `thread::scoped`
   removal, RFC 1084); `scopeguard`; linear types (Clean, Idris 1, Linear Haskell `%1`, Granule); Rust's
   `?Leak`/`Leak`, `must_move`, `must_not_suspend` proposals.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which axis) / **SEAM-BREAKS** (give the safe
cleanup-skipping sequence). And state, explicitly: **the affine floor earns a clean *none* against
must-run-cleanup; this is the same domain as deadlock-freedom but a *different precondition* with a
*different grade* (so the floor-match unit is the precondition, not the domain); it is the enforcement face
of axis 9 with N unchanged; and the verdict mode is the new *wrong-structural-logic / out-of-frame* (affine
vs linear), not graded-match** — confirming, refuting, or sharpening each. And confirm the r23 ergonomics
incoherence (conditional idiom vs unconditional precondition; disarm == the attack) is now resolved.
