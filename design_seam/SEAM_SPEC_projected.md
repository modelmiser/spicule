# Seam spec — projected sessions (deadlock-freedom by construction) (re-import rule under attack)

This is a **design proposal** for a twenty-seventh *seam* — the **salvage of the twenty-sixth** (`priority`),
which broke. `REPORT_SEAM_SPEC_priority.md` (r33) ruled SEAM-BREAKS [horn 1]: the per-role constructor
`role::<Pi, Po>` was **public**, so a caller wired three locally-valid `role::<N1, N2>` relays into a source-less
ring that compiled (no `unsafe`) and deadlocked. The verdict named the real carrier of deadlock-freedom: the
**trusted, edge-consistent constructor** (`relay()` was a "mini-projection"), not the per-channel `Lt` discipline.
This seam makes that explicit by **sealing** the per-role constructor and vending only trusted builders.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/projected.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, built on `binary_session` #23 and `typed_nat` #8). 154 tests
pass (default).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; **module privacy** (a non-`pub` `fn` / private fields are unreachable from
other modules — this is the load-bearing capability); const trait-bound resolution (`Pi: Lt<Po>` via #8's sealed
`typed_nat::Lt`, Peano `<`, no reflexive/decreasing instance). The binary edges' transport is #23's trusted core
(out of scope). Do not attack `typed_nat`/privacy/the transport. Attack the **projected re-import rule** (the
deadlock-freedom-by-construction claim) and the §6 deliverable.

## 1. The primitive

A **projected session**: the #26 roles (`Source`, `Sink`, `Worker<Pi, Po>` with `Pi: Lt<Po>`) but with the
per-role constructor `worker()` **module-private**. The only public way to obtain roles is a **trusted builder**
(`relay()` → 3-party `A→B→C`; `relay4()` → 4-party `A→B→C→D`) that threads **one strictly-increasing `Nat` per
edge** (`N0 < N1 < N2 < …`). The danger addressed is a **liveness** hazard (deadlock), not UB.

## 2. The true precondition (cathedral side)

- **Role-local order + priority monotonicity:** as #26 (`Pi: Lt<Po>` per role).
- **Edge consistency:** the `Nat` a role *sends* on an edge equals the `Nat` its peer *receives* on that edge.
- **Global acyclicity (deadlock-freedom):** the assembly has a source and a strictly-increasing priority order,
  hence a terminating schedule.

## 3. The witness — a sealed constructor + monotone builders

`worker()` is module-private (the #26 fix), so no caller can mint an arbitrary role. The builders thread
strictly-increasing, edge-consistent priorities, so every assembly a caller can obtain is acyclic. Two
mechanisms compound: **access** (no public `worker()`; the #26 exploit is uncompilable — see §4) and **structure**
(a monotone chain cannot close into a ring — the closing edge needs a *decreasing* `Lt` step, which has no
instance, so even the foundation author cannot write a deadlocking builder).

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A projected session vends only **trusted builders** (`relay`, `relay4`) that mint
> deadlock-free assemblies. Safe code composes the resulting roles' `recv`/`send`/`close` with no `unsafe`. The
> rule guarantees:
> 1. **Role-local order + monotone priority** — as #26.
> 2. **Deadlock-freedom by construction** — every assembly a safe caller can obtain has a source and a
>    strictly-increasing priority order; the #26 deadlocking ring is **unconstructible** (no public per-role
>    constructor; a monotone chain cannot close).
>
> The rule **concentrates deadlock-freedom entirely in the trusted constructor** (the mini-projection #26's
> break named): the per-channel `Lt` discipline carries nothing global. The **cost** is *expressiveness*: only
> **acyclic** topologies (pipelines, trees) are expressible, each needing a foundation-authored builder;
> genuinely-deadlock-free **cyclic** protocols (Kobayashi-style) are categorically excluded.

Structural stoppers (claimed): `worker()` private ⇒ no caller-minted roles; monotone `Lt` chain ⇒ no cycle;
`recv` exists only at `Pi: Lt<Po>`.

## 5. Threat model

In scope: any **safe** path that (a) mints a `Worker` outside the trusted builders (defeating the seal — e.g. a
privacy-bypass, a `pub` leak, a `Default`/`Clone`/transmute-free reconstruction); (b) wires a deadlocking ring
from the trusted builders' outputs (re-routing `relay()`'s roles onto cyclic edges); (c) shows a trusted builder
itself admits a cycle; (d) argues the seam is vacuous ("the absence of the dangerous feature, not a re-import
rule"). Out of scope: `typed_nat`/privacy/the transport; async/threaded scheduling (single-threaded witness, as
#23–#26); implementing a *general* projection from an arbitrary global type (only fixed `relay`/`relay4` builders
are vended — the generality cost is the point).

