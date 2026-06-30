# Seam spec — must-complete session (can the affine floor *compel* run-to-`End`?) (re-import rule under attack)

This is a **design proposal** for a twenty-ninth *seam*. The session-type connective set is **exhausted** (#18/#23
order, #24 delegation, #25 multiparty, #26 priority, #27 projected, #28 choice); every one HOLDS or BREAKS with the
**same** leftover — a **completion residue** the affine floor never carries, which the proposer over-claimed as
carried **five** times (#8/#24/#25/#27, and the "handle-all" leg of #28). This seam stops probing features and
attacks the residue itself: it builds the discipline that **tries to force** a session to reach `End`, and asks
whether the floor has **any** way — even a cost-paying abort — to compel it. It is **NOT a fourth-domain bid**; it
stays inside the **third confirmed domain (liveness)**, on the **completion** coordinate.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/must_complete.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, self-contained over the #23 two-endpoint model). 158 tests pass
(default).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing + private fields; trait resolution (`HasDual`); `Rc`/`RefCell`
with their runtime borrow checks. **The leakpocalypse fact (load-bearing here):** `core::mem::forget` is a **safe**
`fn<T>(T)` that consumes a value by one legal affine move and **skips `Drop`** (RFC 1066; the removal of
`thread::scoped`, RFC 1084); `ManuallyDrop`, `Box::leak`, and `Rc`-reference-cycles skip it the same way. `Drop`
**does** run on ordinary scope exit, `?`-bail, and unwind — but it is **not guaranteed to run**. The transport's
byte-moving is the trusted core (out of scope, like #23's). Do not attack `Rc`/`RefCell` or the transport. Attack
the **must-complete re-import rule** and the §6 deliverable.

## 1. The primitive

A **session handle with a completion bomb**. Over #23's `Rc<RefCell<Transport>>` dual endpoints, [`start`] vends two
dual [`Session`]`<P>` handles. `send`/`recv` walk the protocol (affine typestate: an op exists only at its matching
state). [`Session::finish`] — available **only** at `End` — is the **one clean exit**, defusing the bomb. Each
`Session` carries a runtime `live` flag and a `Drop` **completion bomb** (cf. `take` #9's abort-bomb): advancing
hands `live` to the successor, `finish` clears it, but if a `live` handle is **dropped** before `End` (scope exit,
`?`, panic) the bomb fires — a `panic!` standing for `process::abort`. The danger addressed is the **completion**
half of liveness: a peer **starved** by an abandoned session, not UB. This is the **remote** analogue of `linear`
#20's **local** must-run-cleanup — there `mem::forget` skipped your own cleanup; here it skips a `Drop` that was the
**peer's** only guarantee of being answered.

## 2. The true precondition (cathedral side)

- **Order:** `send` only at `Tx`, `recv` only at `Rx`, `finish` only at `End` (affine typestate, as #18/#23).
- **Completion (the thing in question):** every started `Session` runs to `End` and is `finish`-ed — so the peer is
  never left waiting on a value (or a close) that never comes.

## 3. The witness — affine handle + a `Drop` abort-bomb

The affine typestate carries **order** for free (an out-of-state op is a type error; the handle is `!Copy`/`!Clone`).
For **completion**, the witness is the `Drop` bomb: since a `live` session that is dropped aborts, the *only* way
past a `Session` is to advance or `finish` it — so (the claim goes) you **cannot walk away** from a half-run session.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `start`-ed session vends `send`/`recv`/`finish`. Safe code composes them with no `unsafe`. The
> type system + the `Drop` bomb guarantee, **exactly**:
> 1. **Order** — an out-of-state op is a type error.
> 2. **Must complete** — a `Session` **cannot be abandoned before `End`**: dropping a `live` handle fires the bomb
>    (an abort), and the only non-aborting exit is `finish` at `End`. So every session the program *starts* it must
>    *finish*, and no peer is starved.
>
> Structural stoppers (claimed): nominal/private typing forbids forging a `Session` at a wrong state; the affine
> move forbids using a handle after advancing; the `Drop` bomb converts "abandon mid-session" into a hard abort.

## 5. Threat model

In scope: any **safe** path that (a) performs an op out of state and type-checks; (b) **abandons a `live` session
before `End` without aborting and without completing it** — i.e. makes the bomb **not fire** while the peer is left
starved (`core::mem::forget`, `ManuallyDrop::new` + never `into_inner`, `Box::leak`, an `Rc`-reference cycle); (c)
defeats the bomb's abort even when `Drop` *does* run (e.g. `std::panic::catch_unwind` swallowing the panic, turning
the abort into a recoverable no-op). Out of scope: `Rc`/`RefCell` internals; the trusted transport; async/threaded
scheduling (single-threaded witness, as #23–#28); recursion (no `Rec`/`Var` — a fixed-depth witness, as the arc).

## 6. The scoring rubric — soundness, then the must-complete deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the worst case is a **starved/leaked** (memory-safe) session, not UB.
  Confirm. (This seam's break, if any, is a **liveness** break — the *advertised* guarantee is false in safe code —
  not a memory-safety UB, exactly as `linear` #20 and the IFC/lock seams broke.)
- **Horn 2 — decidable & proof-free.** The typestate is structural (Θ(depth), no SMT). Confirm.
- **Horn 3 — concentration.** Order concentrates in the typestate. The **completion** obligation is the question —
  is it carried by the `Drop` bomb, or does it stay on the caller?

Then **the must-complete deliverable** — the point of the phase. Rule on each:

1. **Does the floor COMPEL completion, or is "must complete" a clean NONE?** The hypothesis: a clean **NONE** —
   `core::mem::forget(session)` is **safe**, consumes the handle by one affine move, and **skips the bomb**, so the
   session is abandoned and the peer starved with no abort, no `unsafe`. "Runs to `End`" is a **linear**
   requirement; Rust's floor is **affine** (*at most* once). Confirm — or exhibit a sound discipline on this floor
   that genuinely forces completion (and say what it costs).
2. **Does the PEER / two-endpoint dimension change the break vs `linear` #20?** The claim: **no** — `mem::forget`
   skips `Drop` whether the un-run obligation is *local* (#20's own cleanup) or *remote* (this peer's answer); the
   second endpoint adds a *victim*, not a *fix*. Confirm — or argue the two-endpoint structure admits a completion
   *certificate* (a handshake the peer can require) that the single-handle #20 could not, partially forcing it.
3. **Can the ABORT-BOMB (the #9 `take` discharge) force completion?** The claim: **no** — `forget` skips `Drop`
   *before* the bomb can fire, so the abort (which #9 used soundly for unwind-safety) is bypassed here; and where
   `Drop` *does* run, `catch_unwind` can swallow the panic. So even paying a non-recoverable abort does not buy
   completion. Confirm — or show an abort discharge `forget` cannot defeat.
4. **Is this the SESSION FACE of `linear` #20's break, and does it UNIFY the arc?** The claim: yes — must-complete
   is `linear` #20 one structural step over (closure-cleanup → session-completion), the same *wrong-structural-logic*
   (affine vs linear) NONE; and it **unifies** the whole session arc's completion residue (#17/#18/#23/#27/#28) as
   **one** irreducible linear gap — there is **no** discipline, not even a cost-paying abort, that compels a session
   to `End` on an affine floor. No new domain (liveness), coordinate (completion, the #17 residue), or mode
   (SEAM-BREAKS is one of the nine). Confirm — or argue this is a mere **re-run** of #20 that adds nothing (a
   *deflation/redundancy* ruling), or that completion is categorically new here.
5. **Prior-art deflation (pre-registered).** The leakpocalypse (RFC 1066; `thread::scoped`/RFC 1084); linear vs
   affine types (Walker; the rejected `?Move`/linear-types RFCs); `session_types`/Jespersen–Munksgaard–Larsen 2015
   ("does not resolve the conflict between affine Rust and linear session types"). Confirm the contribution is
   **locating** session-completion on the lattice as the session face of the must-run break, not a new mechanism.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does the affine floor compel a session to run to `End` or is must-complete a
clean NONE (the safe `mem::forget` bypass); does the peer / two-endpoint dimension change the break vs `linear` #20;
can the abort-bomb force completion or does `forget` bypass it; is this the session face of #20's must-run break,
unifying the #17/#18/#23/#27/#28 completion residue as one irreducible linear gap (affine ≠ linear), with no new
domain, coordinate, or mode (N = 9; the FAILED broken-seam tally moves 8 → 9 if it breaks); and does it deflate onto
the leakpocalypse / linear-vs-affine folklore — or is it a mere redundant re-run of #20?**
