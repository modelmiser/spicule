# newlang — a tutorial on the impossible language, and the PL theory you need to read it

**Audience.** A mid-level career developer: you write production code, you've used Rust or
C++ or Go, you know what a segfault is and roughly why it happens, but you have not formally
studied programming-language theory. By the end you will understand (a) the actual PL theory
this project rests on, from the ground up; (b) the state of the art in "safe systems
languages"; and (c) what newlang discovered and *why it matters*.

**What newlang is.** Not a language. It is a research project that asked "what would the
perfect systems language look like?", listed seven properties such a language should have,
and then **proved — adversarially — that the seven cannot coexist**, mapped exactly where the
wall is, and then **built four real things that live right up against the wall** to show what
you *can* do. The deliverables are an impossibility result, a theorem (the *Seam Trilemma*),
a Lean proof, and a small Rust crate of worked examples.

**How to read this.** Parts 1–5 are the PL bootstrap — the prerequisites, taught from
scratch, with the state of the art woven in. Parts 6–10 are the project itself. Part 11 is
the transferable meta-lessons. Each part stands alone; skip what you know. Code references
point at real files in this repo (`seam/`, `proof/`, `design_seam/`).

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
this exact move pay off four more times in Part 9.)

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

# Part 9 — Build and break: five real seams (the heart of it)

Theory in hand, the project stopped arguing and *built*. Crucial design of the experiment:
**build and break are different machines.** Construction is continuity, so a *stateful builder*
(a normal, context-holding process — here, me) writes the smallest thing that compiles.
Demolition is coldness, so the cold harness is the **red team**, aimed *only* at the re-import
rule. Each seam ships as a **text spec** (what the cold attackers attack) *and* a real,
`rustc`-checked crate (`seam/`) whose consumer code is `#![forbid(unsafe_code)]` and whose UB-
laundering attempts are `compile_fail` tests. Five primitives: the first four chosen because their
preconditions have four *different shapes*, the fifth because it reopened a deeper question — *how
many axes is the trilemma actually a projection of?* The headline finding:

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

This primitive forced the **first** of two un-bundlings:

> **The Seam Trilemma is a 3-of-4 projection.** "Concentrated & proof-free" had quietly bundled
> away *zero runtime cost*. The slab prises them apart: a *concentrated* seam can still be
> *non-erasable*. Seam width (horn 3) and zero-cost (#6) are independent axes — the slab aces the
> first and spends the second. And #6 was one of the casualties back in Part 7's unified result;
> the constructive phase puts it back on the map. *(The fifth primitive, below, forces the second
> un-bundling — and turns "3-of-4" into "3-of-N".)*

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

> **The Seam Trilemma is a 3-of-N projection, N ≥ 6.** The slab un-bundled zero-cost from
> "concentrated & proof-free"; the branded index un-bundles **"concentration" itself** into a
> *code* face (seam width) and a *type* face (API-locality). Beyond the three horns the constructive
> primitives have now exposed three more independent axes — zero-cost (#6), API-locality (#5), and
> AI-legibility (#7) — each proven real by a *witness that two trilemma-identical configurations
> differ*. They even have structure: within the branded index, #5 and #6 **trade** (range-branding
> buys erasure by deepening the brand tax). The full face lattice — which property-subsets have a
> witness, and the one still-unwitnessed claim (#5 ⟂ #7) — is `design_seam/REALIZABILITY.md`.

### The taxonomy (the whole point, in one table)

| Primitive | What the missing conjunct *is* | Fate | Pays with |
|---|---|---|---|
| SPSC queue | floor-expressible (affinity/NLL/`Send`) | **closes** | nothing — every axis ✓ |
| Slab | runtime-checkable (generation) | **closes** | zero-cost (#6) |
| Branded index | cross-object identity (generative brand) | **closes** | API-locality (#5); brand is illegible (#7); #6 deferred |
| MMIO | external physical axiom | **sound modulo TCB** | a trusted `assume` |
| Intrusive list | aliasing+liveness that escapes to the caller | **breaks** | a trilemma horn |

The decontamination signal (Part 6) held across all five: the obstruction *vanished* (SPSC),
*moved and stayed decidable* (MMIO), *moved onto a runtime witness* (slab), *moved onto the type
surface* (branded index), or *moved and broke soundness* (intrusive) — and **where it moves is the
entire verdict.**

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
`branded_seam_closes` (closes *indistinguishably from SPSC* in the kernel — which is the point: the
branded index's whole difference, the viral type-parameter cost, is invisible to a kernel that sees
only whether a Boolean witness suffices).

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
   soundness, decidability, concentration, runtime, or API surface. The five-primitive taxonomy is a
   decision procedure for that choice.

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
   runtime cost; "concentration" hid the difference between code crossings and type infection. You
   discover a folded axis exactly one way: exhibit two artifacts the framing calls identical that
   *measurably differ* on the candidate (the slab vs. SPSC on runtime; the branded index vs. the
   slab on the type surface). Until you have that witness, a "new axis" is a hunch; with it, the
   dimension is real. Your tidy trilemma may be a shadow — look for the object casting it.

---

# Part 12 — Do it yourself: exercises and a reading path

**Run the artifacts (30 minutes).**
- `cd seam && cargo test` — read `tests/*.rs` (the sound uses) and the `compile_fail` doctests
  in `src/*.rs` (the laundering attempts the floor rejects). Try to *make one compile* — feel the
  borrow checker push back.
- `lean proof/SeamGap.lean` — read the characterization; change a definition and watch it break.
- Skim `design_seam/REPORT_SEAM_SPEC_*.md` — the cold attackers' actual verdicts, in their words.

**Build a sixth primitive (a day).** Pick a primitive whose precondition has a *new* shape — a
typed atomic cell, a bump allocator handing out overlapping views, a `Pin`-based self-referential
struct — write the spec + a `rustc`-checked toy, predict which horn (or recovered axis) it hits,
and run it through the harness (`python3 -m orchestrator.seam_phase --spec ... --gauntlets 1
--prior-art-web`). **The sharpest open target is the one claim the project argued but never
witnessed:** the trilemma is now a 3-of-N projection (the slab recovered zero-cost #6; the branded
index recovered API-locality #5 and surfaced AI-legibility #7). But in the five-primitive corpus #5
and #7 still *co-vary* — the brand is both viral and illegible. A **refinement-typed seam** (a
Flux-style `i: usize{i < len}` bounds proof) should sit at *brand-free yet legible* — the
off-diagonal that would separate #7 from #5 for real. Build it; if the cold panel agrees it pays
neither #5 nor #7, you have promoted a sixth axis from "argued" to "confirmed." (See
`design_seam/REALIZABILITY.md` for exactly which cell is empty.)

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
6. **Verus** and **Creusot** docs — what "Rust but `unsafe` is proof" actually looks like when
   someone ships it (the productive horn from Part 8).
7. This repo's `WRITEUP.md` (the full result), `design_seam/SYNTHESIS.md` (the five-primitive map),
   `design_seam/REALIZABILITY.md` (the face lattice / 3-of-N proof), and `THE-SAME-WALL.md` (the
   cross-field boundary/veil discipline).

**The one thing to remember.** Decidable, sound, expressive, seamless, zero-cost — pick your
sacrifices on purpose. The seam isn't a failure of your design; it's the place your design tells
the truth about which difficulty it chose not to delete.
