# newlang — a tutorial on the impossible language, and the PL theory you need to read it

**Audience.** A mid-level career developer: you write production code, you've used Rust or
C++ or Go, you know what a segfault is and roughly why it happens, but you have not formally
studied programming-language theory. By the end you will understand (a) the actual PL theory
this project rests on, from the ground up; (b) the state of the art in "safe systems
languages"; and (c) what newlang discovered and *why it matters*.

**What newlang is.** Not a language. It is a research project that asked "what would the
perfect systems language look like?", listed seven properties such a language should have,
and then **proved — adversarially — that the seven cannot coexist**, mapped exactly where the
wall is, and then **built fourteen real things that live right up against the wall (seven that close, plus seven probes — ending by showing the closed basis is *per soundness domain*, and that which seams close depends on the *floor-domain match*)** to show what
you *can* do. The deliverables are an impossibility result, a theorem (the *Seam Trilemma*),
a Lean proof, and a small Rust crate of worked examples.

**How to read this.** Parts 1–5 are the PL bootstrap — the prerequisites, taught from
scratch, with the state of the art woven in. Parts 6–10 are the project itself. Part 11 is
the transferable meta-lessons. Each part stands alone; skip what you know. Code references
point at real files in this repo (`seam/`, `proof/`, `design_seam/`).

> **Catch-up note (2026-06-28) — what's new since the first version.** This tutorial originally
> ended at the seven-property impossibility result and its first four worked seams. It has since
> been carried through the project's *constructive* arc, and the story got materially deeper. Two
> additions, both in Parts 9–11:
> 1. **The trilemma turned out to be a *projection* — of a *closed* object.** Its three horns are a
>    low-dimensional shadow of an object with **nine clean axes** — each an *already-named* PL concern
>    (zero-cost abstraction, generativity, refinement-legibility, the `core`/`alloc` split,
>    exception-safety, leak-freedom) — plus an *inseparable category* and two *outside-the-axes* slots.
>    Eight more primitives (5–12) probed it one at a time, so the result is a **unification** of known
>    axes under one question — *what does the safe witness cost?* — a question with **finitely many
>    answers**. That honest deflation, and the *closure* of the basis, is the spine of the new material.
> 2. **The method has *seven* verdict modes** — the fifth closes the basis, the last two leave the
>    domain. The probes past seven exercise all of them. (8) **rejected** — confounded. (9) **confirmed**
>    — *unwind-safety*. (10) **confirmed and ruled distinct** — *leak-freedom*. (11) **real but
>    structurally inseparable** — *move-stability*. (12, a validity read) **projects entirely onto the
>    basis** — *the cost-basis is closed*. (13, a constant-time secret) stepped **outside memory safety**:
>    information-flow is a *different soundness domain* with its own basis — closure is **per domain**. And
>    (14, a capability-gated declassification) probed whether the IFC domain has a *closing* seam and found
>    it does **not** — both IFC seams break, because **Rust's affine floor carries none of the IFC
>    precondition** (it bounds resource *uses*, not information *bits*). So the deepest result: **which
>    seams close depends on the floor-domain match** — the Seam Trilemma is *domain-general*, the
>    cost-basis *per-domain*, and a floor closes a domain's seams only when it carries that domain's
>    precondition. A discovery procedure that only confirms is measuring nothing; this one rejected,
>    confirmed, distinguished, recognised the inseparable, certified closure, *recognised a new domain*,
>    and *diagnosed a floor-domain mismatch*.
>
> If you read the earlier version, the genuinely new content is **Primitives 5–14** (Part 9), the
> "un-bundlings" + the rejection + the inseparable ruling + the closed-basis capstone + the
> domain-generalization + the floor-domain refinement, and **meta-lessons 6–10** (Part 11). Everything
> in Parts 1–8 is unchanged prerequisite material.

> **One-paragraph spoiler.** A type system that is *decidable* (the compiler always halts with
> yes/no and never asks you to write a proof) fundamentally cannot also be *expressive enough*
> to derive all of memory safety, concurrency, and zero-cost abstraction from first principles.
> Something has to give. Real languages give up different things, and the *seam* — the place
> where a checked, decidable world hands off to a trusted, proven one — is unavoidable. The
> interesting engineering is in *shaping* that seam, and the shape is dictated by one question:
> can the dangerous fact be carried by the cheap checker, checked at runtime, assumed as an
> axiom, or none of the above?

---

# Part 1 — The dream: seven properties of an ideal systems language

Start with the wish list. A "systems language" is one you'd write an operating system, a
database, a trading engine, or device firmware in: it runs on bare metal, with no garbage
collector pausing it at random, and it must never corrupt memory. The dream language has all
seven of these at once:

1. **Executable specification** — the spec *is* the type system *is* the compiler. No gap
   between "what it should do" and "what the checker enforces."
2. **Unified proof/systems layer** — no FFI seam between "proof land" (where you prove things)
   and "systems land" (where you poke hardware). One language.
3. **Ownership without GC, derived from the proof layer** — memory safety with no garbage
   collector, and *not* via a hardcoded special-case checker but derived from the proof rules.
4. **Decidable type checking** — the compiler always terminates with a yes/no answer, and the
   common case needs *no user-written proofs*.
5. **Bare-metal targeting** — no runtime, no reference-count spikes, no GC pauses; usable for
   hard-real-time.
6. **Zero-cost abstractions** — the proof/safety machinery *erases* at compile time; nothing
   survives to slow the program down.
7. **AI-legible semantics** — a model trained on the corpus can reconstruct the spec (a modern
   nice-to-have).

Each one ships *somewhere* today. The conjecture newlang tested is whether they ship
*together*. They do not, and property #4 — decidability — is the antagonist in every conflict.
To see why, you need to know what a type checker actually is.

---

# Part 2 — PL bootstrap I: what a type checker actually is

### Types are a proof system

A type is a *claim*, and type checking is *checking a proof of that claim*. When you write
`fn f(x: u32) -> bool`, the type `u32 -> bool` is the proposition "given a 32-bit unsigned
integer, this produces a boolean," and the compiler verifies the body is a valid proof of it.
This is the **Curry–Howard correspondence**: types are propositions, programs are proofs. You
don't need the deep version; you need the consequence — *a type checker is a theorem prover*,
and everything that is true of theorem provers is true of type checkers.

### Soundness: the property that makes types worth anything

