# Seam spec — a constant-time secret (re-import rule under attack)

This is a **design proposal** for a thirteenth *seam* in the same hypothetical systems language (same
floor, same rules) as the twelve before it — but the first one deliberately aimed at a **different
soundness domain**. The previous twelve were all about **memory safety** (can safe code reach UB?), and
across them the cold panel found the cost-basis **closed**: nine clean axes + an inseparable category +
two outside-the-axes, with a brand-new memory-safety danger (validity) projecting cleanly onto it. This
seam asks whether that closure is **absolute** or **domain-relative**, by re-importing a danger that is
**not UB at all** — an **information-flow / timing side-channel**.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/secret.rs`).

> **Read §6 first if you only read one thing.** The deliverable is *not* "soundness vs UB" (there is no
> UB here) but the **domain question**: is information-flow a tenth axis of the memory-safety basis, is it
> *out of frame* (a different soundness domain with its **own** basis — refining "closed" to "closed *per
> domain*"), or does its cost project onto the existing nine?

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped, **plus two facts that are load-bearing here**: (a) the Rust
Abstract Machine models *functional* behaviour and UB, but **not wall-clock time** — a timing
side-channel is invisible to it; (b) `rustc`/LLVM make **no constant-time guarantee** — branchless source
may be lowered to a branch (`cmov`→branch, peephole), so constant-time-ness is *audited* (`dudect`,
`ctgrind`, `black_box`, asm), not *typed*. Do not attack the borrow checker or the optimizer; attack the
re-import rule and the §6 domain claim.

## 1. The primitive

**Data-dependent control flow over a secret.** The canonical instance is equality: a naive `a == b` over
secret byte arrays **short-circuits at the first mismatching byte**, so its *running time* leaks the
length of the matching prefix — a byte-by-byte key/MAC recovery attack. The danger is *not* memory
corruption (none occurs); it is a **timing leak of secret content**. The seam re-imports this as a
checked safe interface that only ever does **constant-time** (branchless, content-time-independent) work.

## 2. The true precondition (cathedral side)

A computation over a secret is **non-interfering** (timing-safe) iff **its running time is independent of
the secret's value**. For the comparison primitive: fold all bytes with no early exit; never branch,
index, or early-return on a secret-derived value. One conjunct, and it is about a *non-functional*
property (time), not about aliasing, lifetime, bounds, or bits.

## 3. The token — `Secret<N>` + `Choice`, and the controlled exits

`Secret<N>` keeps its bytes private and **does not implement** `PartialEq`/`Eq`/`Ord`/`Deref`/`Index`, so
safe code *cannot* branch/index on it (CF1). The vended operations are all constant-time: `ct_eq -> Choice`
(branchless XOR-fold), `conditional_select(a, b, choice)` (branchless masked merge). A `Choice` is a
constant-time boolean (`0`/`1` `u8`, never branched on) until `Choice::expose_bool()` — the **one
sanctioned exit** back into ordinary control flow, used once at the end on a result that is *meant* to be
observable (accept/reject), never mid-computation on the secret.

**The honest twist (load-bearing for §6).** (1) There is **no `unsafe` keyword** in the foundation — it
is plain arithmetic; the "danger" is purely a timing property the AM does not model. (2) The
constant-time guarantee is **not discharged by the floor** — the type system forecloses the *obvious*
leaks (source-level branching) but cannot stop the optimizer re-introducing a branch. So the seam rests
on a **trusted assumption** (the codegen stays branchless), exactly the shape of MMIO's TCB axiom — a
trusted `assume` boundary — but for a *security* guarantee rather than a hardware fact.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Secret`/`ct_eq`/`conditional_select` interface vends timing-safe operations on
> secrets. The safe fragment may use them with no `unsafe` and no proof because the type forecloses
> data-dependent branching (no `PartialEq`/`Index`/`Deref`) and the vended ops are branchless — so *no
> source-level timing dependence on the secret can be written*, modulo the trusted-codegen assumption.

Structural stoppers: no `PartialEq`/`Eq`/`Ord`/`Deref`/`Index` on `Secret` (CF1); bytes private;
`Choice` opaque except via the loudly-named `expose_bool`.

## 5. Threat model

In scope: any **information leak of a secret's content through timing** reachable by *safe* consumer code
using only §3 — a hidden branch, an early exit, a secret-indexed array access, `expose_bool` called
mid-computation rather than once at the end, a `conditional_select` whose mask leaks. Out of scope:
AM-level UB (there is none); the *audited* constant-time-ness of the foundation's `ct_eq`/`select` lowering
(that is the seam's TCB axiom — flag it if you think it is load-bearing, which it is); cache/branch-predictor
microarchitectural channels beyond source-level control flow (the type system cannot reach them — name
this as a scope limit).

## 6. The scoring rubric — soundness-in-this-domain, then the DOMAIN adjudication

First, soundness *in the information-flow sense* (the real attack surface):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that leaks a secret's content through timing
  despite the interface — a source-level data-dependent branch the type system failed to foreclose, an
  `expose_bool` misuse, a non-constant-time vended op. Or concede the interface forecloses *source-level*
  leaks (and name the TCB/microarch residue).
- **Horn 2 — decidable & proof-free.** `ct_eq`/`select` are ordinary safe calls; no solver, no proof.
  Confirm.
- **Horn 3 — concentration.** The trust is concentrated in two foundation functions; consumer crossings
  O(1). Confirm — *but note the trust is a TCB axiom, not a discharged check.*

Then **the domain adjudication** — the deliverable. The memory-safety cost-basis is closed; rule which
holds for information-flow:

1. **A tenth axis of the *same* basis?** Is "timing-safety / non-interference" just another coordinate
   alongside zero-cost, API-locality, bare-metal, …? (The builder doubts this: it is a *different kind of
   soundness* — non-interference, not UB — so it is not obviously commensurable with the memory-safety
   axes.)
2. **Out of frame — a *different domain* with its own basis (the builder's hypothesis)?** If
   information-flow is a separate soundness notion the memory-safety basis simply does not cover, then
   "the cost-basis is closed" is **closed *per domain***: re-running the build-and-break method over the
   IFC domain would map a *different* basis, whose first members look like **constant-time discipline**
   (the lost data-dependent control flow — an expressiveness-shaped cost *within IFC*) and a
   **no-data-dependent-branch TCB axiom** (the trusted-codegen assumption, MMIO-shaped). Does this seam
   support that reading? Is the closed-basis result thereby *refined, not refuted*?
3. **Projects onto the existing nine after all?** Could one argue the cost is just **expressiveness**
   (you lose `==`/branching — but expressiveness was the *confound* that sank `typed_nat`, not one of the
   nine clean axes) plus a **TCB axiom** (MMIO's category, outside the axes)? If so, IFC adds nothing new
   even cross-domain — the basis is closed *absolutely*. Is that the honest reading, or does it under-count
   constant-time discipline as a genuine IFC-domain axis?

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** / **SEAM-BREAKS** (in the information-flow sense —
give the leak sequence if it breaks). And state, explicitly, the **domain ruling**: is information-flow
**(1) a tenth axis of the memory-safety basis, (2) out of frame — a different domain with its own basis
(closure is per-domain), or (3) a projection onto the existing nine (closure is absolute)?** — and if (2),
sketch what the IFC-domain basis's first axes are, and whether the *method* (build-and-break, cold panel)
transfers to it. Prior art to weigh: `subtle`, `secrecy`/`zeroize`, the constant-time-programming and
information-flow / non-interference literature (FlowCaml, Jif, the IFC type-systems line).
