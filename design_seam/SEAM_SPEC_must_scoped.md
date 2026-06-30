# Seam spec — scoped session (does a *scoped-closure* combinator **concentrate** completion?) (re-import rule under attack)

This is a **design proposal** for a thirtieth *seam*, the **direct salvage of the twenty-ninth** (`must_complete`),
mirroring the #26→#27 break→salvage rhythm. #29 built a **free-handle** `Session<P>` with a `Drop` **completion
bomb** and over-claimed it forced run-to-`End`; the r36 panel **reversed** that: `core::mem::forget` skips the bomb
(the free handle is forgettable), **yet completion is NOT irreducible** — a **scoped-closure / CPS combinator** (the
`std::thread::scope` analogue) compels it *modulo divergence*, by moving the obligation to the **re-import boundary**
so the consumer **never holds a forgettable handle**. This seam builds that combinator concretely and asks whether
the #29 reversal **closes**. It is **NOT a fourth-domain bid**; it stays inside the **third confirmed domain
(liveness)**, on the **completion** coordinate.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/must_scoped.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, self-contained over the #23/#29 two-endpoint model). Tests pass
(default), including two `compile_fail` doctests (CF1 the forcing, CF2 generativity).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing + private fields; trait resolution (`HasDual`); higher-ranked
trait bounds (`for<'a> FnOnce(..) -> ..`) and lifetime **generativity** (an invariant brand `'scope` that two
distinct calls never unify — the `branded` #5 / `GhostCell` mechanism, the same one `std::thread::scope` uses).
**The leakpocalypse fact (still true, and now deliberately defanged):** `core::mem::forget` is a **safe** `fn<T>(T)`
that skips `Drop` (RFC 1066; the removal of `thread::scoped`, RFC 1084) — but **this seam does not rely on any
destructor**, so `forget` skips nothing the forcing depends on. The transport's byte/queue moving is the trusted
core (out of scope, like #23/#29's). Do not attack `Rc`/`RefCell`/`VecDeque` or the transport, or the generativity
soundness of the brand (it is the established `indexing`/`GhostCell` trick). Attack the **scoped re-import rule** and
the §6 deliverable: *is completion really concentrated, or is there a safe escape?*

## 1. The primitive

A **session you can only run inside a scope**, plus a **completion certificate**. There is **no public `start`**.
The sole way to obtain a [`Session`]`<'scope, P>` is [`session_scope`]`(a, b)`, which opens one session over an
`Rc<RefCell<Transport>>` and hands each of two **dual** role closures a freshly brand-`'scope`-tagged endpoint,
**requiring each closure to return a [`Done`]`<'scope>`**. `send`/`recv` walk the protocol (affine typestate; an op
exists only at its matching state). [`Session::finish`] — available **only** at `End` — is the **only** constructor
of `Done`: it marks the transport completed and mints the certificate. There is **no `Drop` bomb** (unlike #29): the
completion obligation is carried by the **return type** of the role closure, not by a destructor.

## 2. The true precondition (cathedral side)

- **Order:** `send` only at `Tx`, `recv` only at `Rx`, `finish` only at `End` (affine typestate, as #18/#23/#29).
- **Completion (the thing in question):** every session a scope opens runs to `End` and is `finish`-ed — so the peer
  is never left waiting on a value (or close) that never comes.

## 3. The witness — a scoped closure that must return a branded `Done`

Order is carried for free by the affine typestate (an out-of-state op is a type error; the handle is `!Copy`).
For **completion**, the witness is the **scoped-closure obligation**: the role closure is `FnOnce(Session<'scope, P>)
-> Done<'scope>`, and `Done` is mintable **only** by `finish` at `End`. So a closure has exactly three fates:
(1) **complete** — finish, return the `Done`; (2) **diverge** — `loop`/`panic`/block, never return; (3) **nothing** —
it cannot return without a `Done` and cannot fabricate one (`mem::forget` mints no certificate; the invariant brand
blocks smuggling a foreign `Done` from another scope). The session is **trapped in the closure** (the `for<'scope>`
binder forbids it escaping), which is the cost.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** `session_scope(a, b)` runs two dual role closures, each handed a brand-`'scope` `Session` and each
> obligated to return a `Done<'scope>`. Safe code composes `send`/`recv`/`finish` with no `unsafe`. The type system
> guarantees, **exactly**:
> 1. **Order** — an out-of-state op is a type error.
> 2. **Must complete (modulo divergence)** — a role closure **cannot return normally without having run its session
>    to `End` and `finish`-ed it**, because `Done` has private fields and `finish` at `End` is its only mint; and it
>    cannot smuggle in a `Done` from elsewhere (invariant brand) nor fabricate one by `mem::forget` (no destructor
>    is involved). The only ways out are *complete* or *diverge*; there is **no safe silent abandonment**.
>
> Structural stoppers (claimed): no public `Session`/`Done`/`start` constructor (private fields); the `for<'scope>`
> binder traps the endpoint and the certificate in the closure (generativity, branded #5); the return-type
> obligation replaces #29's forgettable `Drop` bomb with an unforgeable proof token.

## 5. Threat model

In scope: any **safe** path that (a) performs an op out of state and type-checks; (b) makes a role closure **return
normally without completing its session** — i.e. *fabricate, forge, or smuggle* a `Done<'scope>` without `finish`
(via `mem::forget`, `ManuallyDrop`, a foreign-scope `Done`, transmute-free trickery, `Default`, etc.); (c) lets the
endpoint **escape** the scope (store it, return it, stash it in the captured environment) to be completed-or-not out
of band; (d) argues the "modulo divergence" caveat is a fatal hole — that a *diverging* body re-opens the same NONE
#29 had, so nothing was actually gained. Out of scope: `Rc`/`RefCell`/`VecDeque` internals; the trusted transport;
the soundness of generativity itself (the established `indexing`/`GhostCell` brand); async/threaded scheduling
(single-threaded run-`a`-then-`b` witness, as #23–#29); recursion (no `Rec`/`Var` — fixed-depth witness, as the arc).

## 6. The scoring rubric — soundness, then the completion-concentration deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the worst case is a **hung/leaked** (memory-safe) session, not UB.
  Confirm. (Any break here would be a **liveness** break — the *advertised* forcing is false in safe code — not UB.)
- **Horn 2 — decidable & proof-free.** The typestate is structural (Θ(depth)); the brand is resolved by the
  compiler's HRTB machinery (no SMT, no use-site proof). Confirm.
- **Horn 3 — concentration.** Order concentrates in the typestate. **Completion** is the question: is it now carried
  by the scoped combinator (at an API-locality cost — the session trapped in the closure), where #29's free handle
  left it on the caller? This is the seam's headline claim.

Then **the completion-concentration deliverable** — the point of the phase. Rule on each:

1. **Does the scoped combinator REALLY force completion, or is there a safe escape?** The hypothesis: it forces
   completion **modulo divergence** — a closure cannot return without a `finish`-minted `Done`, and no safe path
   fabricates/forges/smuggles one. Confirm — or exhibit a **safe** closure that returns normally with its session
   **not** run to `End` (a forged/defaulted/foreign `Done`, an escape that defers completion, etc.).
2. **Is "modulo divergence" the HONEST boundary, or a fatal hole?** The claim: honest — a diverging body (`loop {}`)
   hangs *observably*, whereas #29's `mem::forget` terminated *silently*; the residue shrinks from **silent
   abandonment** to **non-termination**, it does not vanish, and that is a genuine improvement (a hang is
   detectable; a silent starve is not). Confirm — or argue divergence makes the forcing **vacuous** (equivalent to
   #29's NONE), so nothing was gained.
3. **Is this a CLEAN salvage of #29 (break→cost), like #27 salvaged #26?** The claim: yes — #29's free-handle bomb
   *factored* (forgettable); the scoped witness *closes* completion at a horn-3 API-locality cost, so the #17–#28
   completion residue was the price of the **free-handle API**, recoverable by surrendering the free handle.
   Confirm — or argue the salvage is **partial** (e.g. it only forces *this* endpoint, so global completion still
   escapes — the #23/#27 per-session-vs-global gap recurs), and say what stays uncarried.
4. **Prior-art deflation (pre-registered).** `std::thread::scope` (stable 1.63, the fix for the removed
   `thread::scoped`/RFC 1084); generativity / branded indices (`indexing`, `GhostCell`, Yallop–White); CPS / linear
   continuations; affine-session cancellation work (Mostrous–Vasconcelos *Affine Sessions*; ECOOP 2022 *Stay Safe
   Under Panic*; MultiCrusty; Exceptional GV). Confirm the contribution is **locating** session-completion as
   *concentrable at an API-locality cost* (the concrete witness closing the #29 reversal), **not** a new mechanism —
   it is `thread::scope` + branded #5 + #29's typestate, reused.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does the scoped combinator force completion modulo divergence (no safe closure
returns with an unfinished session), or is there a safe escape (a forged/smuggled `Done`, an endpoint that escapes);
is "modulo divergence" the honest boundary (silent abandonment → observable hang) or a vacuous hole; is this a clean
break→cost salvage of #29 (completion concentrable at horn-3 API-locality cost, the #17–#28 residue being the price
of the free-handle API) or only a partial one (per-endpoint, global completion still escaping); and does it deflate
onto `thread::scope` / generativity (branded #5) / #29's typestate as a re-application rather than a new mechanism —
with no new domain, coordinate, or mode (N = 9; the FAILED broken-seam tally stays 8 if it HOLDS, moves to 9 only if
a safe escape breaks the advertised forcing)?**