## 6. The scoring rubric — soundness, then the salvage deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the #26 deadlocking ring is now **unconstructible** from safe code;
  the worst case is a memory-safe assembly that completes. Confirm — or exhibit a safe path that mints a rogue
  `Worker` or re-wires the builders' roles into a deadlock (the #26 exploit, resurrected).
- **Horn 2 — decidable & proof-free.** Builders + `Pi: Lt<Po>` stay structural (Θ(depth), no SMT). Confirm.
- **Horn 3 — concentration.** Deadlock-freedom is concentrated in the trusted constructor. Confirm — and note
  the consumer writes no proofs, but the *foundation author* carries the whole-program acyclicity per builder.

Then **the salvage deliverable** — the point of the phase. Rule on each:

1. **Does sealing `worker()` actually HOLD (resolve the #26 break)?** The hypothesis: yes — the ring is
   unconstructible (access) and a monotone chain cannot close (structure). Confirm or refute (is there a residual
   safe path to a deadlock — re-routing builder outputs, a privacy leak, a `Worker` minted via some other public
   surface?).
2. **SEAM-COSTS [horn 3] or SEAM-HOLDS — and is the seam VACUOUS?** Deadlock-freedom now lives entirely in the
   trusted constructor (a mini-projection in the TCB). Is that a legitimate horn-3 concentration (the foundation
   author carries the danger, exactly as `split`/`spsc` do), or is the seam *vacuous* — "you removed the
   dangerous feature (arbitrary topologies) rather than re-importing it safely"? Where is the honest line?
3. **What is the cost, and does the trilemma bite?** The pre-conceded cost: only **acyclic** topologies are
   expressible; deadlock-free **cyclic** protocols are excluded; each topology needs a foundation-authored
   builder (no general user-supplied projection). Is this the trilemma's *expressiveness* corner (sound +
   decidable + concentrated, but not general), or is the generality recoverable (a generic acyclic builder, a
   type-level topological-order witness) without breaking soundness?
4. **Does this CONFIRM the #24–26 arc and stay the THIRD domain (N stays 9)?** The arc: progress opened (#24),
   global topology (#25), per-channel discipline alone broke (#26), trusted constructor holds (#27). Confirm
   global progress's only sound home is a trusted whole-program constructor; N stays 9; the FAILED broken-seam
   tally stays 8 (this HOLDS) — or argue otherwise.
5. **Prior-art deflation (pre-registered).** This is exactly how real multiparty systems work: Scribble / the
   `mpstthree` Rust crate generate the per-role endpoints from a global type via a trusted tool — deadlock-free
   *by construction of the generated code*, never by checking arbitrary compositions. Confirm the contribution
   is **locating** that the type discipline alone cannot carry global progress (the #26 break) and that the
   trusted generator is the sound resolution (this seam), at the expressiveness cost.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does sealing `worker()` resolve the #26 break (no safe deadlock); is the
verdict SEAM-COSTS [horn 3] or SEAM-HOLDS, and is the seam vacuous (feature-removal vs re-import); is the
expressiveness cost (acyclic-only, per-topology builders) the trilemma biting or is generality recoverable; does
this confirm the #24–26 arc that global progress's only sound home is a trusted whole-program constructor with
N = 9 and the FAILED tally at 8; and does it deflate onto Scribble / `mpstthree` (deadlock-free by construction
of generated code)?**
