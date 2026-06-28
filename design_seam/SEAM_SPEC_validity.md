# Seam spec — a checked bit-pattern (re-import rule under attack)

This is a **design proposal** for a twelfth *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, slab, branded-index, refined-index,
shared-ownership, type-level-index, take-and-replace, drain-and-yield, and self-ref-cell seams. A seam
re-exports a dangerous primitive as a checked safe interface ordinary code uses with no `unsafe` and no
proofs. Attack the re-export: can ordinary safe code reach undefined behavior (UB)? **And** adjudicate
the §6 question — which, this time, is explicitly a **meta-question about the lattice itself**.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/validity.rs`).

> **Context, and why this primitive is different.** The lattice has **nine** confirmed clean axes plus
> one *real-but-structurally-inseparable* danger (move-stability). The builder's growing suspicion is
> that the **cost-basis is closed** — that new dangers no longer add coordinates but *project onto the
> existing nine* (or land in the inseparable category). This primitive is built to **test that
> hypothesis**, not to hunt a fresh axis. The §6 deliverable is the adjudication of the closed-basis
> claim, with this seam as the instrument.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL, affinity, `Drop`, `Send`/`Sync`, trait resolution), and
its **validity model**: each type has *validity invariants* (a `bool` is `0`/`1`; a `char` is a Unicode
scalar value; a `#[repr(uN)]` fieldless enum is one of its discriminants), and producing a value with an
invalid bit pattern is **instant UB** — the compiler may *assume* validity (exhaustive `match` without a
fallthrough, niche/discriminant layout optimizations). Safe code cannot produce an invalid value;
`transmute`/`from raw bytes` can, and is `unsafe`. Do not attack the validity model; attack the
re-import rule and the closed-basis claim.

## 1. The primitive

**Reinterpreting raw bytes as a typed value with a validity invariant.** Concretely:
`transmute::<u8, Tag>(b)` where `Tag` is a `#[repr(u8)]` enum with **non-contiguous** discriminants
`{A=0, B=5, C=200}`. For `b ∈ {0,5,200}` it is a valid `Tag`; for any other `b` it is **UB the moment it
exists** (no dereference needed). The discriminants are deliberately non-contiguous because the witness
must be **set-membership, not a range** (a range `b <= 200` would launder bytes `3,4,6..=199` — the
defect the first gauntlet on this spec correctly caught). The danger is the **bit-level validity** of a
value — a danger class no prior seam touches (not aliasing, bounds, ownership, cost, destructor-timing,
or address).

## 2. The true precondition (cathedral side)

`transmute::<u8, Tag>(b)` is sound iff **`b` is a valid discriminant** — i.e. `b ∈ {0,5,200}`, exact
set membership (*not* a range: bytes `1,2,3,4,6..=199,201..` are all invalid). One conjunct, and it is
entirely about the *value's bits*, not about aliasing, lifetime, or any caller-owned state.

## 3. The token — a checked read

The seam reads a `Tag` out of a byte buffer via `TaggedBuf::tag_at(i) -> Option<Tag>`: it loads the
byte, checks **membership in the exhaustive set of valid discriminants** (`b ∈ {0,5,200}`, via
`VALID_DISCRIMINANTS.contains(&b)`), and only then `transmute`s; a non-member byte returns `None`. The
unsafe (`transmute`) is gated behind the runtime set-membership check. There is no brand, no closure, no
lifetime gymnastics — the witness is simply **a runtime test that the bits are a valid discriminant
before the unsafe reinterpretation**. (A range test is *only* equivalent for contiguous-from-zero enums;
the general witness is set membership.)

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `tag_at` interface vends validity-checked reads of a raw byte buffer. The safe
> fragment may call it with no `unsafe` and no proof because the **set-membership check** (`b ∈ the
> exhaustive valid-discriminant set`) establishes the validity invariant before the `transmute`, so no
> invalid `Tag` is ever produced.

Structural stoppers: the `transmute` is `unsafe` (safe code cannot reach it — CF1); the fields/variants
are fixed by `#[repr(u8)]`; the check is **exhaustive discriminant-set membership** pinned to the actual
variant set (*not* a range — a range coincides with the set only for contiguous-from-zero enums, and is
unsound otherwise; this is the correction the first gauntlet forced).

## 5. Threat model

In scope: any AM-level UB reachable by **safe** consumer code. Consider: (a) any path that yields a
`Tag` from a byte **not in `{0,5,200}`** (a bypassed check, a `VALID_DISCRIMINANTS` set that is missing a
real variant or — the first gauntlet's defect — a *range* substituted for the set, a niche-layout
surprise); (b) reading a `Tag` from a partially-initialized or aliased buffer (does the buffer's own
safety cover it?); (c) the `byte_at` vs `tag_at` distinction (is reading raw `u8` ever unsound? — no,
`u8` has no validity invariant); (d) enums with explicit/non-contiguous discriminants or `#[repr(C)]`
subtleties (the witness must stay exhaustive set-membership over the *actual* variants). Out of scope: bugs inside
the foundation `unsafe` (`transmute`); the sizes/layouts the `#[repr(u8)]` guarantees (the floor's job).

## 6. The scoring rubric — soundness, then the CLOSED-BASIS adjudication

First, soundness:

- **Horn 1 — sound re-import.** Exhibit safe consumer code that obtains a `Tag` with an invalid bit
  pattern (or otherwise hits validity UB). Or concede the **set-membership-before-`transmute`**
  discipline (`b ∈ {0,5,200}`) forecloses it.
- **Horn 2 — decidable & proof-free.** `tag_at` is an ordinary safe call. Confirm.
- **Horn 3 — concentration.** One `unsafe` foundation region; O(1) consumer crossings. Confirm.

Then **the closed-basis adjudication** — this is the deliverable. Validity is a **genuinely distinct
danger** (no axis so far concerns a value's bit-level validity). Rule which holds:

1. **A new (tenth) axis?** Validity is its own coordinate, distinct from zero-cost (#6) and the rest.
   The builder doubts this — see (2).
2. **It projects onto an existing axis (the closed-basis outcome the builder predicts)?** The sound
   witness is a **runtime validity check** (set membership `b ∈ {0,5,200}`) per read — which is *exactly*
   the **zero-cost (#6)** cost the slab pays, via a different mechanism (a discriminant set vs a generation tag). If
   so, validity is a **new danger that spends an old axis** — the *dual* of the `get_unchecked` family
   (where one danger had many witnesses spending different axes; here many dangers converge on one
   axis). That would be evidence the **cost-basis is closed**: dangers project onto the nine axes rather
   than extending them. Which axis does it project onto — zero-cost, or something else (could a
   compile-time validity witness, e.g. a refinement or a sealed constructor, instead spend
   API-locality, making validity *also* non-uniform/inseparable like move-stability)?
3. **Inseparable?** If validity's cost is non-uniform across witnesses (runtime check ⇒ zero-cost;
   compile-time proof ⇒ API-locality) with no isolable coordinate, it lands in the move-stability
   category.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which axis) / **SEAM-BREAKS** (give the UB
sequence). And state, explicitly, the **closed-basis ruling**: does validity **(1) add a tenth axis,
(2) project onto an existing axis** (say which — the builder predicts zero-cost #6), **or (3)
inseparable**? And — the meta-payload — **is the evidence consistent with the cost-basis being
*closed*** (new dangers project onto the nine axes rather than extending them), or does this primitive
*extend* it? If closed, that is itself the result: the lattice has a small fixed basis, and "what does
the safe witness cost?" has only finitely many answers.