A type system is **sound** if "well-typed" implies "well-behaved." Formally, soundness is two
lemmas: **progress** (a well-typed program never gets stuck — it can always take the next step
or is a finished value) and **preservation** (taking a step keeps the program well-typed).
Together: a well-typed program never reaches an undefined state. Soundness is what lets you
trust `u32` actually holds a number and not a dangling pointer. An *unsound* type system is
decoration. (newlang's calibration controls test exactly this — see Part 6.)

### Decidability: the checker must halt

Here is the constraint that ends up being load-bearing for the whole project.

A *decidable* type checker is one that always terminates with yes or no. You want this — a
compiler that might loop forever on valid code is unusable. For simple type systems (Java, Go,
basic Rust), decidability is easy. But the dream's property #1 ("the spec *is* the type
system") pushes toward **dependent types** — types that contain *values*, like
`Array<T, n>` where `n` is a runtime-ish number, or `Vec<T> where len > 0`. Dependent types
let the type system express *arbitrary* correctness properties. And there's the catch:

> Once types can express arbitrary propositions, type checking *is* theorem proving in
> arbitrary logic, which is **undecidable** (you can encode the halting problem, or Hilbert's
> 10th problem on Diophantine equations, into type equality). So the checker either (a)
> restricts to a *decidable fragment* — losing expressiveness — or (b) asks the human to supply
> the proof — losing "no user-written proofs."

This is the first fork in the road, and property #4 sits right on it. "Decidable AND no user
proofs" means you must find a fragment of logic that is simultaneously **expressive enough** to
state your safety properties and **decidable enough** to check automatically. That fragment is
small. The whole negative result is the discovery that it is *too* small to also carry
ownership, concurrency, and erasure.

**Worked intuition.** Refinement types like `{x: u32 | x < 100}` are decidable when the
refinements live in a theory an SMT solver decides quickly (linear integer arithmetic,
"QF-LIA"). Push to nonlinear arithmetic (`x * y < z`) and you're at the edge of decidability;
push to "this pointer aliases no other live pointer" and you're in undecidable separation
logic. The dream wants the *power* of the second with the *automation* of the first.

State of the art for "decidable + cheap": **Liquid Haskell** and **Flux** (refinement types
for Rust) carve out a decidable refinement fragment and lean on an SMT solver. They are
excellent and they are *bounded* — they work precisely where the refinement logic stays
decidable, and they punt (to the human, or to "can't express it") elsewhere. That boundary is
the trilemma in the wild.

---

# Part 3 — PL bootstrap II: memory safety without a garbage collector

### What "undefined behavior" actually is

A systems language without a GC must guarantee memory safety *statically*, because there is no
runtime to catch you. The failure modes (collectively **undefined behavior**, UB):

- **Use-after-free (UAF):** you free memory, then read or write through a pointer to it. The
  memory may now hold something else. This is not "a crash" — it is the compiler's assumptions
  being violated, which means *anything* can happen, including silent corruption and remote
  code execution. UB is "the abstract machine has no defined meaning for this," and optimizers
  are allowed to assume it never happens.
- **Double-free:** freeing the same allocation twice — corrupts the allocator.
- **Data race:** two threads touch the same memory, one writing, with no synchronization.
  Undefined at the language level even if it "looks fine."

A GC prevents UAF by never freeing memory anyone can still reach. The dream forbids a GC
(property #5), so it needs another mechanism. That mechanism is **ownership**.

### Ownership, moves, and affine types

The core idea: every value has exactly one **owner**; when the owner goes away, the value is
freed; you cannot use a value after you've given it away. This is a **substructural type
system**. Normal logic lets you use a fact as many times as you like; substructural logics
restrict that:

- **Linear** types must be used *exactly once* (can't duplicate, can't drop).
- **Affine** types must be used *at most once* (can't duplicate, *can* drop). **Rust's
  ownership is affine.**
- **Relevant** types must be used *at least once*.

The key property affinity buys you: **no duplication.** If a value cannot be copied, then there
is only ever one of it, and "free it when the owner is done" is safe — there's no second owner
to be surprised. Remember this; it is exactly the trick that makes one of newlang's four
primitives (the SPSC queue) work, and exactly the trick that *fails* for another (the slab,
whose keys must be copyable).

### Borrowing, lifetimes, and the borrow checker

Owning-and-moving alone is too restrictive — you'd have to give a value away to let anyone look
at it. So you add **borrowing**: temporary references that don't transfer ownership. Two kinds:

- `&T` — a **shared** (immutable) borrow. Many allowed at once.
- `&mut T` — a **unique** (mutable) borrow. Exactly one at a time, and no shared borrows
  coexist with it.

The rule "many readers XOR one writer" is what prevents data races and iterator invalidation
*at compile time*. To enforce it, the compiler must know *how long* each borrow lasts — its
**lifetime**, which is really a **region** of the program. The **borrow checker** is a dataflow
analysis that solves these region constraints. Modern Rust uses **NLL (Non-Lexical Lifetimes)**:
a borrow lasts only until its last *use*, not until the end of its lexical scope, computed by
liveness analysis. (The next-gen engine, **Polonius**, reformulates this with more precision.)

The thing to internalize:

> The borrow checker is a **decidable, proof-free, hardcoded** analysis. It is *not* derived
> from a general proof layer; it is a special-purpose dataflow pass with a fixed notion of
> "region." That is exactly why it is cheap and always terminates — and exactly what property
> #3 of the dream forbids (it wants ownership *derived from the proof layer*, not hardcoded).

This is the second fork. Rust gets memory-safety-without-GC by **hardcoding** an affine borrow
checker. The dream wants the same guarantee *derived* from a unified, decidable, proof-erasing
layer. newlang's calibration control (Part 6) proves that the hardcoded version settles cleanly
and the *derived* version is precisely the impossible part.

---

# Part 4 — PL bootstrap III: the limits, and where proofs come back

The borrow checker is powerful but it cannot see everything. The places it fails are exactly
where real systems code lives, and they are where the "seam" of the project comes from.

### Copy handles defeat affinity

Affinity works because a value can't be duplicated. But some handles *must* be copyable — an
array index, a slab key, a `Copy` integer. The moment a handle is `Copy`, the borrow checker
can no longer track "is this the unique live reference?" — you can copy it freely and it has no
idea. This is why Rust cannot put **interior mutability** (mutating through a shared `&`) into
the static checker, and instead provides:

- **`Cell`/`RefCell`** — move the borrow check to **runtime** (a flag checked on each access;
  `RefCell` panics on violation). You pay a runtime cost for a guarantee the static system
  couldn't give.
- **`UnsafeCell`** — the primitive that says "I, the human, will uphold the invariant; compiler,
  trust me." Every safe interior-mutability type is built on it.

Hold onto this: **the static checker is blind to a `Copy` handle's validity, and the standard
escape is a runtime check (`RefCell`).** newlang's slab primitive is exactly this pattern, and
its lesson — that you can recover soundness at *runtime cost* — is the project's deepest finding.

### Separation logic: reasoning about the heap and aliasing

To reason about programs that *do* alias and mutate shared memory, you need a logic richer than
types: **separation logic**. Its key connective is the *separating conjunction* `P * Q` —
"the heap splits into one part satisfying P and a *disjoint* part satisfying Q." It makes
ownership and non-aliasing first-class. **Concurrent separation logic** adds reasoning about
threads and shared state. The state-of-the-art framework is **Iris** (a higher-order concurrent
separation logic, mechanized in Coq) — it is the power tool for proving that tricky concurrent,
aliasing, `unsafe` code is actually sound.

### RustBelt: why `unsafe` can be *justified* but not *checked*

Here is the deepest prerequisite. Rust's safe fragment is sound, but its standard library is
full of `unsafe` (in `Vec`, `Arc`, `Mutex`, channels…). Why is *that* sound? **RustBelt**
(Jung, Jourdan, Krebbers, Dreyer — POPL 2018) answered this: it built, in Iris, a *semantic
model* of Rust types and proved that each `unsafe` abstraction, behind its safe interface,
upholds the model. The profound consequence for newlang:

> The safety of an `unsafe` block is a theorem in undecidable concurrent separation logic. It
> can be **proven once, by a human, with a proof assistant** — but it cannot be **decided by a
> compiler**. The proof does not fit in the type system; it lives in a paper and a Coq
> development. This is *the* seam: a checked, decidable world (safe Rust) sitting on top of a
> trusted, separately-proven world (the `unsafe` core).

### The unsafe boundary as a trusted computing base

Every verified system has a **TCB** — a trusted computing base, the part you *assume* correct
rather than prove. `unsafe` is Rust's TCB boundary; an `assume` in a proof is one; in seL4,
CompCert, and F\*/Low\* the TCB is explicit and minimized. A crucial distinction the project
leans on: some facts are *unprovable in principle from inside the program* — e.g. "this memory
address really is the UART register" is a fact about *physical hardware*, an axiom you discharge
by reading a datasheet, not a theorem any checker in any language could decide. That is a
*different kind* of trust than "this is provable but we didn't mechanize it." (newlang's MMIO
primitive is exactly this case.)

### Weak memory: why concurrency is even harder

On real hardware (ARM, POWER, RISC-V), memory operations are *not* globally ordered — different
cores can observe writes in different orders. To write correct lock-free code you use
**acquire/release** atomics to establish **happens-before** edges: a `Release` store
"publishes" everything before it, and an `Acquire` load that reads it "sees" all of that. The
safety of a lock-free queue *is* an argument in this weak-memory model. (newlang's SPSC queue
puts exactly two such atomics at its core — see Part 9.)

---

# Part 5 — State of the art: the landscape of safe systems languages

Everything above sorts the field into a map. Every real system **picks a horn** of the same
trilemma the project later names. Knowing this landscape is most of being literate here.

**The decidable, cheap, proof-free end** (give up expressiveness / accept a seam):
- **Rust** — affine ownership + borrow checking, hardcoded and decidable; `unsafe` is the seam,
  and the obligation at the seam lives "in the programmer's head," not the type system.
- **Flux**, **Liquid Haskell** — refinement types over a decidable SMT fragment. Great until the
  property leaves the fragment.

**The proof-bearing end** (give up decidability / accept user proofs):
- **Verus** — Rust + SMT-checked proofs; you write `requires`/`ensures` and proof code.
- **F\* / Low\*** — dependently typed, SMT + tactics; Low\* extracts to C (used for verified
  crypto in Firefox/Linux — the HACL\* project).
- **Creusot**, **Prusti**, **Aeneas** — Rust verification via SMT (Creusot/Prusti) or a pure
  functional translation for proof assistants (Aeneas).
- **Dafny** — verification-oriented language, SMT-backed.
- **Kani** — bounded model checking for Rust (no proofs, but *bounded*, not a full guarantee).
- **VeriFast**, **CN** — separation-logic verifiers for C.

**The cautionary tales** (tried to have more, paid for it):
- **ATS / ATS2** — extremely expressive linear+dependent types; the canonical *ergonomics
  cliff*: theoretically capable, practically punishing to write. (newlang's attackers use "is
  this the ATS2 cliff?" as a standing test.)
- **Cyclone**, **Mezzo** — research languages with rich region/permission systems; influential,
  not adopted.
- **Linear Haskell**, **Austral**, **Cogent** — linear-types experiments, each conceding
  something (ergonomics, scope, or a restricted domain).

**The specific tools newlang's primitives use:**
- **GhostCell** (Yanovski, Dang, Jung, Dreyer — ICFP 2021) — a *branding* trick: an invariant
  lifetime `'brand` plus a token, letting you safely mutate aliased graph structures *if* you
  thread the token through every access. The soundness rests on RustBelt-style reasoning.
- **slotmap / generational-arena** — `Copy` keys made safe by a per-slot **generation counter**
  checked at runtime; the production answer to "how do you hand out copyable handles into a
  pool without UAF."

**The one sentence that organizes the field:** *every shipping system is decidable-and-cheap
with a trusted seam (Rust), or expressive-and-sound with human proofs (Verus/F\*), and none is
all three.* That observation, made rigorous, is the project.

---

# Part 6 — The method: adversarial design with cold AI instances

newlang's twist is *how* it established impossibility. You cannot prove "no language has all
seven" by enumeration. So it ran an **adversarial design process** and watched it fail in a
*structured, convergent* way.

### The roles

- **PROPOSER** — produces one concrete, formal design decision (a type rule, an algorithm).
- **Five cold ATTACKERs**, each with one angle: *ergonomics* (will anyone write this?),
  *soundness* (exhibit a well-typed program that does UB, or show checking is undecidable),
  *implementation* (which compiler pass blows up?), *expressiveness* (what real program can't
  be written?), *prior-art* (who tried this and what did it cost them?).
- **JUDGE** — rules each attack genuine-flaw / argued-past / tradeoff, then SURVIVED / FAILED.
- **SYNTHESIZER** — when a question fails every revision, distills the *convergent reason* into a
  recorded property-incompatibility (the payload).
- **RECONCILER**, **HISTORIAN** — check pairwise collisions of settled decisions; record state.

### "Cold" is the load-bearing trick

An attacker must reason from the proposal *alone* — no shared context, no memory, no knowledge
of who wrote it. The project runs each role as a fresh `claude -p` subprocess with all ambient
context stripped (`--system-prompt`, empty settings so no hooks, no MCP, a clean working
directory). Statelessness is *demolition's* correct property: a cold reviewer can't be charmed
by the design's backstory. (The mirror image — *construction* needs continuity — is why the
later build phase uses a stateful builder and keeps the cold instances as the red team only.)

### Calibration: why a 0/N result needs controls

If a process settles *nothing*, that is ambiguous: either the target is genuinely impossible, or
your instrument is rigged to reject everything. You break the ambiguity with **controls**:
- **Good control** — feed it a known-good, shipping design (Rust's borrow checker, with the
  impossible constraints waived). A calibrated process must *settle* it. (It did — and on the
  first try it even caught a real lifetime bug before accepting the fix.)
- **Bad control** — feed it obvious garbage ("no checker, programmers just don't write bugs").
  A calibrated process must *reject* it. (It did.)

Only a process that settles good designs and rejects bad ones makes a 0/N on the real questions
*signal*.

### Decontamination: the move that makes the result trustworthy

The first attempt to design a "seamful" language kept too many constraints, so failures mixed
"the seam is impossible" with "the proposer couldn't write a bug-free borrow checker." The fix
is the project's sharpest methodological idea: **hand the proposer the known-good sub-component
and watch whether the failure *moves* or *disappears*.** They stipulated "use Rust's borrow
checker exactly as shipped" — a concession the good control had *already proven costless* — and
the failure did not vanish. It **relocated** onto the seam itself. A failure that *moves* is a
real structural obstruction; one that *disappears* was just proposer weakness. (You will see
this exact move pay off across all eight primitives in Part 9 — where the obstruction *vanishes*,
*moves and stays decidable*, *moves onto a runtime witness*, or *moves and breaks* is the entire
verdict each time.)

---

# Part 7 — The negative result: the decidability fault line

Run the full process over the seven properties, decomposed into five design questions
(sequential ownership; the decidable fragment; proof erasure; the unsafe boundary; interior
mutability & weak memory). Result: **0 of 5 settled, 6 hard incompatibilities, and every single
one places property #4 (decidable, no user proofs) on one side of the conflict.** The dream
cracks along one fault line:

> **decidable-and-cheap  XOR  sound-derived-seamless-zerocost.**

Two readings matter:

1. **It's a covering, not an intersection.** For *each* property individually, *some* language
   has it (Rust, Lean/Coq, RustBelt). A few languages *together* cover all seven. But no single
   language has all seven, because each concedes one of #2/#3/#6. "A few languages together do
   everything" is true and is *not* the same as "one language does everything."
2. **Merging is itself the forbidden move.** "Combine them into one seamless language" *is*
   property #2 (no seam). So the act of fusion is one of the things you must give up. The
   un-fusable residue *is* the seam — and the seam is *load-bearing*, because it's where the
   undecidable obligation gets quarantined behind a trusted boundary. Erase the seam and the
   undecidability reappears at the weld.

This is the conservation law the whole project circles: you cannot delete essential difficulty,
only move it.

---

# Part 8 — The constructive flip: the Seam Trilemma

If the seam is unavoidable, stop fighting it and *design it*. Concede property #2 (a seam is
allowed) and ask: what is the best *seamful* language? Re-running the process under that framing
(with the borrow checker stipulated, per decontamination) yields a clean three-way constraint —
the **Seam Trilemma**. A seam is the boundary where a primitive whose true precondition lives in
rich, undecidable separation logic is re-exported as a checked safe interface. You want three
things; you can have any two:

1. **Sound re-import** — crossing back from the seam launders no UB into safe code.
2. **Decidable, proof-free safe fragment** — ordinary code type-checks with no user proofs, no
   SMT, no threaded token.
3. **Seam concentration** — ordinary code touches the seam *rarely* (ideally once, at
   construction), not on every operation. (The opposite is the ATS2 ergonomics cliff.)

Why you can't have all three (the mechanized core, `proof/SeamGap.lean`): the foundational
primitives' true preconditions are Iris/RustBelt-grade (undecidable, certificate-free). The safe
fragment can only *receive* a witness that is simultaneously decidable, proof-free, and erasable
— strictly weaker. So the re-import obligation is necessarily weaker than the precondition it
must certify, and the missing conjunct forces exactly one of: **drop soundness** (UB launders
in), **drop decidability** (an external prover / user proofs), or **drop concentration** (every
use crosses the seam). Every shipping language picks a horn: Rust drops concentration informally;
Verus/F\*/Creusot drop decidability; the unsound corner is the one to avoid (and the bad control
shows it's detectable).

**The productive reading:** a seamful language *is* worth building. The sweet spot is "Rust, but
`unsafe` is `proof`" — drop the decidability horn at the seam, keep the safe fragment cheap, and
concentrate the proof burden in a few foundation authors. The value of the project is knowing
you are *choosing a horn*, not escaping the trilemma — and which horn, and why the fourth option
doesn't exist.

---

# Part 9 — Build and break: fourteen real seams (the heart of it)

Theory in hand, the project stopped arguing and *built*. Crucial design of the experiment:
**build and break are different machines.** Construction is continuity, so a *stateful builder*
(a normal, context-holding process — here, me) writes the smallest thing that compiles.
Demolition is coldness, so the cold harness is the **red team**, aimed *only* at the re-import
rule. Each seam ships as a **text spec** (what the cold attackers attack) *and* a real,
`rustc`-checked crate (`seam/`) whose consumer code is `#![forbid(unsafe_code)]` and whose UB-
laundering attempts are `compile_fail` tests. Fourteen primitives: the first four chosen because their
preconditions have four *different shapes*; the next three (5–7) because they reopened a deeper
question — *how many axes is the trilemma actually a projection of?* — each recovering one; and **seven
probes past seven** (8–14) — one *no* (rejected), two *yes* (unwind-safety at N ≥ 8, leak-freedom at N ≥ 9,
ruled *distinct*), one *"real but inseparable"* (move-stability, N stays 9), one that asked whether the
axis-list was *finished* and got back *"yes — the basis is closed"* (validity), and **two that left memory
safety entirely**: a constant-time secret (*"that is a **different domain** — closure is per-domain"*)
and a capability-gated declassification (*"and no seam closes in that domain on this floor — the **floor
is mismatched** to it"*). The seven verdicts together are the point: the last three *conclude* the arc,
then *generalize* it, then *bound* the generalization to the floor. The headline finding:

> **Whether a seam closes is a function of the precondition's shape** — specifically, *what the
> dangerous conjunct is*: can the cheap checker carry it, can a runtime check carry it, is it an
> external axiom, or does it escape to the caller entirely? **And when a seam *does* close, *which*
> of several hidden axes it spends to do so is a second, independent question** — the one the fifth
> primitive forced open.

### Primitive 1 — SPSC lock-free ring buffer → the seam CLOSES (`seam/src/lib.rs`)

A single-producer/single-consumer queue: one thread pushes, one pops, no locks, two
acquire/release atomic indices. Its true precondition is genuinely hard (per-slot exclusion,
happens-before publication, single-producer cardinality, payload thread-safety). And yet **every
conjunct that the caller could observe is carried by the floor for free**:
- *Cardinality* (only one producer) → **affinity**: the `Producer`/`Consumer` handles are
  move-only (`!Clone`), so you literally cannot make a second producer.
- *Storage lifetime* (no use-after-free) → **NLL lifetimes**: `split(&'a mut self)` hands both
  handles a `&'a` borrow of the buffer, so the borrow checker forbids the buffer from dropping
  while either handle lives. UAF becomes a *compile error*.
- *Payload safety* → a **decidable `T: Send` bound** the compiler resolves; the consumer never
  writes it.
- *Happens-before* (the genuinely undecidable part) stays **inside the `unsafe` block**, leaving
  *no caller-visible residue*.

Cold verdict: **SEAM-HOLDS** across four independent panels. This is not luck — it is literally
what `heapless::spsc` / `rtrb` ship, and RustBelt machine-checked the class. **Seam width: O(1)
per buffer** (cross once at `split`, then `push`/`pop` are free). When the dangerous fact fully
*encapsulates*, the seam closes losslessly.

### Primitive 2 — MMIO read-to-clear register → SOUND modulo an external axiom (`seam/src/mmio.rs`)

A memory-mapped device register where *reading clears* the pending bits. An affine singleton
handle + encapsulated volatile access makes the memory-safety seam close to O(1) just like SPSC.
But one conjunct of its precondition — "this address *really is* this register, with these
datasheet bit-semantics" — is **a fact about the physical world**. No checker, in this language
or any, can decide it; it is an **axiom discharged by fiat** at one `unsafe` line. The cold
harness named the category exactly right: this is a **TCB `assume` boundary** (the same kind
seL4, CompCert, Low\* rely on), **not a fourth trilemma horn**. The real recordable *cost* is a
horn-3 concentration nudge (the interrupt-handler use needs a foundation `Send` opt-in). Verdict:
**SEAM-COSTS**. Lesson: some obligations aren't "undecidable," they're *outside the formal system
entirely* — and the honest move is to *name* the axiom, not hide it.

### Primitive 3 — generational slab → the seam CLOSES, at RUNTIME cost (`seam/src/slab.rs`)

A pool that hands out `Copy` keys (`insert -> Key`, `get(Key)`, `remove(Key)`). The handle
*must* be `Copy` — storing and passing keys around is the whole point — so **affinity is
useless** (recall Part 4: a `Copy` handle defeats the static checker). The danger is stale keys
and ABA: a copied key outliving its slot. The fix is exactly `RefCell`'s move, generalized: a
per-slot **generation counter**, bumped on `remove`, checked at runtime; `get` returns `None`
for a stale key instead of reading freed memory. This keeps **all three trilemma horns** (sound,
decidable, concentrated — the whole interface is safe, seam width O(1)) — and pays on a **fourth
axis the trilemma folded away: zero-cost erasure (property #6).** The generation word per slot
and the compare-branch per access *do not erase*. Verdict: **SEAM-COSTS [zero-cost]**.

This primitive forced the **first** of four un-bundlings:

> **The Seam Trilemma is a 3-of-4 projection.** "Concentrated & proof-free" had quietly bundled
> away *zero runtime cost*. The slab prises them apart: a *concentrated* seam can still be
> *non-erasable*. Seam width (horn 3) and zero-cost (#6) are independent axes — the slab aces the
> first and spends the second. And #6 was one of the casualties back in Part 7's unified result;
> the constructive phase puts it back on the map. *(The fifth, sixth, and seventh primitives, below,
> force three more un-bundlings — API-locality, then legibility/solver-freedom, then bare-metal —
> turning "3-of-4" into "3-of-≥7". Then **two eighth-axis probes**: the eighth is **rejected** (its
> axis confounds with expressiveness), the ninth **confirmed** (unwind-safety, a clean off-diagonal) —
> "3-of-≥8". That the method rejects one and confirms the other is itself a result.)*

*(A teaching aside from the real run: the slab's **first** cold gauntlet FAILED on a genuine UB —
but the bug was in the **spec** (a generation-only check authenticates a fresh pool's vacant
slot 0), while the **implementation** checked occupancy first and was sound. A regression test
proved it. That spec↔implementation divergence is exactly what the build-attacks-text /
code-validated-by-compilation separation exists to catch. The fix is slotmap's real
`version | 1` parity trick.)*

### Primitive 4 — intrusive doubly-linked list → the seam BREAKS (`seam/src/intrusive.rs`)

Nodes contain their own `next`/`prev` links and are owned *elsewhere* (an arena, the stack). Now
the danger **escapes to the caller**: nodes alias each other while the caller still owns them.
Affinity is *defined* as the absence of aliasing, so the SPSC recipe is structurally
inapplicable. The sound tool is **GhostCell branding** — but the cold harness proved something
sharper than expected: **a brand is an *access* capability, not a *storage* lifetime.** The token
discharges aliasing (who may form `&mut`) but carries *no* relation between a node's membership
and its allocation — so the rule type-accepts a **use-after-free** (a still-linked node drops,
a later traversal reads freed memory). Verdict: **SEAM-BREAKS [horn 1]**. Every repair just
relocates the horn: tie nodes to one lifetime → lose early reclaim (drop *usability*, the
primitive's purpose); full separation-logic proof → drop *decidability*. The trilemma bites
hardest here.

### Primitive 5 — branded index → the seam CLOSES, and the *dimension count grows* (`seam/src/branded.rs`)

This one was chosen as a deliberate probe. The slab proved zero-cost is a hidden fourth axis; the
test for whether there is a *fifth* is to find a primitive that aces **all four** known axes —
sound, decidable, concentrated, *and* zero-cost — yet still pays. **Sound unchecked indexing**
(Beingessner/Gankra's thesis Ch. 6.3; bluss's `indexing` crate) is exactly that. A *branded* index
is a raw `usize` tagged with a generative lifetime `'id` minted fresh per `scope(slice, |view| …)`;
because `'id` is unique to one scope over one immutable (length-frozen) slice, an index that passed
*one* bounds check is provably in-bounds forever — so indexing elides the check. It is the first
seam here that is **faster than safe code**, and the brand is a zero-size `PhantomData` that erases
completely. All three horns hold; zero-cost is aced. So what's left to pay?

The cold panel — grounding the answer in four decades of literature — found a real cost *outside
the three horns*: the brand `'id` is a **viral generative type parameter**. It never appears in
consumer *code* as a seam crossing (seam width is O(1), one `scope` per slice), but it appears in
every consumer *type*, and traps every index inside the `scope` closure (an `Idx<'id>` literally
cannot be returned). That is **API-locality**, and it is provably *not* seam width: Launchbury &
Peyton Jones's `runST` (1994) and Kiselyov & Shan's branded indices (2006) pay the identical tax in
Haskell — a language with *no `unsafe` and no seam at all*. A cost that exists where seam width is
undefined cannot *be* seam width. Verdict: **SEAM-COSTS [API-locality]**. (The panel also caught a
builder overclaim — naive single-index `try_index` re-checks per element, so the zero-cost win
needs the *range*-branding the toy scoped out — and noted that AI-legibility (#7) is yet another
axis: a brand is a nominal token, whereas a refinement type `{v:Int | v < len}` is brand-free *and*
legible, so #5 and #7 dissociate.)

This forced the **second** un-bundling, and the project's deepest structural result:

> **The Seam Trilemma is a 3-of-N projection, N ≥ 7.** The slab un-bundled zero-cost from
> "concentrated & proof-free"; the branded index un-bundles **"concentration" itself** into a
> *code* face (seam width) and a *type* face (API-locality). Beyond the three horns the constructive
> primitives expose more independent axes — zero-cost (#6), API-locality, AI-legibility (#7), and
> bare-metal (#5) — each proven real by a *witness that two trilemma-identical configurations
> differ*. They even have structure: within the branded index, API-locality and #6 **trade**
> (range-branding buys erasure by deepening the brand tax). The full face lattice is
> `design_seam/REALIZABILITY.md`. *(Two claims are still open here — that #7 is independent of
> API-locality, and that bare-metal is a separate axis. The sixth and seventh primitives witness them.)*

### Primitive 6 — refinement-typed bounds → CLOSES the cell #5⟂#7 (`seam/src/refined.rs`)

The branded index left a gap: it pays #5 *and* #7 together (a brand is viral *and* a nominal token),
so it cannot prove they are *separate* axes — for that you need an **off-diagonal**, a primitive that
agrees with another on #5 but differs on #7. That is what a **refinement type** gives. Instead of a
brand, the witness is a **legible predicate**: `i: usize{i < len}` is "a `usize` the checker has
*proven* less than `len`," discharged by an SMT solver over a decidable arithmetic fragment (this is
**Flux** for Rust, Lehmann et al. 2023, and **Liquid Haskell**, Vazou et al. 2014). It is the exact
opposite corner from the brand:

- **brand-free** (#5 aced): a `Refined` is a plain value — you can store it, return it from a
  function, collect it into a `Vec`. The keystone test does exactly that, and it is a *compile error*
  for the branded `Idx<'id>` (whose `'id` cannot escape its scope). No brand, no closure.
- **legible** (#7 the opposite of the brand): the predicate `value < bound` *is* the type's contract;
  an outside reasoner reconstructs the safety invariant from the signature alone.

So pair it against the **slab**, reclassified **(brand-free, *illegible*)** — a `Copy` `Key`'s
live-slot validity is *not* reconstructible from its type (the earlier table's "legible" was too
generous, and two cold attackers independently endorsed the correction). Refined-vs-slab now holds
#5 fixed (both brand-free) and **flips #7**; branded-vs-slab holds #7 fixed and flips #5. The cold
judge ruled it outright: **"#5⟂#7 close? — YES."** AI-legibility is a *confirmed* independent axis.

And the refined index forced a **third** un-bundling — of **horn 2** itself. A refinement type is
*decidable* (the fragment terminates) but not *solver-free*: it needs an SMT solver in the trusted
toolchain. So "decidable & proof-free" was a bundle of **decidability** and **solver-freedom**, and
the refined index pays the second. (Honest guard: that solver-in-the-TCB cost is *not* a new seventh
axis — it is the original `unified` result's "keep the invariant faithfully → external prover" horn,
the documented founding tradeoff of refinement types since DML in 1998. The map gains *resolution*,
not a dimension.) Two more cold findings sharpened the artifact: the **R2 trade** (you can have
brand-free *or* zero-cost, not both — the #5↔#6 exchange again, from the other side), and a real
expressiveness bug (the model checked `bound == len` where safety needs only `bound ≤ len` — fixed).
Verdict: **SEAM-COSTS [horn 2]**.

### Primitive 7 — reference-counted shared ownership → recovers BARE-METAL (`seam/src/shared.rs`)

The first six seams share a hidden uniformity: none needs a heap. They are `core`-only — fixed
buffers, fixed pools, borrowed slices — so they run on bare metal (an OS kernel, a microcontroller
with no allocator). The seventh breaks it, and recovers seed property **#5 (bare-metal)** as an axis.

The new precondition shape is **dynamic shared ownership** (a minimal `Rc`): a value with multiple
owners, where the live-owner set is decided *at run time* by `clone`/`drop`, and the value is freed
exactly when the *last* owner leaves. That instant is not statically knowable — affinity gives you
*one* owner, NLL gives you *static* lifetimes, and the slab's generation check still has a single
owner (the pool). The only sound witness is a **runtime reference count**, and a refcount needs
storage that outlives every owner and is reclaimed on the last drop — i.e. a **runtime manager**,
here the global heap. The cost is made *compile-enforced*: the crate is `#![no_std]`, and `shared` is
the sole reason it pulls in `extern crate alloc` — delete it and the other six still build. A no-heap
kernel author simply cannot link this seam. That is the bare-metal axis.

It is genuinely independent of zero-cost (#6) — the **off-diagonal vs the slab** proves it: the slab
pays a runtime cost (#6 ✗) yet is bare-metal (a fixed pool, no heap); `Shared` *also* pays a runtime
cost (the refcount) *and* needs the heap. They agree on #6 and differ on bare-metal — so a runtime
*check* and a runtime *manager* are not the same thing. Verdict: **SEAM-COSTS [bare-metal]**.

But here the cold panel did its most useful work yet — it **deflated the novelty**. Yes, bare-metal
is a real, independent seventh axis. But it is not a *discovery*: it is precisely Rust's own
`core`/`alloc`/`std` layering (RFC 2480), and `heapless::pool`'s core-only refcounted pointer already
proves the trade (shared ownership doesn't *force* the heap — *unbounded dynamic count* does; a
bounded pool stays bare-metal). The honest name is *unbounded-dynamic-allocation vs
caller-provided-bounded-storage*. **Keep the axis; drop the "undiscovered dimension" framing.**

That deflation — the second of two (solver-freedom was the first) — is the project's most honest
result. *Every* axis the method recovers turns out to be an **already-named PL concern**: zero-cost
abstraction, generativity, refinement-legibility, the `core`/`alloc` split. newlang did not discover
new physics. It found that the trilemma's three horns silently *bundle a set of independently-known PL
design axes*, and that one question — **"what does the safe witness cost?"** — unifies them. The
contribution is the unification and the recovery method, not the dimensions.

### Primitive 8 — type-level-proof index → the eighth probe FAILS (`seam/src/typed_nat.rs`)

The first seven primitives each recovered (or, for MMIO/intrusive, located) something. The eighth was
built to *push the count to eight* — and the gauntlet **rejected it**. That rejection is the most
instructive moment in the project, so it gets a worked example of its own.

The idea: prove an index in bounds *entirely at compile time*, with no runtime check and no external
solver. Encode index and length as type-level naturals (`Z`, `S<Z>`, …); the in-bounds fact `I < Len`
is the trait bound `I: Lt<Len>`, discharged by two recursive `impl`s the trait resolver chases. The
proof carries no data, needs no Z3, and erases to a bare `get_unchecked`. It is **sound** — and the
panel conceded that unanimously: `Nat` is a *sealed* trait, so a consumer can impl the public `Lt` for
their own local type (orphan rule) but that type can never be `Nat`, and `get`'s `I: Nat + Lt<Len>`
bound rejects every forgery. No UB path exists.

The builder pitched an eighth axis — **checker-tractability**: the refined index needs a solver but
that solver halts *cheaply*; this one needs no solver but its checker halts only under an arbitrary
`#![recursion_limit]` (resolving `I: Lt<Len>` costs `I::VALUE` steps, so a deep-but-in-bounds index
fails with E0275 "overflow" until you raise the knob — out of fuel, not out of bounds). Held against
SPSC, the claim was: flips *only* checker-tractability.

Three independent attackers killed the axis (the seam stays sound; only the *deliverable* failed):
- **The off-diagonal is confounded.** `get::<I>` takes a *type*, never a runtime `usize` — so this
  seam can only index compile-time *constant* positions. It is a strict expressive subset of the
  branded/refined indices, whose entire reason to exist (`get_unchecked` on ring/arena/slab) is
  *data-dependent* indexing this seam **cannot express**. The "one-coordinate flip" silently also
  flips expressiveness — and you cannot read an independence off two primitives that don't accept the
  same programs. The confound is *intrinsic*: a runtime index carried by a compile-time proof is a
  contradiction.
- **The cost is encoding-relative.** The E0275 wall is an artifact of *unary* Peano. *Binary*
  type-level naturals (typenum's `UInt<U,B>`) make resolution O(log N) — they never hit the limit for
  realistic indices. The "intractable" cell measures *which numeral library you linked*, not the
  primitive.
- **It is mislabeled.** `Lt` resolution is *linear* — strictly *cheaper* than the refined index's
  NP-hard QF-LIA. So the cost was never "tractability." What `recursion_limit` marks is the absence of
  a guaranteed *terminating-resolution* fragment — exactly GHC's `UndecidableInstances` / the Paterson
  conditions / `-freduction-depth`. A real, **pre-existing** concern, not a new lattice axis.

Verdict: **FAILED [the axis deliverable]; the seam is sound.** The count stays **N ≥ 7** *on this
probe*. The lesson is in Part 11 (#8): a discovery procedure that cannot reject a bad candidate is not
measuring anything. But the *next* primitive shows the other half — the same method confirming a clean
candidate.

### Primitive 9 — take-and-replace → the eighth axis, CONFIRMED (`seam/src/take.rs`)

The second eighth-axis probe, built deliberately to avoid Primitive 8's failure mode: a primitive whose
candidate cost is isolable *without* dragging another axis along. The danger is different in kind — not
aliasing or bounds but **time**. `take(slot, f)` moves the value out of a `&mut T` (`ptr::read`), runs
`f`, and writes the result back. Between the read and the write there is a **gap**: the slot holds a
bitwise *duplicate* of the old value, a `T` that must never be dropped or read. The one thing that can
run arbitrary code in the gap, on Rust's floor, is **unwinding** — if `f` panics, the stack unwinds,
`f`'s frame drops the real value, and then the caller would drop the *duplicate* still in the slot: a
**double free**. This is the exact UB of a naive `take_mut`.

The sound witness is an **abort-bomb** (`take_mut`'s real pattern): a zero-sized local whose `Drop`
panics, armed across the gap. If `f` panics, unwinding drops the bomb *first* (it is `take`'s inner
local; inner frames unwind before outer ones), and a panic *during* unwinding is a **double-panic →
process abort** — which fires before the caller can drop the duplicate. The broken slot is never
observed. On the happy path the bomb is `mem::forget`-ten and nothing residual runs.

Here is the clean off-diagonal Primitive 8 lacked: this seam **aces all seven prior axes** — sound,
decidable & proof-free, O(1)-concentrated, brand-free, zero-cost on the happy path, legible, `core`-only
— and pays on **exactly one** new thing: U2, *unwind-safety*. Identical to SPSC on every named axis,
split on the candidate alone. Verdict: **SEAM-COSTS [none]** (a TRADEOFF — sound, all horns held). The
cold panel **confirmed unwind-safety as the eighth axis (N ≥ 8)** and explicitly contrasted its clean
isolation with Primitive 8's confound. Deflated, as ever, to pre-existing: Abrahams' basic/strong/nothrow
exception-safety guarantees, C++ `noexcept`, Rust's `UnwindSafe`/`catch_unwind`, `Mutex` poisoning.

The panel added two corrections that are now part of the result:
- **The axis is the *exception-safety-guarantee-level*, not "the abort cost".** Abort is merely *this
  seam's* discharge of U2. A `replace_with`-style sentinel (`T: Default`) discharges the *same* U2
  **soundly and recoverably** by filling the gap with a valid value — paying **API-surface** instead. A
  lock pays by **poisoning**. So the axis carries a **discharge menu**: one obligation, several pay-points
  that each spend a *different other* resource. (A naive move-out/move-back with no guard is just unsound.)
- **Historical honesty:** RFC #1508 *rejected* exactly this `take_mut` from `std` on precisely the
  abort-downgrade cost — the "ace seven, pay one" framing was the historically *losing* argument.

So the two eighth-axis probes together are the whole instrument: Primitive 8 is the method **rejecting**
a confounded candidate, Primitive 9 is the same method **confirming** a clean one (and relabelling it).
N reaches 8 only on a witness that holds every other axis fixed.

### Primitive 10 — drain-and-yield → the ninth axis, CONFIRMED (`seam/src/drain.rs`)

Having confirmed unwind-safety, the obvious next question: is there a *sibling* — and is it the same
axis or a different one? `Drop` not running *normally* has two failure modes: it runs at a **bad time**
(an unwind, Primitive 9), or it **never runs at all** (a leak). Rust makes the second a first-class
hazard, because **`mem::forget` is a *safe* function** (so are `Box::leak`, `ManuallyDrop`, and `Rc`
cycles) — you can always *skip* a destructor. This is the famous Rust **leakpocalypse**.

The danger primitive is `Vec::drain`: an iterator that moves elements out of a buffer by value. While
it iterates, the already-yielded slots are *moved-from* (bitwise live, must not be dropped). The
historical pre-1.0 bug ("leak amplification"): if the `Drain` guard is leaked mid-iteration and the
buffer still counts those slots, the buffer's `Drop` double-frees them. The sound witness is
**pre-pessimization** (the real `Vec::drain` fix): set the buffer length to **0 up front**, before
yielding anything; the guard tracks the un-yielded range to drop on its own `Drop`. Then a leaked guard
leaves a valid *empty* buffer — the un-yielded tail is **leaked** (a memory leak, not UB), never
double-freed. (Because `mem::forget` is safe, the consumer test can exercise the leak path *itself*
under `#![forbid(unsafe_code)]` — a rare keystone.)

The crucial design choice, learned from Primitive 8: a `Drain` is a **free-standing** value — a plain
`&mut` borrow, no closure. That matters because the *obvious* leak-freedom witness, a scoped-thread
guard (`thread::scoped`), would have confounded with **API-locality**: its sound form is a closure
(`thread::scope`), and "forced scope-closure" is *by definition* an API-locality cost. The drain guard
sidesteps that — it buys leak-safety by *owning its data* and downgrading a guarantee (leak ⇒ lose the
tail), so it needs no closure and aces API-locality. It aces all eight prior axes and pays on **exactly
one** new thing: leak-freedom. Verdict: **SEAM-HOLDS / SURVIVED**, the ninth axis confirmed (N ≥ 9),
deflated to pre-existing (the leakpocalypse; `mem::forget`-is-safe; RFC 1066; the `JoinGuard`/
`thread::scoped` removal; linear-vs-affine types).

The load-bearing ruling: **leak-freedom is *distinct* from unwind-safety, not a merged
"destructor-robustness" axis.** The off-diagonal is populated both directions — take-and-replace is
*unwind-sensitive* (needs the abort-bomb) but *leak-safe* (forgetting the bomb is fine); drain is
*leak-sensitive* (needs pre-pessimization) but *unwind-safe* (a panic mid-iteration just runs the
guard's `Drop`). "`Drop` runs at a bad time" and "`Drop` never runs" are independently witnessed, and
Rust's own docs codify them separately (*Exception Safety* vs *Leaking*). One honest nuance the panel
flagged: leak-freedom is the **thinnest, most-deflatable** axis in the lattice — its cost (leaked-tail
data loss) is invisible in all non-leak code — yet it is a genuinely populated off-diagonal, not the
non-axis the checker-tractability probe turned out to be. "Deflatable" and "not an axis" are different
verdicts; this is the former.

So three probes past seven map most of the instrument's behavior: **reject** the confounded (8),
**confirm** the clean (9), **confirm-and-distinguish** the sibling (10). The fourth probe found a mode
none of those covers.

### Primitive 11 — self-referential cell → the *inseparable* verdict (`seam/src/selfref.rs`)

The danger here is new in kind: a value whose **address** is load-bearing. A `SelfRef` holds a `u64`
and a `*const u64` that, after `init`, points at its own `value` field. Reading through that pointer is
sound **only if the struct never moves after `init`** — a move relocates `value` but leaves the pointer
aimed at the old address, a dangling read. None of the nine confirmed axes is about a value's address,
so move-stability is a *genuinely distinct* danger.

The floor's witness is **`Pin`**: a `Pin<&mut Self>`/`Pin<&Self>` over a `!Unpin` cell (here via
`PhantomPinned`) statically forbids the move — every move path (`swap`/`replace`/`get_mut`/return) routes
through a capability that needs `Unpin`, which `PhantomPinned` denies. So the seam is **sound** (the
panel conceded it unanimously; the dangling move is unconstructible in safe code).

And yet — this is the project's most novel verdict — move-stability is **not a clean tenth axis**. The
cold panel ruled it **real but structurally *inseparable* (N stays 9)**, on three grounds:
- **It cannot ace API-locality.** `Pin<&Self>` is the viral-*restrictive* wrapper: it infects every
  receiver **and revokes `&mut`** — and that revocation *is* the soundness mechanism. You cannot have the
  safety without the type-surface cost; there is nothing to isolate.
- **Its cost is *non-uniform* across witnesses.** Stack-pin (`pin!`) spends **API-locality**; `Box::pin`
  spends **bare-metal**; a hypothetical `?Move` immovable type would spend an ambient bound. One danger,
  several different costs — so it is not one mapped axis "wearing a hat" (the `typed_nat` confound) either.
- **So the honest cost is *immovability*** — the revocation of the move/return/store capability, heavier
  than API-locality (a merely leaked type surface). Move-stability funds itself by *being* immovable.

Two caveats the panel insisted on (and they sharpen the result rather than soften it): the inseparability
is **contingent, not necessary** — RFC 2349's *rejected* `?Move`/`Move` immovable types *were* a designed
free-standing witness; Rust **chose** hookless `memcpy` moves + a library `Pin`. And the crisp `core`-only
reading is the isolation of a **non-composable single-frame toy**: any real intrusive node / `Future` /
slab entry that move-stability exists to serve forces `unsafe` pin-projection or `Box::pin`. So this is a
*fourth* verdict mode, beyond {reject, confirm, confirm-and-distinguish}: **a real danger with no clean
coordinate** — the lattice's first inseparable entry. N stays 9.

### Primitive 12 — validity-checked bytes→enum → the basis is CLOSED (`seam/src/validity.rs`)

Eleven primitives in, a different question presses: are we *finding* axes, or just enumerating them
forever? Every danger so far either landed on a clean axis, a confound, or the inseparable category — but
maybe the next danger adds a tenth coordinate, and the one after that an eleventh, with no end. The
twelfth primitive is built to test that directly: **is the cost-basis closed?**

Its danger is genuinely new — a **validity invariant**. A `#[repr(u8)]` enum `Tag {A=0, B=5, C=200}` is
one byte, but only the bytes `0,5,200` are valid; `transmute::<u8,Tag>(7)` is **UB the moment it exists**
(no dereference — the compiler may assume a `Tag` is always a valid discriminant, e.g. an exhaustive
`match` with no fallthrough). No prior axis is about a value's *bit pattern*. The seam reads a `Tag` out
of a byte buffer via `transmute`-after-check; sound because the check (set membership `b ∈ {0,5,200}`)
precedes the unsafe. *(A teaching aside from the real run: the first two gauntlets **FAILED** — the first
on a genuine bug, a *range* check `b <= 200` that launders the non-contiguous bytes `3,4,6..199` into
invalid enums; the second on a fix the builder propagated incompletely, leaving a stale `b <= 2` in
another section. The adversary earned its keep to the very last primitive. The third, fully-corrected,
survived.)*

And the verdict is the **capstone of the whole arc**: *unanimously*, this brand-new danger class adds
**no tenth axis**. Every witness for it spends a coordinate *already on the map* — a single checked read
is a runtime tag check (**zero-cost #6**, the slab's axis via a different predicate), and the zero-copy
`&[u8] → &[Tag]` view that systems code actually wants is either eager-O(n) (#6) or a sealed constructor
(**API-locality**). (That non-uniformity also makes validity a *second* inseparable danger, after
move-stability — so inseparability recurs.) This is the **dual** of the `get_unchecked` family: there
*one* danger had *many* witnesses spending *different* axes; here *many* dangers (stale-key, bit-validity)
converge on the *same* axis. Read together, the two directions say the cost-space is a **small fixed
basis** that dangers *project onto*. The fifth verdict mode — **"real, but it adds nothing new"** — is the
one that *closes* the basis.

So the constructive arc concludes: the Seam Trilemma is a **3-of-N projection of a closed object** —
**nine clean axes**, an **inseparable category** (move-stability, validity), and **two outside-the-axes**
(the MMIO TCB axiom, the intrusive horn-drop). *"What does the safe witness cost?"* has finitely many
answers. (Honest caveats the panel logged: validity is sound only in the fieldless single-byte corner —
data-carrying enums need a recursive `is_bit_valid` — and the whole thing is non-novel, the shipped
design of `num_enum`/`bytemuck`/`zerocopy`. The bound stays a *lower* bound; but a brand-new danger
projecting cleanly onto the basis is what closure looks like.)

### Primitive 13 — a constant-time secret → the closure is PER DOMAIN (`seam/src/secret.rs`)

The closed basis answers "is the axis-list finished?" *within memory safety*. One question remains: is
that closure **absolute**, or is "memory safety" itself just one *domain*? The thirteenth primitive tests
it by leaving the domain entirely. Its danger is **not UB** at all — it is an **information-flow / timing
leak**: a naive `a == b` over secret bytes *short-circuits at the first mismatch*, so its **running time**
reveals the matching-prefix length — a byte-by-byte key/MAC recovery attack. No memory is corrupted; the
Rust Abstract Machine, which models no time, sees nothing wrong.

`Secret<N>` re-imports comparison as **constant-time**: it omits `PartialEq`/`Eq`/`Index`/`Deref` (so you
*cannot* branch on a secret — CF1), and vends only branchless ops (`ct_eq → Choice`, `conditional_select`)
with one loud exit, `Choice::expose_bool`. It is the `subtle` crate's pattern. Two things make this seam
unlike the previous twelve, and both are the point: there is **no `unsafe` keyword** (the danger lives in
a non-functional property the AM ignores), and constant-time-ness is **not guaranteed by the compiler**
(LLVM may re-introduce a branch).

The cold panel's verdict is the project's most generative. First, the seam **BREAKS** in the
information-flow sense — the exact analog of the intrusive list's memory-safety horn-drop:
`secretA.ct_eq(&secretB).expose_bool()` then `if b { … }` **type-checks** with only the vended ops and
leaks a secret *relationship* through control flow. The type forecloses branching *on `Secret`*, but
`expose_bool` returns an **untainted `bool`** you can then branch on — so "declassify once, at the end" is
**etiquette, not type-structure**. Closing it would need a whole-program, **undecidable** non-interference
analysis (a full IFC type system — Jif, FlowCaml). *A concentrated, decidable seam cannot soundly enforce
non-interference* — Horn 2 passes only because the real property is never checked. **The Seam Trilemma
recurs in the new domain, with a sharper bite.**

Second — and this is the generalization — the panel ruled information-flow **out of frame**: a *different
soundness domain* with its **own basis**. So "the cost-basis is closed" refines to **closed *per
domain***. Two attackers independently surfaced **declassification discipline** (which / where /
how-often a secret-derived bit may re-enter control flow — exactly the hole that broke this seam) as a
genuine IFC-domain axis with **no memory-safety analog**, alongside termination and concurrency channels.
The build-and-break *method* transfers — its very first IFC seam reproduced the trilemma and surfaced a
new axis — but the *coordinates* are the new domain's. (Honest correction the panel forced: an earlier
draft called the trust-the-codegen assumption "MMIO-shaped / irreducible"; it is in fact **dischargeable**
— CompCert-CT, Jasmin, and FaCT *formally preserve* constant-time — a contingent artifact of stock
`rustc`/LLVM, a different category than MMIO's permanent axiom.)

**So the deepest form of the result is not "the basis is closed" but "the Seam Trilemma and its cost-basis
are *domain-parameterized*."** Memory safety is one domain — mapped, basis closed, nine axes. Information
flow is another — its own basis (declassification, …), its own sharper trilemma.

### Primitive 14 — capability-gated declassification → a closing seam needs a matching FLOOR (`seam/src/labeled.rs`)

The secret seam established information-flow is a *different domain* — but it *broke*. One question
completes the picture: does the IFC domain have a **closing** seam (its own SPSC / slab), confirming it is
a real domain with a real cost-basis, not just a place where things break? The fourteenth primitive tries
to build one — and its failure is the project's deepest single result.

The secret seam broke because declassification was *etiquette*. The fix is the exact analog of how every
memory-safety seam works — you don't get *"no `unsafe`"*, you get *concentrated, checked `unsafe`*; so
here you don't get *"no declassification"*, you get **capability-gated** declassification: `Secret<T>` has
no read-out except `declassify(self, cap: Cap)`, and `Cap` is **affine** (`!Copy`/`!Clone`, minted only at
the trust boundary by `Cap::issue()`). Now *where/when/how-often* a secret is released is type-structural,
not convention — exactly the gap that broke the secret seam, closed.

And yet the cold panel **broke it too, unanimously (5/5)** — for two reasons that *together* are the
finding. (1) To **compute** on a secret, `map(self, f: FnOnce(T) -> U)` must hand the closure the **bare
plaintext** `T`; a safe closure with captured `&mut` state exfiltrates it — `s.map(move |x| sink.push(x))`
— with **zero caps and no `declassify` site**. The affine cap "guards a door while the wall beside it is
open." Unpatchable without an **effect system** Rust's floor disclaims. (2) The "budget" counts **uses,
not bits**: `Cap::issue()` in a loop mints N caps (statically unbounded — Rice), and one cap can release a
megabyte. What the cap *does* enforce — release *location* — **projects onto the memory-safety affine
axis** (it is one-shot privilege, not a new coordinate); the genuinely-IFC coordinate is the *amount*
(**quantitative information flow** — Smith min-entropy / g-leakage), and *an affine type system cannot
bound bits, only uses*.

So **both information-flow seams break, for one structural reason: Rust's affine/borrow floor carries
*none* of the IFC precondition.** And that is the deepest result of the whole project — not about
information flow specifically, but about the relationship between a floor and a domain:

> **A closing seam exists only when the floor *matches* the domain.** Rust's affine floor is *matched* to
> memory safety — where SPSC closes for *free* (the floor's affinity/NLL carry the precondition) and the
> basis is closed — and *mismatched* to information flow — where *no* seam closes, because affinity tracks
> *resource uses*, while IFC needs *information amount*. To get closing seams in the IFC domain you need a
> *different floor*: a quantitative / effect-tracking type system (Jif, FlowCaml, QIF).

So the final shape is three layered claims. **The Seam Trilemma is domain-general** — it recurs in every
soundness domain. **The cost-basis is per-domain** — memory safety has its nine closed axes; information
flow has its own (declassification, quantitative leakage). **And which seams close depends on the
floor-domain match** — a floor closes a domain's seams only when it carries that domain's precondition.
Memory safety: mapped, on a matched floor. Information flow: a real, distinct domain whose seams all break
on *this* floor — pointing at the floor it would need instead. *That* is where the constructive arc ends:
not at a wall, but at a border, with the next country named.

### The taxonomy (the whole point, in one table)

| Primitive | What the missing conjunct *is* | Fate | Pays with |
|---|---|---|---|
| SPSC queue | floor-expressible (affinity/NLL/`Send`) | **closes** | nothing — every axis ✓ |
| Slab | runtime-checkable (generation) | **closes** | zero-cost (#6) |
| Branded index | cross-object identity (generative brand) | **closes** | API-locality; brand is illegible (#7); #6 deferred |
| Refined index | in-bounds via a legible predicate (SMT) | **closes** | solver-freedom (horn 2); R2 trade — brand-free *xor* zero-cost |
| Shared (Rc) | dynamic shared ownership (refcount) | **closes** | bare-metal (#5) — needs a heap; bare-metal↔capacity trade |
| Type-level index | in-bounds via a compile-time trait proof | **closes (sound)** | a `recursion_limit` cost inside horn 2 — but **no clean axis** (confounded + encoding-relative) |
| Take-and-replace | transient broken invariant across an unwind | **closes** | **unwind-safety (8th axis)** — via a discharge menu (abort / repair / poisoning) |
| Drain-and-yield | a load-bearing guard `Drop` on a leak-permitting floor | **closes** | **leak-freedom (9th axis)** — pre-pessimize; a leaked guard loses the tail (distinct from unwind-safety) |
| Self-ref cell | a value whose *address* is load-bearing | **closes (sound)** | **move-stability — INSEPARABLE** (immovability; non-uniform: stack-pin→API-locality, heap-pin→bare-metal). A distinct danger with **no clean axis**; N stays 9 |
| Validity (bytes→enum) | a value whose *bit pattern* must be valid | **closes (sound)** | **PROJECTS ONTO THE BASIS** — per-read → #6, zero-copy view → API-locality. **No new axis** (2nd inseparable). The closed-basis proof; N stays 9 |
| MMIO | external physical axiom | **sound modulo TCB** | a trusted `assume` |
| Intrusive list | aliasing+liveness that escapes to the caller | **breaks** | a trilemma horn |
| Constant-time secret | branching on a secret leaks it (timing — **not UB**) | **breaks (IFC sense)** | **DIFFERENT DOMAIN** — information-flow, out of frame. The trilemma recurs; closure is *per domain* (its own basis: declassification discipline, …) |
| Capability-gated declassify | release of a secret must be *bounded* (where/when/how-much) | **breaks (IFC sense)** | **FLOOR MISMATCH** — `map` hands a closure bare plaintext (needs an effect system); the affine cap counts *uses*, not *bits* (needs QIF). No IFC seam closes on Rust's floor; affinity bounds *resource uses*, not *information amount* |

The decontamination signal (Part 6) held across all twelve **memory-safety** primitives: the obstruction
*vanished* (SPSC), *moved and stayed decidable* (MMIO), *moved onto a runtime witness* (slab), *moved onto
the type surface* (branded index), *moved into the trusted toolchain* (refined index — into the SMT
solver), *moved into a runtime subsystem* (shared — into the global allocator), *moved onto the resolver's
fuel budget* (type-level index — but that site proved *not* to be a clean axis), *moved onto the unwind
path* (take-and-replace — a mid-take panic becomes an abort; that site *was* a clean axis, the eighth),
*moved onto the leak path* (drain-and-yield — a forgotten guard loses the tail; the ninth axis, distinct
from the eighth), *moved onto immovability* (self-ref cell — a `Pin`-revoked move capability; a real
danger but with no clean coordinate, the inseparable category), *moved onto an already-occupied axis*
(validity — bit-validity → #6 or API-locality, adding nothing new; the closed-basis signature), or *moved
and broke soundness* (intrusive) — and **where it moves is the entire verdict.** The thirteenth, the
constant-time secret, *moved out of the memory-safety domain entirely* (onto a timing channel the AM does
not model) and **broke** there — the same decontamination logic, applied in a new domain, locating a
*different* basis. The fourteenth, capability-gated declassification, *tried to close that new domain* on
Rust's floor and **broke for a structural reason**: the obstruction has nowhere to move *to*, because the
floor carries none of the IFC precondition — `map` must hand the closure bare plaintext (the contaminant
escapes uncaught) and the affine cap counts *uses* where the danger is *bits*. So the decontamination
signal completes its own statement: **the obstruction vanishes only when the floor matches the domain.**
On a matched floor it moves somewhere nameable (every memory-safety primitive); on a mismatched floor it
has no landing site and the seam breaks (both IFC primitives) — pointing at the floor that *would* carry it
(Jif / FlowCaml / QIF).

---

# Part 10 — Mechanization and honest bounds

The trilemma's logical skeleton is machine-checked in Lean 4 (`proof/SeamGap.lean`,
`propext`-only, no `sorry`). The core characterization:

> **A seam closes if and only if its soundness precondition factors through the safe witness.**
> (`seam_closes_iff_factors`.)

Instantiated: `spsc_seam_closes` (precondition = witness → closes); `intrusive_seam_does_not_close`
(precondition is the gap → no closing decision exists); the slab pairing
`slab_runtime_closes_but_static_does_not` (the *same* precondition closes through a *runtime*
witness but not a *static* one — the zero-cost axis, made formal at the model level); and
`branded_seam_closes`/`refined_seam_closes`/`shared_seam_closes`/`typed_nat_seam_closes`/`take_seam_closes`/`drain_seam_closes`/`selfref_seam_closes`/`validity_seam_closes`
(all close *indistinguishably from SPSC* in the kernel — which is the point: the branded index's viral
type-parameter cost, the refined index's solver dependency, the shared seam's heap requirement, the
type-level index's `recursion_limit` cost, the take seam's abort-on-panic, the drain seam's leaked-tail,
the self-ref cell's immovability, and the validity seam's bit-check are alike invisible to a kernel that
sees only whether a Boolean witness suffices — which is *exactly* why the axis claims had to be
adjudicated by the cold panel in prose, not the kernel: the kernel certifies that all of
`typed_nat`/`take`/`drain`/`selfref`/`validity` *close*, and says nothing about the fact that one spends a
confounded non-axis, two spend clean distinct axes, one spends a real-but-inseparable danger, and one
spends a coordinate *already on the map* — the last of which is how you know the basis is closed). The
last two, the information-flow primitives, are the sharpest illustration: their Lean instances are named
`secret_typed_discipline_factors` and `labeled_typed_discipline_factors`, *not* `..._closes`, because the
kernel models neither time nor non-interference nor quantitative leakage — each certifies only that the
*typed* discipline factors, while the cold panel ruled both real seams **break** in a domain the kernel
cannot see at all (the secret out of frame, the labeled on a mismatched floor).

**The honest bound — and why it matters as a lesson.** A cold adversarial review of the proof
returned the right verdict: the *checked* math is a real characterization, but the *faithfulness*
of "precondition" and "witness" to actual Rust primitives still lives in the **prose modeling
dictionary**, not the Lean kernel. Lifting that into the kernel is a RustBelt/Iris-scale,
months-long effort. The integrity move is to **ship the adversary's verdict alongside the proof**
and to never let the word "machine-checked" transfer more confidence than the *checked part*
earns. This is a transferable habit: when you mechanize something, be precise about *which*
claim the machine actually checked.

---

# Part 11 — The transferable meta-lessons

You can forget every primitive and keep these.

1. **Conservation of essential difficulty (Brooks).** Fred Brooks split complexity into
   *accidental* (incidental to your tools, removable) and *essential* (inherent to the problem,
   not removable). Memory-safety-with-aliasing-and-no-GC has irreducible essential difficulty;
   you can *relocate* it (to a borrow checker, a runtime flag, a proof, a trusted axiom) but not
   *delete* it. When a design promises to make essential difficulty vanish, look for where it
   actually went.

2. **Trilemmas are the signature of a real boundary.** A "pick 2 of 3" where every *pair* works
   but the *triple* doesn't is a stronger signal than a flat "impossible." It tells you the
   constraints genuinely pull in different directions. Compare CAP (consistency/availability/
   partition-tolerance), the macro "impossible trinity," FLP. The companion essay
   (`THE-SAME-WALL.md`) develops a **falsifiability test** distinguishing these real boundaries
   from fake ones: a deletable seam (Maxwell fusing electricity and magnetism) is a *veil over a
   latent unity*; a permanent one (decidability vs expressiveness) is a *boundary between opposed
   constraints*. The test: does merging reveal the two were the same thing (→ delete) or force a
   choice neither side can make (→ relocate)?

3. **Name your seam, and shape it deliberately.** Every nontrivial system has a boundary between
   a cheap/checked world and a trusted/proven one. The amateur move is to pretend it isn't there;
   the professional move is to (a) put it in one place (concentration), (b) make the obligation
   at it as visible as possible, and (c) choose *consciously* whether the boundary costs you
   soundness, decidability, concentration, runtime, API surface, a solver in your toolchain, or
   exception-safety, leak-freedom. The fourteen-primitive taxonomy is a decision procedure for that choice.

4. **Adversarial separation of build and break.** Construction wants continuity; demolition wants
   a cold, context-free skeptic. Keeping them as separate machines — and *calibrating* your
   skeptic on known-good and known-bad inputs before trusting it — is a method you can apply to
   code review, design review, and your own reasoning. A reviewer who never rejects anything (or
   rejects everything) is uncalibrated and worthless; prove yours isn't.

5. **Decontamination beats argument.** When something fails and you can't tell whether the *idea*
   is impossible or your *attempt* was weak, hand the attempt a stipulated known-good component
   and watch whether the failure *moves* or *vanishes*. Moving = structural; vanishing = you were
   just bad at it. This single discriminator is worth more than a week of debate.

6. **A bundled property hides axes; the witness that splits them proves they were there.** "Pick 2
   of 3" framings are clean, but a horn is often a *bundle* — "concentrated & proof-free" hid zero
   runtime cost; "concentration" hid the difference between code crossings and type infection; even
   "decidable" hid the difference between a terminating checker and a solver in your TCB. You
   discover a folded axis exactly one way: exhibit two artifacts the framing calls identical that
   *measurably differ* on the candidate (the slab vs. SPSC on runtime; the branded index vs. the
   slab on the type surface). And mind the geometry: to prove two axes *independent* you need an
   **off-diagonal** — two artifacts that agree on one axis and flip the *other* (the refined index
   and the slab are both brand-free but differ on legibility; a single brand-free-and-legible point
   would not have done it). Until you have that witness, a "new axis" is a hunch; with it, the
   dimension is real. Your tidy trilemma may be a shadow — look for the object casting it. **But the
off-diagonal must be *clean*:** the eighth probe (the type-level index) *looked* like it flipped only
checker-tractability against SPSC — and was rejected because it silently *also* flipped expressiveness
(it indexes only compile-time constants). An off-diagonal that moves two coordinates at once proves
nothing. Check that everything *except* the candidate is genuinely held fixed; if it can't be (here,
intrinsically — a runtime index cannot ride a compile-time proof), the axis is not isolable by that
witness, and the honest move is to drop the claim.

7. **Deflate your own discoveries: "new to me" ≠ "new."** *Every* axis the project's adversary
   confirmed as real and independent it also proved *not novel* — solver-freedom is refinement-types'
   founding tradeoff (DML, 1998); bare-metal is Rust's `core`/`alloc` boundary (RFC 2480); unwind-safety
   is Abrahams' exception-safety guarantees (and RFC #1508 had already rejected the very `take_mut` that
   witnesses it). Each was a genuine dimension of *this* lattice and each already had a name in the
   literature. The discipline: before claiming a discovery, ask "what is this *already called*?" The
   honest, larger result was not "nine new axes" but "nine *known* PL design axes are all faces of
   one tension." A unification you can name is worth more than a dimension you can't — and a cold
   reviewer who deflates your count is doing you the biggest favor in the room. (One subtlety the ninth
   axis added: "deflatable" is not "not an axis." Leak-freedom is the *thinnest* axis — its cost is
   invisible in all non-leak code — yet still real, a populated off-diagonal. Thinness is a property of
   an axis, not a disqualification from being one.)

8. **A measuring instrument must return "no", "yes", the right *name*, the right *boundaries*, and "real
   but unmeasurable".** The axes past seven took four probes, and together they exercise every mode an
   instrument needs. On the first (a type-level index pitched as *checker-tractability*) the adversary
   returned **no**: it threw the axis out — confounded with expressiveness, the cost a unary-encoding
   artifact, the name wrong. On the second (take-and-replace, *unwind-safety*) it returned **yes, but** —
   confirmed a clean axis *and* corrected the label (it is the exception-safety-*guarantee-level*; abort
   is one discharge of it). On the third (drain-and-yield, *leak-freedom*) it returned **yes, and here's
   why it doesn't merge** — confirmed a clean axis *and* ruled it distinct from its tempting sibling
   unwind-safety (leak = `Drop` never runs; unwind = `Drop` runs at a bad time). On the fourth (a
   self-referential cell, *move-stability*) it returned the subtlest answer of all — **yes, this danger
   is real and distinct, but it has no isolable cost**: every witness for it spends a *different other*
   axis (stack-pin → API-locality, heap-pin → bare-metal), so it is neither a clean coordinate nor a
   confound. A procedure that only ever confirms is a confirmation machine; one that only ever rejects is
   a cynic; one that confirms but can't tell two neighbors apart is a blurry lens; one that calls every
   real danger a "dimension" is an over-counter. And on the fifth (a validity-checked read) it returned
   the answer that *ends the search* — **yes, this danger is real and brand-new, but it adds nothing to
   the list; its cost is a coordinate you already have**. This one did all of it — reject, confirm,
   relabel, distinguish, recognise-the-inseparable, *and certify-that-a-new-thing-adds-nothing* — which is
   exactly why you can trust the nine axes it reports (and the dangers it honestly declined to make a
   tenth). Seek the input that makes your method return *false*, the input that makes it confirm *the
   wrong thing*, the input that tempts it to *merge two different things*, the input that is *real but
   cannot be cleanly measured*, and the input that is *new but redundant*; the value is in those five gaps.

9. **A measuring system is finished when a genuinely new input lands on an old coordinate.** The whole
   constructive arc was a search for axes, and the honest way to *end* it was not to run out of energy
   but to find a stopping criterion. That criterion is the **closed basis**: when you deliberately feed
   in a danger from a category you have never touched (here, bit-level validity — no prior axis was about
   a value's bit pattern) and the cost it pays is *already on your map*, you have evidence the map is
   complete. It is the dual of the discovery move — there, *one* danger (out-of-bounds access) needed
   *many* witnesses that each spent a *different* axis (so the axes were really distinct); here, *many*
   dangers (a stale handle, an invalid bit pattern) spend the *same* axis (so the axis-list is really
   finite). Neither direction is a proof — the bound stays a *lower* bound, a future danger could still
   surprise — but "a new danger class projecting cleanly onto the basis" is what closure *looks like*,
   and naming that as a result is more honest than either claiming completeness or probing forever. When
   your taxonomy stops growing under new inputs, say so, and stop.

10. **A closed basis is closed *within a domain* — name the domain, then look for the next one.** The
    strongest move after "stop" is to ask what you were *implicitly* holding fixed. The whole nine-axis
    basis answers **"what does a sound *memory-safety* witness cost?"** — and *memory safety* was the
    silent parameter. Feed the method a danger from a *different soundness domain* — **information flow**,
    where the danger is a timing leak, not UB — and three things happen at once: the Rust Abstract Machine
    sees *nothing* (it models no time), so "can safe code reach UB?" is the wrong question; the seam
    **breaks** (sound non-interference needs whole-program undecidable analysis — the trilemma recurs);
    and a *new* axis appears with no analog in the old domain (**declassification discipline**). The
    lesson: an impossibility result and its cost-basis are usually **domain-parameterized**. The Seam
    Trilemma is the same *shape* in memory safety, information flow, and (one expects) real-time, liveness,
    or distributed consistency — but a *different basis* in each. So "the basis is closed" is never the
    last word; "**closed in *this* domain; here is the next domain, and the method transfers**" is.

    **And one refinement deeper — the floor-domain match.** Asking the obvious follow-up — does the new
    domain have a *closing* seam, the way memory safety has SPSC? — the fourteenth primitive (a
    capability-gated declassification, the IFC analog of "concentrated checked `unsafe`") tried to build
    one and **broke, structurally**: to compute on a secret you must hand a closure the bare plaintext (the
    contaminant escapes uncaught — needs an effect system), and an affine capability counts *uses* where
    the danger is *bits* (needs quantitative information flow). The reason is general: **a closing seam
    exists only when the *floor* carries the *domain's* precondition.** Rust's affine/borrow floor is
    *matched* to memory safety — affinity *is* the cardinality precondition, so SPSC closes for free — and
    *mismatched* to information flow — affinity bounds resource uses, not information amount, so *no* IFC
    seam closes on it. The full shape is therefore three-layered: the trilemma is **domain-general**, its
    cost-basis is **per-domain**, and *which seams close* depends on the **floor-domain match**. The most
    honest way to finish a map is to draw its border, point past it — *and* name the floor the next country
    needs (here: Jif / FlowCaml / a QIF type system).

---

# Part 12 — Do it yourself: exercises and a reading path

**Run the artifacts (30 minutes).**
- `cd seam && cargo test` — read `tests/*.rs` (the sound uses) and the `compile_fail` doctests
  in `src/*.rs` (the laundering attempts the floor rejects). Try to *make one compile* — feel the
  borrow checker push back.
- `lean proof/SeamGap.lean` — read the characterization; change a definition and watch it break.
- Skim `design_seam/REPORT_SEAM_SPEC_*.md` — the cold attackers' actual verdicts, in their words.

**Build a fifteenth primitive — but in a *new domain*, and on a *matched floor*.** The memory-safety basis
is closed: its six verdict outcomes are all attested — a clean axis, a confound, a mislabel, a
distinctness-ruling, an inseparable, and a plain projection — so a within-memory-safety probe (a typed
atomic cell, a bump allocator handing out overlapping views) will almost certainly *project*, not extend.
The genuinely open frontier is the one Primitives 13–14 opened: **a different soundness domain — and the
floor that matches it.** Primitive 13 showed information flow is a *distinct* domain; Primitive 14 showed
*no IFC seam closes on Rust's affine floor* (affinity counts uses, not bits). So the next move has two
halves. (i) Pick a danger whose "UB" is *not* memory corruption — a **timing/cache side-channel**, a
**liveness / deadlock-freedom** violation, a **real-time / WCET** miss, a **distributed-consistency**
anomaly. (ii) Pick a **floor that carries its precondition** — for information flow a quantitative/effect
type system (**Jif, FlowCaml, or a QIF-typed DSL**); for real-time a WCET-annotated one; and *there* try to
close a seam. Write the spec + a checked toy, and run the *same* harness (`python3 -m
orchestrator.seam_phase --spec ... --gauntlets 1 --prior-art-web`). Predict four things, because the two
information-flow probes got them all: (a) the Rust Abstract Machine sees *nothing* (it models neither time
nor schedules), so "can safe code reach UB?" is the wrong question; (b) the Seam Trilemma *recurs* — sound
+ decidable + concentrated, pick two — with the domain's own bite; (c) the domain has its *own* axes
(information flow's were **declassification discipline** and **quantitative leakage**) with no
memory-safety analog; and (d) **a seam closes only if your floor matches the domain** — on Rust's floor
every IFC seam breaks. The deliverable is no longer "a tenth axis" but **"the next domain's basis, the
floor that closes it, and confirmation the method transfers."** That is what newlang's last two primitives
showed is possible — and what they leave for you. (See `design_seam/REALIZABILITY.md` for the
memory-safety lattice, the closed-basis capstone, the per-domain generalization, and the floor-domain
refinement.)

**Reading path, in dependency order:**
1. *The Rust Book*, ownership/borrowing chapters — the affine system in practice.
2. *Programming Language Foundations* (Pierce et al., *Software Foundations* vol. 2) — soundness
   (progress + preservation) done properly, mechanized.
3. The **RustBelt** paper (Jung et al., POPL 2018) — the semantic-soundness idea; skim the Iris
   parts, absorb the *unsafe-is-justified-not-checked* thesis.
4. **GhostCell** (Yanovski et al., ICFP 2021) — the branding trick, and why it carries aliasing
   but not lifetime.
5. **Generativity, end to end** — Launchbury & Peyton Jones, "Lazy Functional State Threads"
   (PLDI 1994, the `runST` brand); Kiselyov & Shan, "Lightweight Static Capabilities" (PLPV 2006,
   branded indices for bounds-check elision, two decades early); Beingessner ("Gankra"), *You Can't
   Spell Trust Without Rust* (MSc thesis, Ch. 6.3); bluss's `indexing` crate. This lineage *is* the
   fifth primitive, and the source of its API-locality cost.
6. **Refinement types** — **Flux: Liquid Types for Rust** (Lehmann, Geller, Vazou, Jhala, PLDI 2023,
   the brand-free predicate witness *on Rust's own floor*) and **Liquid Haskell** (Vazou et al., ICFP
   2014). This pair *is* the sixth primitive, and the source of its solver-in-the-TCB cost — the
   founding tradeoff named back in DML (Xi & Pfenning 1998).
7. **Reference counting and the `core`/`alloc` line** — Rust's **RFC 2480** (the `alloc` crate split,
   stable 1.36) and the **`heapless`** crate (core-only, fixed-capacity containers incl. a pooled
   `Rc`). This pair *is* the seventh primitive's axis: the bare-metal boundary, and the proof that
   shared ownership trades *unbounded dynamic allocation* against *caller-provided bounded storage*.
8. **Verus** and **Creusot** docs — what "Rust but `unsafe` is proof" actually looks like when
   someone ships it (the productive horn from Part 8).
9. This repo's `WRITEUP.md` (the full result), `design_seam/SYNTHESIS.md` (the fourteen-primitive map),
   `design_seam/REALIZABILITY.md` (the face lattice / 3-of-N proof), and `THE-SAME-WALL.md` (the
   cross-field boundary/veil discipline).

**The one thing to remember.** Decidable, sound, expressive, seamless, zero-cost — pick your
sacrifices on purpose. The seam isn't a failure of your design; it's the place your design tells
the truth about which difficulty it chose not to delete.
