# Seam spec — intrusive doubly-linked list (re-import rule under attack)

This is a **design proposal** for a third *seam* in the same hypothetical systems language
(same floor, same rules) as the SPSC and MMIO seams. A seam re-exports a dangerous primitive
as a checked safe interface ordinary code uses with no `unsafe` and no proofs. Attack the
re-export: can ordinary safe code, using only this interface, reach undefined behavior (UB)?

You are attacking a **text specification**, not source code.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity,
`Drop`, `Send`/`Sync`, trait resolution). Assume it is sound for the single-threaded safe
fragment. **Do not attack the borrow checker.** Attack only what the seam adds: the node and
list types, the link operations, and the re-import rule.

## 1. The primitive

An **intrusive doubly-linked list**: the `next`/`prev` link pointers live *inside* each
element node (not in separate heap cells the list allocates). Nodes are allocated and owned
*elsewhere* (an arena, a slab, the stack, a pool) and *threaded onto* the list by pointer.
This is the standard kernel/embedded data structure (Linux `list_head`, `intrusive-collections`)
chosen because it allocates nothing and an element can be O(1)-unlinked without searching.

## 2. The true precondition (cathedral side — rich, undecidable, and *escaping*)

Memory-safe operation requires **all** of:

- **L1 (single membership).** Each node is on **at most one** list at a time, linked at most
  once. Linking an already-linked node corrupts both lists.
- **L2 (no dangling links).** A node's backing storage must outlive its membership: if the
  node is freed (or moved) while still linked, its neighbours' pointers dangle ⇒ UAF on the
  next traversal. The list does **not** own the nodes, so this is a cross-ownership lifetime
  relation between the list and whoever owns each node.
- **L3 (link-graph coherence).** `a.next == b ⟺ b.prev == a` for every adjacent pair, and the
  list is acyclic from head to tail. Every insert/remove must restore this whole-structure
  invariant atomically w.r.t. observers.
- **L4 (no aliasing `&mut`).** Traversal hands out references to nodes; two `&mut` to the same
  node (e.g. a cursor plus an outstanding handle) is instant UB.

**Why this does not encapsulate.** Unlike SPSC (whose hard conjuncts P1–P3 stay *inside* the
`unsafe` block with no caller-visible residue), L1/L2/L4 are **relations between the list and
nodes the caller still owns and names**. The caller holds the nodes; the caller decides when
to free/move them; the caller asks for `&mut` access during traversal. So the invariant is a
*whole-heap aliasing-and-provenance* fact spanning the boundary — exactly the predicate that
lives only in an undecidable, certificate-free concurrent separation logic (RustBelt-grade).

## 3. The token — and the problem

The seam wants to emit affine handles, as SPSC did. But consider what a sound interface needs:

- To **insert** a node, the list must take ownership (or a pinned exclusive borrow) of that
  node's link fields for the whole membership — but the caller still owns the node's *data*.
- To **traverse and mutate**, a cursor yields `&mut Node`, and the type system must guarantee
  no second reference to that node exists anywhere — across an arena the compiler can't see.

The candidate sound mechanisms (all real):

- **(a) `GhostCell`/`ghost-cell` branding.** A `GhostToken<'brand>` is threaded through *every*
  access; the list and all its nodes share one `'brand`, and mutation requires presenting
  `&mut GhostToken<'brand>`. Sound and proof-free at the SMT level — but the **token is a
  parameter the consumer must carry through ordinary code** at every access site.
- **(b) `intrusive-collections`-style `unsafe` adapters.** The consumer (or adapter author)
  writes `unsafe impl` and upholds L1/L2 *by hand*; the obligation lands in caller-adjacent code.
- **(c) Pin + a runtime-checked "linked?" flag per node.** Adds a per-node bit and runtime
  branches; turns some L1 violations into panics, not all into compile errors; `Pin` discipline
  leaks into every node-owning site.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import (attempted).** A `List<'brand>` and its `Node<'brand>`s are certificates that all
> nodes sharing `'brand` obey L1–L4; the safe fragment may type `insert`/`remove`/`cursor` as
> safe methods **provided the caller presents the matching `&mut GhostToken<'brand>`** at each
> mutating access. The brand lifetime structurally ties node access to single-threaded
> exclusive token possession, which discharges L4 and (with the affine list) L1.

The honest question: does *this* re-import rule keep ordinary code seam-free and proof-free, or
has the obligation (the `'brand` token / the `unsafe` adapter / the `Pin` discipline) **moved
out of the foundation and into the consumer's ordinary code**?

## 5. Threat model

In scope: any AM-level UB (UAF via L2, aliasing `&mut` via L4, corruption via L1/L3) reachable
by **safe** consumer code using only the re-exported interface. Adversarial node ownership
(arena, stack, pool), adversarial free/move timing, and adversarial traversal patterns are all
in scope. Out of scope: bugs inside the foundation author's `unsafe` link-splicing glue.

## 6. The scoring rubric — which horn does your attack hit?

- **Horn 1 — sound re-import.** Exhibit safe consumer code that frees/moves a still-linked node
  (L2 UAF), double-links a node (L1), or obtains two `&mut` to one node (L4) — *without* the
  branding/token machinery of §3(a). I.e.: show the *naive* affine-handle re-import (the one
  that worked for SPSC) is **unsound** here. Give the exact sequence.
- **Horn 2 — decidable & proof-free safe fragment.** Show that the *only* way to recover
  soundness (§3a/b/c) forces the consumer to **thread a ghost/brand token through ordinary
  code, write an `unsafe` adapter, or carry a `Pin`/flag discipline** — i.e. ordinary code can
  no longer be proof-free / seam-free.
- **Horn 3 — seam concentration.** Count the crossings: for SPSC the consumer crossed the seam
  **once** (at `split`) and then `push`/`pop` were free. For this primitive, how many times must
  ordinary code present the token / cross the seam — once per list, or **once per access**? If
  it is once-per-access, concentration has dropped.

The design claim under test: **the affine-handle re-import that closed the SPSC seam does NOT
close here; soundness forces the obligation back into consumer code (a horn drops).** Confirm or
refute, and say which horn, with the mechanism.
