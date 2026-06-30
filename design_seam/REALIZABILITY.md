# The realizability table — the Seam Trilemma is a 3-of-N projection

*Phases D–R. The constructive question the slab opened, the branded index extended, the refined index
closed, the shared-ownership seam pushed to seven, the take-and-replace seam to eight, and the
drain-and-yield seam to nine: is the Seam Trilemma the whole story (3 horns), or a low-dimensional
shadow of a larger tension? Answer: it is a **3-of-N projection with N ≥ 9** — where the recovered axes
are known PL boundaries, not novel discoveries (see the meta-pattern below). Each constructive primitive
lit up one more axis by being the **witness that two configurations the trilemma treated as identical
actually differ** — and the method's discrimination is now demonstrated in **nine** modes, not just
accumulation: it **confirmed** axes (cleanly deflated to pre-existing), **sharpened** a horn without
extending N (solver-freedom), **rejected** a confounded candidate (checker-tractability → not an axis),
found one **real but structurally inseparable** (move-stability → a distinct danger with no isolable
cost), and — the capstone — found a new danger class that **projects entirely onto the existing basis**
(validity → no tenth axis), which together with the recurrence of inseparability (validity is a second
inseparable danger) is the evidence that **the cost-basis is CLOSED** (see "The cost-basis is closed",
below). N stays 9; the closed basis is the conclusion of the constructive arc — **and three further primitives
generalized it: a constant-time secret showed closure is *per domain* (information-flow is a *different*
soundness domain with its own basis); a capability-gated declassification showed that *which seams close*
depends on the *floor-domain match* (Rust's affine floor closes memory-safety seams but *no*
information-flow seam, because affinity bounds resource *uses*, not information *bits*); and a type-level
lock ordering, probing a *third* domain (liveness/deadlock), showed the floor-domain match is *graded —
full / partial / none*: it lands *partway*, the static lock-order fragment closing while the dynamic and
per-thread fragments escape, so the method now discriminates in **eight** modes (adding *partial-match*).
And a sixteenth — a *runtime* lock registry (the floor-*upgrade* probe) — showed that **upgrading the
floor is NON-MONOTONE**: it fixed the static seam's *coverage* gap (dynamic orderings now close) and paid
the predicted price (bare-metal #5, runtime-check #6, a prevention→detection modality), yet **granularity
recurred one rung finer** — the held-set is keyed on the OS *thread* but async *tasks* are the unit of
control, so work-stealing migration launders an undetected inversion. The *granularity* axis is a
containment-scope **ladder** (token < thread < task < …) that no ambient Rust floor tops out. And a
seventeenth — a *must-run cleanup guard* — probed **breadth-wise** (a new *precondition*, "this finalizer
MUST run", not a new domain or ladder rung): it **breaks** on a clean **none** grade, sharpening the
match-unit from *domain* to *precondition* (the *same* liveness domain grades deadlock-freedom *partial*
but must-run-cleanup *none*) and adding the **ninth** method mode — *wrong-structural-logic*: the floor is
**affine** ("at most once") where the precondition needs **linear** ("exactly once"), so `mem::forget`
skips the finalizer and *no decidable price* buys it back (Boats' scoped-task trilemma, linear horn
rejected; the leakpocalypse) — distinct from a domain-mismatch. And an eighteenth — a *session-typed
channel* (a `Chan<P>` typestate walk) — was the **first** out-of-(memory-safety) probe to **hold with a
cost** rather than break: **SEAM-COSTS [horn 3]**, because protocol fidelity is a *composite* matched
**component-wise** (the components orthogonal — `mem::forget` cannot shorten `'a` to start a second
session, so completion cannot infect safety): the *order-safety* half **closes** (affine typestate, no
`root`-hole) while the *completion* half is an undischargeable horn-3 *concentration* cost (the #17
linearity gap, recurring at every `?`/early-exit). The illumination: the **same** `mem::forget` gap that
**broke** #17's horn 1 only **costs** here — completion was #17's *whole* soundness promise but is *one
component beside a sound core* here, so **which horn a linearity gap threatens depends on how the rule
*packages* it**, not on the gap. (Deflations, accepted: a single-handle *walk* not a binary session
[no duality in `core`/no-`unsafe`, E0499]; "usable" only for infallible straight-line protocols; and the
split is documented folklore — Jespersen–Munksgaard–Larsen 2015, ECOOP 2022 *Stay Safe Under Panic* — so
the contribution is *locating* it on the lattice.) N stays 9; the five broken seams are unchanged — this
one holds.** And a nineteenth — a *value-owning scope-restore guard* (the real `scopeguard::guard`) —
turned that "**which horn depends on packaging**" observation from an incidental cross-primitive coincidence
into a **deliberate, controlled probe**: it carries #17's *identical* un-compellable-`Drop` danger but wraps
it in #18's structure (a sound affine-ownership *core* beside which the restore is a side-obligation), and
the cold panel (r26) ruled **SEAM-COSTS [horn 3]** — the *same* `mem::forget` gap that **broke** #17's horn 1
only **costs** here. **The dial is real — but the panel BOUNDED the thesis, all accepted:** the move is
*confounded* (#19 also adds the value-ownership core #17 lacked, so the value-ownership model co-varies — a
better-controlled probe than the incidental #17-vs-#18 pair, not a single-variable manipulation); the horn
is *forceable* (the same skippable-`Drop` danger in a **borrowed-slot / `take_mut`** packaging forces
**horn-1 UB** — the #9 `take` danger — and `Pin`'s drop guarantee plus the `thread::scope` restructure
exhibit horns that **cannot** be repackaged, so *which* horn depends on **where safety rests** — partly
danger-*intrinsic*); and it is *not novel* (`JoinGuard` broke horn 1, `scopeguard` costs horn 3, concluded
in RFC 1066 in 2015 — so the contribution is the *method* and *locating the dial*, **no new mode**). The
refined finding: **which horn a danger threatens is a design choice *within a structural envelope* set by
whether soundness can be DECOUPLED from the un-compellable closure** (owned ⇒ costs; borrowed-across-unwind /
`Pin` / `thread::scope` ⇒ forces). And a twentieth — a *take-and-reseat guard over a **borrowed slot*** (the
real `take_mut`, in guard form) — ran the clean single-variable experiment Phase R's confound forbade: it
keeps #19's packaging **byte-identical** and flips **only** owned → borrowed, and the cold panel (r27) ruled
**SEAM-BREAKS [horn 1], FAILED — the *sixth* broken seam.** A 4-line *safe* `String` witness
(`new`/`push_str`/`into_inner`) double-frees with no `unsafe` in user code (`into_inner` returns the owned
value while the slot keeps the `ptr::read` duplicate — two owners). **Two findings, both decisive:** the
horn flip is *clean* (#19 owned+`into_inner` cost-3 → #20 borrowed+`into_inner` break-1, packaging
byte-identical — so Phase R's confound is **repaired** and the flip is attributable to the decoupling
property *alone*); and the fence is **FORCED, not movable** — the `into_inner`-less "sound" subset is
*degenerate* (it deletes the by-value move-out that *defines* `take_mut`, strictly dominated by plain
`&mut T`), so every *usable* packaging breaks, and the fence sits exactly at the **owned-vs-borrowed** line.
(Deflation, pre-registered: no novel mechanism — `take_mut` + `scopeguard`-`into_inner` + RFC 1066,
reassembled; contribution analytical.) Phase Q said the horn moves; Phase R says it moves *within a fence*;
Phase S proves the fence **real and FORCED** at owned-vs-borrowed, closing the packaging trilogy: **the horn
is movable iff the value-ownership model lets soundness DECOUPLE from the un-compellable closure** (owned
can ⇒ cost; borrowed cannot, for any *usable* packaging ⇒ forced break). N stays 9 verdict modes; **the
broken-seam count rises to six** (this one breaks). And a twenty-first — a *state-based CRDT merge*
(`Replica<T: JoinSemilattice>`, the first probe aimed at a **fourth** soundness domain, distributed/replicated
**convergence**) — found **no clean fourth domain there.** Floor-relatively the cold panel (r28) confirmed
**SEAM-BREAKS [floor-mismatch]** (the `T: JoinSemilattice` bound is a *syntactic proxy, not a witness* — a
safe non-commutative/non-idempotent `join` type-checks and **diverges replicas that saw the same updates**;
closing it is undecidable by Rice), but it scored a **TRADEOFF survival** and the candidate domain
**DEFLATED**: the gap is the *generic unchecked-typeclass-law phenomenon* — Rust checks the **signatures** of
`Ord`/`Eq`/`Hash`/`Monoid`/semilattice, **none of their laws** — so convergence **projects onto `restore`'s
application-correctness basis**, not a new domain. (Two corrections: the blessed-CRDT TCB rescue is *not
MMIO-shaped* — the lattice space is open-ended, so law-trust recurs per user CRDT, never amortizing; and
'dischargeable' holds only at the *model* level — no Rust pass checks model-to-shipped-code.) So the
soundness-domain map **stays at three** (memory / IFC / liveness) + the application-correctness basis, into
which convergence folds; the **FAILED broken-seam count stays six** (this one survived/deflated); N stays 9.**
And a twenty-second — a *cost-typed computation* (`Timed<C, T>`, a graded cost monad reusing `typed_nat`'s
type-level `Nat`) — was the **second** probe aimed at a fourth domain, this time **real-time / WCET**, and it
too **found no fourth domain — but it FAILED** where convergence merely deflated. The cold panel (r29) ruled
**SEAM-BREAKS [horn 1], FAILED — the SEVENTH broken seam**: `step` charges `+1` *decoupled from its closure*,
so a million-iteration loop is cost `1` and `within::<1>` admits it (my §4 over-claimed the deadline
guarantee — the spec defect that made this FAIL rather than survive). All five attackers landed: **Horn 3 is
false** (non-unifying `if`-branches force manual step-padding); the unary cost **does not scale** (O(C)
blow-up, uncompilable at real 10³–10⁷-cycle budgets); **no additive composition**; and the primitive is
**Danielsson's 2008 cost monad** — "op-count ≠ cycles" is WCET's *founding premise*, so **pick a lane**
(abstract cost xor physical deadline). WCET **deflates** multiply: the static cost monad onto Danielsson /
`typed_nat` (#8 — which does not even *scale*); the physical-deadline claim onto an `mmio`-shaped axiom (#2)
this seam **never installs** (worse than MMIO — the cost model is a fiction *and* absent); the data-dependent
gap onto liveness's **partial-match**. The op-count is an **unsound** proxy — *sharper* than `secret`'s
sound-but-incomplete one. **Meta:** two fourth-domain candidates (`crdt` deflated+survived, `wcet`
deflated+FAILED), **neither opens a fourth domain** — the three confirmed domains look **special**. N stays 9;
the FAILED broken-seam count **rises to seven**. Then a **liveness-deepening** probe — a **real binary session**
(two dual affine endpoints, both live, over `Rc<RefCell<Transport>>`; `binary_session`, r30) — removed #18's
single-handle limitation and **HOLDS WITH COST** (SEAM-COSTS [horn3], TRADEOFF): #18's component-wise grading
**survives** (order closes, *per-session* relative progress closes, completion costs horn-3, now
peer-observable), but the "real binary session" framing **deflated** — relative progress closes only **per
session** (global deadlock-freedom needs multiparty typing), the single-threaded witness makes "peer waits
forever" self-abandonment-flavored, and the fallible `recv`'s `Err` is **non-actionable** (the leakpocalypse
double-bind: `mem::forget` leaks the `Rc`). Liveness stays the third domain, now **deepened**; N stays 9; the
FAILED broken-seam count **stays seven** (this survived). Then **delegation** (channel-over-channel, the
higher-order session step #23 lacked; `delegation`, r31) — **SOUND** (SEAM-COSTS [horn 3], no UB, decidable)
**but the gauntlet FAILED on the §6 DELIVERABLE**: the method's **REJECTION mode** (2nd after `typed_nat` #8). My
"delegation adds no new coordinate — projects onto leak-freedom (#9/#10)" claim was **refuted from my own cited
lineage**: delegation's signature hazard (B forgets `Chan<Q>`, **starving C**) is "*deadlock at the other end*"
(Jespersen et al. 2015) — **progress / deadlock-freedom**, a coordinate *distinct* from completion and
leak-freedom, and delegation is the *recognized* feature that breaks first-order sessions' automatic
deadlock-freedom (Coppo–Dezani–Yoshida–Padovani; Kobayashi; deadlock-free GV). I saw the `Rc` hazard's
leak-FACE and mislabeled the progress AXIS; the single-threaded witness hides deadlock by construction. So the
**liveness domain has progress as a third coordinate** (the lock seams #15/#16 were its *lock* face, delegation
its *session* face); the sound order + don't-use-after-delegate core CLOSES (the affine *move* carries
don't-use-after-delegate). N stays 9 (rejection is one of the nine modes); the FAILED broken-seam count **stays
seven** (sound — a deliverable rejection, not a soundness break). The probe earned its keep by being
instructively **wrong**. And a twenty-fifth — a *multiparty session* (an n-party protocol assembled from #23's
binary edges; `multiparty`, r32) — built the very feature #23's deflation #1 named (*global deadlock-freedom
needs multiparty typing*): a `RecvThenSend` role enforces recv-before-send, and `ring()` — three perfectly-dual
binary edges wired into a source-less cycle — **type-checks a globally-deadlocked system** (the floor cannot
range over global topology), while the coherent `relay()` completes. **SOUND** (SEAM-COSTS [horn 3], no UB — a
memory-safe recv-`Err` hang; decidable) **but the gauntlet FAILED the §6 DELIVERABLE again** — the **rejection
mode's *third* firing** (after `typed_nat` #8 and `delegation` #24), and, as with #24, **refuted from my own
cited lineage**. My deliverable staked a **false dichotomy**: that global progress lives *only* whole-protocol
in a *generative* global-type projection, no per-role discipline able to reject the ring. **False** — the
omitted middle rung, **per-channel priority/obligation typing** (Kobayashi linear-π; Padovani; **Dardha–Gay
FoSSaCS 2018**; Kokke–Dardha), rejects the source-less ring *compositionally, with no global type*, via
`prio(recv) < prio(send)` chaining to a contradictory `p < p` on the cycle; and I had **mis-filed Kobayashi as
*support* when he is the *counterexample***. The honest taxonomy has **three rungs**: (a) the per-edge `HasDual`
floor [doesn't carry]; (b) per-channel priority typing + a whole-program acyclicity check [no global type]; (c)
generative MPST projection. What survives is the *weaker* truth: global progress is **not concentrable
per-call** and needs strictly more than the per-edge floor — but it is a **whole-program** obligation, *not*
necessarily a generative one. The sound role-local order + linearity core CLOSES (Lean
`multiparty_role_order_core_closes`, like `typed_nat`/`delegation`). Liveness stays the third domain;
**progress** stays its third coordinate (lock seams #15/#16 = its *lock* face, delegation = its *session* face,
multiparty = its *global-topology* face); N stays 9 (rejection's 3rd firing); the FAILED broken-seam count
**stays seven** (sound — a deliverable rejection, not a soundness break). 25 primitives; the probe earned its
keep by being instructively **wrong** a third time. And a twenty-sixth — a *per-channel priority typing*
(`Relay<Pi, Po>`, a forwarder that recv-then-sends under `Pi: Lt<Po>`; `priority`, r33) — built the very
**rung (b)** #25's rejection named: per-channel priority/obligation typing (Kobayashi linear-π; **Dardha–Gay
FoSSaCS 2018**), assembled over #23's binary edges and #8's sealed `typed_nat::Lt` (Peano `<`, no reflexive
`Lt<N>`). Built to **confirm** #25's weaker thesis *humbly* — it instead **SEAM-BREAKS [horn 1], FAILED — the
EIGHTH broken seam**, the **first soundness break since `wcet` #22** (the FAILED tally rises 7→8); I predicted
SEAM-COSTS [horn 3] and the horn-classifier moved it to horn 1. **The flaw:** the "ring is untypable" argument
— `p_CA < p_AB < p_BC < p_CA` collapsing to a contradictory `p: Lt<p>` — **secretly assumes EDGE-PRIORITY
CONSISTENCY**, that the `Nat` A *sends* on edge AB is the `Nat` B *receives* on AB. Rust does **no transitive
reasoning**; it checks each `Pi: Lt<Po>` in *isolation*, and the contradictory chain forms only when one `Nat`
is *shared per edge* — which only the coherent `relay()` enforces. So the **public** `role::<Pi, Po>` lets
three `role::<N1, N2>` relays (every bound `N1: Lt<N2>`, e.g. `1 < 2`, discharging locally) wire into the
source-less ring: it **COMPILES, no `unsafe`, and DEADLOCKS** — falsifying §4.3's "deadlock-freedom of any
well-typed assembly." **The sting:** I had *described this exact flaw* in §4 ("edge consistency is the
constructor's invariant, not a per-endpoint type fact") and shipped `role()` public anyway; the soundness
attacker quoted my own words back, demonstrated by a passing test. **It confirms #25 by BREAKING, not
costing:** the per-channel `Lt` bound carries **nothing global** (it never names the dual endpoint), and
deadlock-freedom rests on a **trusted edge-consistent constructor** (a mini-projection) — salvaging to
SEAM-COSTS [horn 3] *only if* `role()` is hidden. **The unification:** this is the **same break as
`ordered_locks` #15/#16, one soundness sub-domain over** — type-level order carries the *local* bound while
whole-program consistency/acyclicity *escapes* (the wrong-granularity gap), so progress's **lock face**
(#15/#16) and its **session face** (#26) break **identically**. Liveness stays the third domain; **progress**
stays its third coordinate (lock #15/#16 = lock face, delegation = session face, multiparty = global-topology
face, priority = the per-channel-order face that breaks); N stays 9 (the SEAM-BREAKS [horn 1] verdict is one of
the nine modes); the FAILED broken-seam count **rises to eight** (Lean `priority_typed_discipline_factors`, a
`..._factors` like `ordered_locks`). 26 primitives; the probe earned its keep by being instructively **wrong
about its own predicted verdict**. And a twenty-seventh — a *scheduled relay network*
(`seam/src/projected.rs`, r34) — was the **SALVAGE** of #26 (priority), which broke because its per-role
constructor `role()` was **PUBLIC**: the fix **SEALS** `worker()` (module-private) and vends only trusted
builders (`relay`, `relay4`) that thread strictly-increasing `Nat`s per edge, so the #26 ring is
**unconstructible** two ways — (1) *access* (no public `worker()` to wire freely); (2) *structure* (a
monotone chain cannot close — the closing edge needs a *decreasing* `Lt`, and no instance exists). **SOUND**
(SEAM-COSTS [horn 3] — the salvage WORKS, Horn 1 conceded) **but the gauntlet FAILED the §6 DELIVERABLE — the
rejection mode's *fourth* firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25; #26 was a
*break*, not a rejection). **The overclaim:** §4 claimed "deadlock-freedom by construction" + "the consumer
carries nothing global." But the builder proves only that a deadlock-free *schedule* **EXISTS** (a static
`Nat` witness); affine Rust never **FORCES** the runtime to follow it — a run-all / in-order / no-drop
**PROGRESS RESIDUE** stays on the consumer. Honest: "**schedulability** by construction, *given all roles
execute*." **The giveaway:** Scribble/mpstthree endpoints are deadlock-free *only assuming every participant
executes* — I imported that guarantee while claiming the consumer carries nothing. **Concentration is
PARTIAL:** the constructor *does* carry **topology** (no cycle — the #26 fix is real), but
**completion/execution** never left the consumer (the #17/#18/#23 residue, recurring). (Secondary deflation:
the expressiveness cost was **OVER**stated — acyclic generality is recoverable via *one* generic combinator,
mpstthree.) **The refined finding: the PROGRESS COORDINATE DECOMPOSES** — deadlock-freedom / topology (the
trusted constructor carries it) versus completion / execution (the consumer carries it) — sharpening the
#24–26 arc with a cleaner split. The sound topology-carrying core CLOSES (Lean
`projected_schedulability_closes`, a `..._closes` like `typed_nat`/`delegation`/`multiparty`). Liveness stays
the third domain; **progress** stays its third coordinate (lock #15/#16 = lock face, delegation = session
face, multiparty = global-topology face, priority = the per-channel-order *break* face, projected = the
**scheduled-network** face that *salvages but residues*); N stays 9 (rejection's 4th firing); the FAILED
broken-seam count **stays eight** (sound — a deliverable rejection, not a soundness break). 27 primitives;
the probe earned its keep by **completing the salvage and exposing what it still cannot carry**. And a
twenty-eighth — a *branching choice* (the session-type `&`/`⊕` connective: external choice `offer` receives a
label and branches, internal choice `select` sends one; `choice`, r35) — was the **last unprobed CORE
session-type feature**, and it is **SOUND** (no UB — worst case a *starved/leaked* memory-safe session;
decidable) **but the gauntlet FAILED the §6 DELIVERABLE — the rejection mode's *fifth* firing** (after
`typed_nat` #8, `delegation` #24, `multiparty` #25, `projected` #27; #26 was a *break*, not a rejection).
**The break:** §4 guarantee #2 "handle every label" is **VACUOUS** — Rust exhaustiveness gives *scrutinee
coverage*, not *run-to-`End`*: a safe `match offer() { Ok(Branch::Left(c)) => .., _ => {} }` type-checks, **no
`unsafe`**, and **DROPS `Chan<R>`**, starving that branch. "Handle-all" collapses into the conceded
#17/#18/#23/#27 **completion residue**. **Four panel corrections, all accepted:** (1) branch-safety is
**two-legged** (label–continuation correlation + branch-set duality), **not four** — "handle every label" is
the false leg, and *order* is the generic typestate guarantee (since #18), not branch-specific; (2) the
deflation is onto **binary `Either` + exhaustiveness**, not *n-ary named* sum types (labeled n-ary choice needs
nesting that drops the label names, or `offer!`/`choose!` macros); (3) horn 2 is **Θ(type-size)**, not Θ(depth)
(2^depth leaves, but interning keeps it linear-in-source — costless, merely mislabeled); (4) completion is
**amplified per-branch and peer-choice-reachable** (adversary-triggered), not "unchanged by branching." **The
core thesis SURVIVES, sharpened: choice adds NO NEW COORDINATE** — completion stays the #17/#18/#23/#27 residue,
now fed by **two** safe paths (`mem::forget` *and* the dropped wildcard branch). No new domain (liveness stays
third), no new coordinate (progress, from #24), no new mode (N **STAYS 9** — rejection is one of the nine).
Prior art **CONFIRMS** (Honda 1993's `&`/`⊕`; the `session_types` / Jespersen–Munksgaard–Larsen 2015
affine-vs-linear gap *is* this completion residue). The sound branch-typing core CLOSES (Lean
`choice_branch_typing_closes`, a `..._closes` — the two legs + generic order close, branch *completion* does
not). Liveness stays the third domain; **progress** stays its third coordinate (choice feeds *completion*, not
progress — it adds no new progress face); N stays 9 (rejection's 5th firing); the FAILED broken-seam count
**stays eight** (sound — a deliverable rejection, not a soundness break). 28 primitives; the probe earned its
keep by being instructively **wrong** a fifth time. And a twenty-ninth — a *must-complete session handle* (a
`Session<P>` affine handle with a `Drop` **completion bomb**, cf. `take` #9) — turned, after the session
connectives were exhausted (#28), back on the **completion residue itself** (the #17/#18/#23/#27 leftover every
prior session seam conceded). It is **SOUND** (SEAM-COSTS **[horn3-concentration]**, no UB — a free handle's
skipped bomb only *starves a peer*, never laundering UB; decidable) **but the gauntlet FAILED the §6
DELIVERABLE — the rejection mode's *sixth* firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25,
`projected` #27, `choice` #28), and **the most instructive of the arc because it REVERSES my finding.** I
pre-registered **SEAM-BREAKS** — "the session face of `linear` #17", completion an **IRREDUCIBLE linear gap**
the affine floor cannot close. The mechanism break is *real* (a free handle's `Drop` bomb is bypassed by
`mem::forget`, starving the peer) **but it only shows the WITNESS wrong, not the claim**: the decisive
refutation is that my **"UNIVERSAL NONE" is FALSE** — a **scoped-closure / CPS combinator** (`with_session(|s|
…)`, the **`std::thread::scope` analogue**) **COMPELS completion modulo divergence, with no `unsafe`**, by
moving the obligation to the **re-import boundary** (there is no forgettable free handle to leak). **The sting:**
I cited `thread::scoped`'s **REMOVAL** (RFC 1084) but **ignored its FIX**, `thread::scope` (stable since 1.63) —
the very pattern that compels the closure. **THE REVERSAL:** completion is **NOT irreducible — it is
CONCENTRABLE**, at a **horn-3 API-locality** cost (surrender the free handle); the whole #17–#28 residue was
**the price of the FREE-HANDLE API**, not a linear gap in the floor. And sessions are **NOT #17's cleanup**: a
*delimited* session CAN be scoped, where *open-ended* cleanup cannot — **refuting deflation (a)** carried since
#17. (Further, accepted: ergonomics — the abort-bomb turns `?` into a **process-abort**, unusable on fallible
code; prior-art — peer starvation is fixable by **CANCELLATION PROPAGATION**: Mostrous–Vasconcelos *Affine
Sessions*; ECOOP 2022 *Stay Safe Under Panic*; MultiCrusty; EGV. SURVIVES: decidability — Θ(depth), and
leak-detection-is-NONE correctly conceded.) **Meta — this INVERTS the usual error:** the prior rejections
caught me *underclaiming* a coordinate; here I **overstate the EXTREMES** (total-carry / irreducible) when the
truth is a **cost between**. The sound completion core CLOSES via the **scoped** witness (Lean
`must_complete_scoped_completion_closes`, a `..._closes` — the scoped witness closes completion at an
API-locality cost; the free-handle witness only *factored*). Liveness stays the third domain; **completion**
stays the #17/#18/#23/#27 residue — now shown **concentrable, not irreducible**; N stays 9 (rejection's 6th
firing); the FAILED broken-seam count **STAYS EIGHT** (SEAM-COSTS, *not* the break I predicted — sound, a
deliverable rejection). 29 primitives; the concrete scoped salvage (`with_session`) is the natural **#30**
(mirroring #26→#27); the probe earned its keep by being instructively **wrong** a sixth time — by *reversing*
me. (Phases L–AB.)*

> **A note on the "#5/#6/#7" shorthand (and a numbering collision).** `#6` (zero-cost) and `#7`
> (AI-legible) are the literal seed-property numbers. The "**#5⟂#7 cell**" shorthand, coined for the
> refined phase, uses `#5` for the *fifth lattice axis* — **API-locality / brand-freedom**, the
> *type-face of seed property #2 (no-seam)*. The shared phase then recovered the *actual* seed
> property #5 — **bare-metal** — as the seventh lattice axis. To avoid the clash: **API-locality is
> referred to by name or as the "#5 cell"; bare-metal is referred to by name (it is seed #5).** Lattice
> numbering ≠ seed numbering; this note is the key.

---

## How a "new axis" is established (the method, not an assertion)

An axis is genuinely independent of the others iff some primitive **dissociates** it: two primitives
that agree on every other named axis but split on this one. That split is a realizability fact — a
witness — not a definition. The slab gave the template (SPSC and slab agree on {sound, decidable,
concentrated} but split on zero-cost ⇒ zero-cost is a real 4th axis). Phase D applied the same test
to find the 5th (API-locality), and Phase E built the off-diagonal that witnesses the 6th
(AI-legibility) — promoting it from the branded phase's argued claim to a confirmed one.

Every ✓/✗ below is computed from real `rustc`-checked code in `../seam/` and an **independent cold
verdict** (`REPORT_SEAM_SPEC_*.md`), never asserted by the builder.

## The axes (columns)

The original seed had seven properties. In the *seam* framing the floor concedes #1 (executable
spec) and #3 (derived ownership) by construction (the floor is a fixed checker). The live,
per-primitive axes — note **bare-metal (#5), unwind-safety (8th), and leak-freedom (9th) are not
uniform**: the shared-ownership seam pays the first, take-and-replace the second, drain-and-yield the third:

| Axis | Seed # | What it asks | Folded into the trilemma as |
|---|---|---|---|
| **Sound** | (the wrapper) | Does re-import launder no UB into safe code? | horn 1 |
| **Decidable & proof-free** | #4 | Does the safe fragment need no solver, no hand proof? | horn 2 — *itself a bundle* of **decidability** ⊗ **solver-freedom**, un-bundled by the refined index |
| **Concentration (seam-width)** | #2 (code face) | Does *ordinary code* cross the `unsafe` boundary only O(1)/primitive? | horn 3 |
| **API-locality (brand-freedom)** | #2 (type face) | Does the witness stay out of consumer **type signatures** (no viral `'id`, no forced scope-closure)? | **folded inside "concentration"** — un-bundled by the branded index |
| **Zero-cost erasure** | #6 | Does the witness erase at compile time (no runtime residue)? | **folded inside "concentrated & proof-free"** — un-bundled by the slab |
| **AI-legible** | #7 | Is the witness's *meaning* reconstructible (a readable predicate), not a nominal token? | not in the trilemma at all — surfaced by the branded index, **confirmed independent by the refined index** |
| **Bare-metal (runtime-freedom)** | **#5** | Does sound re-import avoid a *runtime memory manager* (no global allocator / GC)? Operationally: compiles under `#![no_std]` with no `alloc`? | not in the trilemma — surfaced by the shared-ownership seam; **= Rust's `core`/`alloc` boundary (RFC 2480)**, real & independent but pre-existing, *not* a novel dimension |
| **Unwind-safety (exception-safety-guarantee-level)** | — | Does sound re-import survive a `panic`/unwind *through a transiently-broken invariant* without downgrading the basic guarantee? | not in the trilemma nor the seed — surfaced by the take-and-replace seam; **= Abrahams' basic/strong/nothrow guarantees** (C++ `noexcept`, Rust `UnwindSafe`/`catch_unwind`/`Mutex` poisoning, `take_mut`↔`replace_with`), real & independent but pre-existing. **Carries a *discharge menu*** (abort / placeholder-repair / poisoning), not a fixed cost |
| **Leak-freedom** | — | Does sound re-import survive the witness being *leaked* (its `Drop` **never** running, e.g. `mem::forget`)? | not in the trilemma nor the seed — surfaced by the drain-and-yield seam; **= Rust's leakpocalypse** (`mem::forget`-is-safe; leak amplification; `Vec::drain`; RFC 1066 / `thread::scoped` removal; linear-vs-affine types), real & independent but pre-existing. **Distinct from unwind-safety** (leak = `Drop` never runs; unwind = `Drop` runs at a bad time). The *thinnest* axis — cost invisible in non-leak code |

Two separate "cost" categories sit *outside* the axes and must not be miscounted as horns:
- **TCB axiom** (MMIO): a physical-world fact ("this address really is this register") no checker
  can discharge — a trusted `assume` boundary (seL4/CompCert/Low\*), not a property the seam fails.

## The table

Legend: ✓ holds / aced · ✗ paid or dropped · ◐ conditional · *(plain)* a non-viral ordinary lifetime.

| Primitive | Sound | Decidable | Concentration | **API-locality** | **Zero-cost** | **AI-legible** | **Bare-metal** | **Unwind-safe** | **Leak-safe** | What it costs |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|---|
| **SPSC ring buffer** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `'a`)* | ✓ | ✓ | ✓ | ✓ | ✓ | nothing — the fully-encapsulable corner |
| **MMIO read-to-clear** | ✓ | ✓ | ✓ O(1)¹ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | a **TCB axiom** (outside the axes) |
| **Generational slab** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Key`)* | **✗** | **✗**² | ✓⁹ | ✓ | ✓ | **zero-cost (#6)** — runtime generation+occupancy check |
| **Branded index** | ✓ | ✓ | ✓ O(1) | **✗** *(viral `'id`)* | ◐³ | **✗**⁴ | ✓ | ✓ | ✓ | **API-locality (#5)** + deferred zero-cost; brand is nominal |
| **Refined index** | ✓ | ◐⁶ | ✓ O(1) | ✓ *(brand-free; `Refined` escapes)* | ◐⁷ | **✓**⁸ | ✓ | ✓ | ✓ | **horn-2 solver-freedom** (SMT in the TCB) + the R2 trade |
| **Shared (Rc)** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Shared`)* | **✗** | ✓ | **✗**¹⁰ | ✓ | ✓ | **bare-metal (seed #5)** — needs a global-allocator refcount manager |
| **Type-level index** | ✓ | ◐¹¹ | ✓ O(1) | ✓ *(plain turbofish)* | ✓ | ✓ | ✓ | ✓ | ✓ | **eighth-axis probe FAILED** — sound, but the "new axis" is confounded + encoding-relative (see below) |
| **Take-and-replace** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `take`)* | ◐¹² | ✓ | ✓ | **◐**¹³ | ✓ | **unwind-safety (8th axis)** — sound via an abort-bomb; basic guarantee downgraded |
| **Drain-and-yield** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Drain<'a>`)* | ✓ | ✓ | ✓ | ✓ | **◐**¹⁴ | **leak-freedom (9th axis)** — sound via pre-pessimization; a leaked guard loses the tail |
| **Self-ref cell** | ✓ | ✓ | ✓ O(1) | **✗**¹⁵ *(viral, `&mut`-revoking `Pin`)* | ✓ | ✓ | ✓ | ✓ | ✓ | **move-stability — REAL but INSEPARABLE**, *not* a new axis (N stays 9); cost = *immovability*, non-uniform (stack-pin→API-locality, heap-pin→bare-metal) |
| **Validity (bytes→enum)** | ✓ | ✓ | ✓ O(1) | ◐¹⁶ | ◐¹⁶ | ✓ | ✓ | ✓ | ✓ | **validity — INSEPARABLE (2nd), and the CLOSED-BASIS proof**: a new danger class adds *no tenth axis* (unanimous); its own cost is non-uniform (per-read→#6, zero-copy view→API-locality) |
| **Intrusive list** (branded) | **✗**⁵ | ✓ | ✗ O(accesses) | ✗ | ◐ | ✗ | ✓ | ✓ | ✓ | a **horn genuinely drops** — no sound+usable+concentrated decision exists |

¹ with a foundation `Send` opt-in, else O(ISRs). ² **reclassified legible→illegible** (cold panel,
endorsed by two independent attackers vs the slotmap/generational-arena precedent): a `Copy` `Key`'s
validity (live slot at matching generation) is *not* reconstructible from the `Key` type or the
`get -> Option` signature — the occupancy+generation mechanism is foundation-internal. ³ **aced only
under amortization** — index reuse, or the range-branding the spec scopes out; naive per-element
`try_index` relocates the bounds check (same cost as `slice[i]`). ⁴ the brand `'id` is a *nominal*
token; the in-bounds proof lives in the foundation `unsafe`, not in any consumer-readable type.
⁵ sound *only* by collapsing all nodes to one arena lifetime (design A), which drops usability (no
early reclaim); design B keeps usability and is unsound (L2 UAF). ⁶ **horn 2 un-bundles**:
*decidable* ✓ (QF-LIA/UTVPI terminates) but *solver-free* ✗ — it needs an external SMT solver in the
trusted loop, and the fragment is too weak for the bitwise/modular/stride index math (`pos&(cap-1)`,
`i*stride`) that motivates `get_unchecked`. ⁷ the **R2 trilemma**: brand-free + sound forces a
runtime `bound ≤ len` compare (the shipped artifact — *not* zero-cost); recovering erasure forces a
viral length-index brand (`const N`) — the #5↔#6 trade, from the other side. ⁸ **legible** — the
predicate `value < bound` *is* the type's contract; the safety invariant is reconstructible from the
signature alone. This is the cell-closing witness. ⁹ **bare-metal ✓** — the slab is a fixed
`[Slot; N]` pool, `core`-only, no heap; it is the off-diagonal partner that proves bare-metal ⟂
zero-cost (both pay a runtime cost, only `Shared` needs a manager). ¹⁰ **bare-metal ✗** —
`Shared::new` allocates on the global heap; it is the sole reason the crate pulls in `extern crate
alloc` (compile-enforced: the other six are `core`-only). The axis is real and independent but *not
novel* — it is Rust's `core`/`alloc` boundary (RFC 2480), precisely *unbounded-dynamic-allocation vs
caller-provided-bounded-storage* (a `heapless`-pool `Rc` would be bare-metal but capacity-bounded).
¹¹ **decidable ✓, but pays a recorded `recursion_limit` cost inside horn 2 — *not* a new axis.** The
type-level index proves `I < Len` by trait resolution (`I: Lt<Len>`), which erases (zero-cost ✓),
needs no solver (solver-free, unlike refined), and is sound (sealed `Nat` + the orphan rule block
every forgery — conceded unanimously). Its cost: a deep-but-in-bounds index fails to *check* under the
default `recursion_limit` (E0275 overflow). The eighth-axis claim built on this **FAILED the gauntlet**
(see the dedicated section below): the cost is encoding-relative (binary numerals erase it) and
confounded with a strict expressiveness loss (static-index-only), so it isolates no clean axis.
¹² **zero-cost ◐ — runtime-clean, codegen-dented.** The abort-bomb is a ZST whose `Drop` never runs on
the happy path (it is `mem::forget`-ten), so there is *no runtime residue*. But the live bomb forces an
unwind landing pad to exist and acts as an **optimization barrier** (the cold panel's recorded
qualification), so erasure is perfect at the runtime level and mildly imperfect at codegen. ¹³ **the
eighth axis — unwind-safety ◐.** Sound re-import of a transient-invariant-breaking op (move-out/move-
back) survives a `panic` through the gap only by paying: this seam installs an **abort-bomb** (a mid-
take panic aborts before the broken slot is observed), which is sound but **downgrades the basic
exception-safety guarantee to "abort"** (non-`catch_unwind`-recoverable). The cold panel confirmed this
a **real, independent eighth axis (N ≥ 8)** and a *clean* off-diagonal (vs SPSC: splits on the
guarantee-level alone), deflated to **pre-existing** (Abrahams basic/strong/nothrow; C++ `noexcept`;
`UnwindSafe`/`catch_unwind`; `Mutex` poisoning; RFC #1508 *rejected* this exact `take_mut` from std on
this very cost). Mandatory relabel: the axis is the **guarantee-level**, and abort is *this seam's*
discharge of it — a `replace_with` sentinel discharges the same obligation soundly-and-recoverably by
paying **API-surface** instead, so the axis carries a **discharge menu**, not a fixed cost. ¹⁴ **the
ninth axis — leak-freedom ◐.** Sound re-import of a guard whose `Drop` does load-bearing cleanup
survives the guard being *leaked* (`mem::forget` is safe — the Rust leakpocalypse) only by paying: this
seam **pre-pessimizes** (sets the buffer length to 0 *before* yielding), so a forgotten guard leaves a
valid empty buffer (the un-yielded tail leaks, no double-free). The cold panel confirmed it a **real,
independent ninth axis (N ≥ 9)** and a *clean* off-diagonal vs SPSC, **distinct from unwind-safety**
(the off-diagonal is populated both ways — take is unwind-sensitive/leak-safe, drain is
leak-sensitive/unwind-safe). Deflated to **pre-existing** (leakpocalypse / PPYP / leak amplification;
RFC 1066; `thread::scoped`/`JoinGuard` removal; linear-vs-affine types). Recorded nuance: the
**thinnest, most-deflatable** axis — its cost (leaked-tail data loss) is invisible in all non-leak code
— but a genuinely populated off-diagonal, and it aces API-locality precisely *because* the guard owns
its data (a free-standing `Drain<'a>`, no closure), unlike the scoped-thread leak-freedom witness.
¹⁵ **API-locality ✗, and the *cost itself* is the find.** The self-ref cell's `Pin<&Self>`/`Pin<&mut Self>`
is the viral-*restrictive* wrapper (it infects every receiver **and revokes `&mut`** — that revocation
is the load-bearing soundness mechanism), so it flips API-locality. But the deeper finding (see the
inseparable-axis section below) is that move-stability is a **distinct danger with no isolable cost** —
its real price is *immovability* (heavier than API-locality), and it is *non-uniform* across witnesses
(stack-pin spends API-locality, `Box::pin` spends bare-metal). So this row does **not** add a 10th axis
column; it adds a new *category*. Soundness held (the move is unconstructible: `Pin::get_mut` needs
`Unpin`, the cell is `!Unpin`).
¹⁶ **API-locality ◐ / zero-cost ◐ — non-uniform, hence inseparable; and the closed-basis proof.** A
`transmute`-after-set-membership-check read of a `#[repr(u8)]` enum byte. Sound (set membership = the
validity predicate *exactly*; moving from a *range* check, unsound for non-contiguous discriminants, was
the fix two gauntlets forced). Its cost is **witness-non-uniform**, the move-stability signature: a
single `tag_at` is a per-read runtime check (**zero-cost #6**), but the zero-copy `&[u8] → &[Tag]` view
that restores concentration needs either eager O(n) validation (#6) or a **sealed constructor**
(**API-locality**). So validity is **inseparable** — a *second* example, making inseparability a
recurring category. The unanimous meta-ruling: it adds **no tenth axis**; every witness spends a
pre-existing coordinate. Caveats: sound only in the *fieldless single-byte corner* (data-carrying /
`repr(C)` enums need a recursive `is_bit_valid`), and wholly non-novel (`num_enum` / `bytemuck` /
`zerocopy` + refinement types).

## What the table proves

**1. SPSC is the only all-✓ row.** Exactly one primitive aces every axis — and it is the one whose
precondition is *fully floor-expressible* (every escaping conjunct maps to affinity / NLL `'a` /
decidable `T: Send`). Everything else pays somewhere. "Seamless" is achievable, but only for
SPSC-shaped preconditions.

**2. The recovered axes are pairwise dissociable — so they are real, independent dimensions:**

| Claimed-independent pair | Witness that splits them | Status |
|---|---|---|
| Zero-cost ⟂ Concentration | slab (conc ✓, zero-cost ✗) vs SPSC (both ✓) | **gauntlet-confirmed** (slab phase) |
| API-locality ⟂ Concentration | branded index: O(1) seam-width yet viral `'id`; and the `runST`/lightweight-capabilities lineage pays the brand tax **in languages with no seam or `unsafe` at all** — a cost that exists where seam-width is *undefined* cannot *be* seam-width | **gauntlet-confirmed** (this phase) |
| API-locality ⟂ Zero-cost | slab (zero-cost ✗, API-local ✓) vs branded index (brand erases, API-local ✗) — opposite corners | **gauntlet-confirmed** |
| AI-legible ⟂ API-locality | **the off-diagonal, now built.** Refined index = (brand-free, **legible**) vs slab = (brand-free, **illegible**): holds #5 fixed, flips #7. Branded index = (viral, illegible) vs slab = (brand-free, illegible): holds #7 fixed, flips #5. Both directions witnessed. | **gauntlet-confirmed** (refined phase): the judge ruled "#5⟂#7 close? — **YES**", and the slab→illegible reclassification was endorsed by two independent attackers. Promoted argued→**confirmed**. |
| Bare-metal ⟂ Zero-cost | slab (zero-cost ✗ runtime check, **bare-metal ✓** fixed pool) vs Shared (zero-cost ✗ refcount, **bare-metal ✗** global heap): agree on #6, flip bare-metal. A runtime *check* ≠ a runtime *manager*. | **gauntlet-confirmed** (shared phase): the judge ruled bare-metal a real, independent seventh axis — *but* "reduces-to-X for novelty": X = Rust's `core`/`alloc` boundary (RFC 2480); `heapless::pool` already witnesses the bare-metal⟂capacity trade. **Real and independent, but pre-existing — not a new dimension.** |
| Unwind-safety ⟂ all seven | **take-and-replace (unwind-safe ◐, all seven ✓) vs SPSC (all eight ✓): agree on every named axis, flip unwind-safety alone.** The cleanest off-diagonal in the set — and the panel explicitly contrasted it with the *failed* typed_nat probe (which secretly co-flipped expressiveness). | **gauntlet-confirmed** (take phase): the judge ruled unwind-safety a real, independent **eighth** axis (N ≥ 8), deflated to **pre-existing** (Abrahams basic/strong/nothrow; `noexcept`; `UnwindSafe`; `Mutex` poisoning). Mandatory relabel: the axis is the **guarantee-level**; abort is one **discharge** of it (see the discharge-menu note below). |
| Leak-freedom ⟂ unwind-safety | **take-and-replace (unwind-sensitive ◐, leak-safe ✓) vs drain-and-yield (leak-sensitive ◐, unwind-safe ✓): the off-diagonal is populated *both* directions.** "`Drop` never runs" (leak) vs "`Drop` runs at a bad time" (unwind) are independently witnessed. And drain vs SPSC flips leak-freedom alone (all eight else ✓). | **gauntlet-confirmed** (drain phase): the judge ruled leak-freedom a real, independent **ninth** axis (N ≥ 9), **distinct from unwind-safety — do not merge**, deflated to **pre-existing** (leakpocalypse / PPYP / leak amplification; RFC 1066). No confound: aces API-locality *because* the guard owns its data (free-standing, no closure). The **thinnest** axis (cost invisible in non-leak code), but a genuinely populated off-diagonal. |

**3. The trilemma is exactly ONE forbidden pattern in this space.** {Sound ∧ Decidable ∧
Concentrated} is unachievable *only* when the precondition factors through no available witness — and
the intrusive list is the witness that the forbidden pattern is real (whole-heap aliasing+liveness
escapes every witness vocabulary, so a horn drops). Every other primitive **closes** the three
horns and pays on a *recovered* axis instead. The trilemma never said otherwise — it is a true
shadow; the table is the object casting it.

## The eighth axis took TWO probes — one rejected, one confirmed (the method's discrimination)

The eighth axis was reached on the *second* try, and the contrast between the two attempts is the
sharpest evidence the method measures something real: **it rejected the confounded candidate and
confirmed the clean one.**

- **Probe 1 — checker-tractability (type-level index): REJECTED.** Pitched as "does the native checker
  halt without an arbitrary fuel knob?" The seam is sound, but the candidate axis is confounded
  (details below): it does *not* isolate one coordinate, so N stayed 7.
- **Probe 2 — unwind-safety (take-and-replace): CONFIRMED, N ≥ 8.** A different danger (*time*, not
  bounds): moving a value out of a `&mut T` and back leaves a duplicate in the gap, and an unwinding
  panic drags the caller through it into a double-drop. The sound witness is an **abort-bomb**
  (`take_mut`'s pattern): a mid-take panic aborts before the broken slot is observed. This seam **aces
  all seven prior axes and pays on unwind-safety alone** — the clean off-diagonal vs SPSC the panel
  demanded. The judge ruled it a real, independent eighth axis, deflated to pre-existing (Abrahams;
  `noexcept`; `UnwindSafe`; `Mutex` poisoning; the `take_mut`↔`replace_with` lineage; RFC #1508 rejected
  this exact primitive from std on this very cost). **Mandatory relabel:** the axis is the
  *exception-safety-guarantee-level*, and abort is merely *this seam's* discharge of it — a
  `replace_with` sentinel discharges the same obligation soundly-and-recoverably by paying API-surface
  instead. So the axis carries a **discharge menu**, not a fixed cost (see "Structure among the axes").

### Probe 1 in detail — why checker-tractability FAILED (the method's STOP)

The first three recovered axes (slab→zero-cost, branded→API-locality, refined→legibility) each came
from a primitive that **cleanly dissociated** one coordinate. The first eighth-axis probe — the
type-level index, pitched as recovering **checker-tractability** (does the native checker halt without
an arbitrary fuel knob?) — **did not**, and the cold gauntlet rejected it. This is recorded as a
first-class result, not hidden: it is where the method's own discipline drew a line.

**The seam is sound** (unanimous): `I: Lt<Len>` proves `I::VALUE < Len::VALUE = len`; sealing `Nat`
(`Nat: Sealed`, in-crate only) plus the orphan rule plus `new`'s conservative length check block every
forgery. No UB path. That sub-result stands, and the primitive pays a real `recursion_limit` cost.

**But the eighth *axis* claim failed three convergent ways** — three independent attackers, one judge:

1. **The off-diagonal is confounded by expressiveness.** `get::<I>` takes a turbofish *type*, never a
   runtime `usize`, so the type-level index can only address **compile-time-constant** positions — a
   *strict expressive subset* of the branded and refined indices, whose whole reason for existing
   (`get_unchecked` on ring/arena/slab) is **data-dependent** indexing the seam cannot express. So
   "typed_nat vs refined flips *only* checker-tractability" is false: it silently *also* flips
   expressiveness. You cannot read a one-coordinate independence off two primitives that do not accept
   the same programs. This is the **OFF-DIAGONAL requirement** (the same gotcha Phase E had to honor to
   close #5⟂#7) failing — here unfixably, because carrying a *runtime* index with a *compile-time*
   proof is a contradiction in terms: the expressiveness loss is intrinsic to the witness kind.
2. **The decisive cost is encoding-relative.** The ✗ checker cell is an artifact of **unary Peano**
   (`S<S<…>>`, depth = the index value). **Binary** type-level naturals (typenum's `UInt<U,B>`) make
   resolution **O(log N)** — never approaching `recursion_limit` for realistic indices. The cell
   therefore measures *which numeral library you linked*, not a property of the primitive. A table
   coordinate that flips with an implementation detail is not an axis.
3. **It is mislabeled, and the real concern is pre-existing.** Even the friendly (prior-art) attacker
   granted separability but corrected the name: `Lt` resolution is **linear** in the index — strictly
   *cheaper* than refined's NP-hard QF-LIA — so the cost is **not** "tractability." What
   `recursion_limit`/E0275 actually marks is the absence of a guaranteed **terminating-resolution
   fragment**: exactly GHC's `UndecidableInstances` / the **Paterson conditions** / `-freduction-depth`,
   and the Turing-completeness of trait resolution (Mairson 1990 for the DEXPTIME lineage). A real,
   named, pre-existing PL boundary — but one this primitive cannot witness as a clean lattice axis.

**The honest verdict on probe 1: a sound seam that pays a recorded `recursion_limit` cost *inside* horn
2, but recovers no new axis.** The value of the *negative* result: the method is not a
novelty-generating machine. Three primitives isolated axes because their witnesses dissociated cleanly;
this one could not, and the off-diagonal discipline said so out loud. A method that only ever confirms
is not measuring anything — this one has a STOP. Probe 2 (take-and-replace, above) then supplied the
*clean* off-diagonal probe 1 lacked, and the panel confirmed unwind-safety as the eighth axis — so the
two probes together show both halves of a working instrument: it **rejects** the confounded candidate
and **confirms** the clean one.

## The fourth verdict: a *real but structurally inseparable* axis (move-stability)

Three verdict modes were on the table — **confirm** (a clean axis, deflated to pre-existing),
**sharpen** (a sub-face, not a new axis), **reject** (a confounded non-axis). The self-referential-cell
phase found a **fourth**: an axis that is a **genuinely distinct danger** yet has **no isolable cost**.

The **self-ref cell** holds a pointer into itself; *moving* it dangles that pointer — a danger no other
axis touches (none concerns a value's *address*). The seam is sound: `Pin` over a `!Unpin` cell makes
the post-`init` move unconstructible in safe code (every move path needs `Unpin`; `PhantomPinned` denies
it). So the danger is real, distinct, and soundly handled. **But it is not a clean tenth axis**, and the
cold panel ruled — all five attackers converging — exactly *why*:

- **It does not ace API-locality**, so the cost cannot be isolated. `Pin<&Self>` is the
  viral-*restrictive* wrapper (it infects every receiver **and revokes `&mut`**), and that revocation
  *is* the soundness mechanism — you cannot have the safety without the type-surface cost.
- **Its cost is *non-uniform across witnesses*** — stack-pin (`pin!`) spends **API-locality**, `Box::pin`
  spends **bare-metal**, a hypothetical `?Move` immovable type would spend an ambient bound. One danger,
  several different costs ⇒ not one mapped axis "wearing a hat" (so it is *not* the reduction the
  `typed_nat` probe was either).
- **So the honest cost is *immovability* — the revocation of the move/return/store capability**, which
  is distinct from and *heavier* than API-locality (a merely leaked type surface). Move-stability funds
  itself by *being* immovable; there is no separate coordinate to read.

Two caveats the panel insisted on, both now recorded: (a) the inseparability is **contingent, not
necessary** — RFC 2349's *rejected* `?Move`/`Move` immovable types *were* a designed free-standing
witness; Rust **chose** hookless `memcpy` moves + a library `Pin`, so "no free-standing witness can
exist" overstates it. (b) The crisp `core`-only reading is the isolation of a **non-composable**
single-frame toy; any real intrusive node / `Future` / slab entry forces `unsafe` pin-projection or
`Box::pin`. **N stays 9.** The find is the *category*: a real danger with no clean coordinate, only a
menu of *other* axes to spend. **And it is not a one-off** — the validity seam (next section) is a
*second* inseparable danger (bit-level validity, cost non-uniform: per-read #6 vs zero-copy-view
API-locality), so "inseparable" is a recurring class. Two members so far, both about a property the
witness can discharge in more than one place: an *address* (move-stability) and a *bit pattern*
(validity).

## The cost-basis is CLOSED — the constructive arc's conclusion

The validity seam (`seam/src/validity.rs`) was built not to hunt a tenth axis but to ask the question
all the previous probes were circling: **is the cost-basis closed?** It re-imports a danger no prior
primitive touched — a **validity invariant** (a `#[repr(u8)]` enum byte that must be a valid
discriminant, UB even without a dereference). A genuinely new danger class. And the cold panel's
**unanimous** ruling (across three gauntlets — two of which found real spec bugs first, the adversary
earning its keep to the last) is the headline:

> **A new danger class added NO new axis. Every witness for it spends a *pre-existing* coordinate.**
> *"What does the safe witness cost?"* has **finitely many answers** — the nine clean axes, plus the
> inseparable category. The lattice's cost-basis is **closed**.

Concretely, validity's cost is **(3) inseparable** (the panels split (2)-projects-onto-#6 vs
(3)-inseparable, and the more careful reading won): a single checked read spends **zero-cost #6** (a
runtime tag check, the slab's axis via a different predicate), but the zero-copy `&[u8] → &[Tag]` view —
the real reason systems code transmutes byte buffers — spends either eager #6 or a sealed constructor's
**API-locality**. Either way the cost lands on coordinates *already on the map*. This is the **dual** of
the `get_unchecked` family: there *one* danger (out-of-bounds) had *many* witnesses spending *different*
axes (branded → API-locality, refined → solver-freedom, typed_nat → confounded, slab → #6); here *many*
dangers (stale-key, bit-validity) converge on the *same* axis (#6). Read together, the two directions
say the cost-space is a small fixed basis that dangers *project onto*, not an open-ended list of
discoveries.

So the constructive arc concludes: the Seam Trilemma is a **3-of-N projection of a closed object** —
**nine clean axes** (sound, decidable, concentration, API-locality, zero-cost, AI-legibility,
bare-metal, unwind-safety, leak-freedom), **plus an inseparable category** (move-stability, validity) of
real dangers with no clean coordinate, **plus the two outside-the-axes categories** (the MMIO TCB axiom;
the intrusive list's genuine horn-drop). Every memory-safety danger probed since lands in one of these.
The honest bound stays a *lower* bound (a future danger could still surprise), but the evidence — a
brand-new danger class projecting cleanly onto the basis, and inseparability recurring rather than
appearing once — is what closure looks like.

### …but closed PER DOMAIN — the information-flow probe (the arc's generalization)

The closure is for **memory safety** (can safe code reach UB?). One further primitive tested whether it
is *absolute*: a **constant-time secret** (`seam/src/secret.rs`), re-importing a danger that is **not UB**
— an information-flow / **timing** side-channel (branching on a secret leaks it). The cold panel's ruling
**generalizes the whole result**:

> **Information-flow is *out of frame* — a different soundness domain, with its own basis. The closed
> cost-basis is closed *per domain*.** The Seam Trilemma and the cost-basis are **domain-parameterized**:
> memory safety is one domain; information flow is another, and the build-and-break *method* transfers to
> it (its very first IFC seam reproduced the trilemma and surfaced a new axis), but the *coordinates* are
> the new domain's, not these nine.

Two confirmations make it concrete. (1) The secret seam **BREAKS [horn 1] in the IFC sense** — the exact
analog of the intrusive list's memory-safety horn-drop: `ct_eq(a,b).expose_bool()` then `if b` leaks a
secret relationship through control flow, *declassification is etiquette not type-structure*, and closing
it needs whole-program **undecidable** IFC — so a *concentrated, decidable* seam cannot soundly enforce
non-interference (Horn 2 passes only because the strong property is never checked, a syntactic proxy).
**The trilemma recurs, sharper.** (2) The IFC domain's basis has genuinely *new* axes with **no
memory-safety analog** — **declassification discipline** (which / where / how-often a secret-derived bit
may re-enter control flow), plus termination and concurrency channels. (Honest correction the panel
forced: an earlier draft called the trust-the-codegen assumption "MMIO-shaped / irreducible"; it is in
fact **dischargeable** — CompCert-CT / Jasmin / FaCT formally preserve constant-time — a *contingent*
artifact of stock `rustc`/LLVM, a different category than MMIO's permanent axiom.)

#### The deepest refinement: a closing seam exists only when the FLOOR matches the DOMAIN

A second IFC primitive (`seam/src/labeled.rs`, capability-gated declassification) probed the natural
follow-up: does the IFC domain have a *closing* seam (the IFC analog of SPSC/the slab), confirming a real
cost-basis? It **also BREAKS [horn 1], unanimously** — and the *reason* is the project's deepest result.
To *compute* on a secret, `map(self, f: FnOnce(T) → U)` must hand the closure the **bare plaintext**, and
a captured-`&mut` closure exfiltrates it (`s.map(move |x| sink.push(x))`) — cap-free, no `declassify`
site: an uncapped exit, **unpatchable without an effect system Rust's floor disclaims**. And the affine
"budget" counts *uses-per-cap*, not caps-*minted* (a loop mints N) nor **bits** (one cap leaks a megabyte)
— so what it enforces is release *location* (Sabelfeld–Sands' *where/when*), which **projects onto the
memory-safety affine axis**; the genuinely-IFC coordinate is the *amount* (**quantitative information
flow** — Smith min-entropy / g-leakage), and **an affine floor cannot bound bits, only uses**.

So both IFC seams break, for one structural reason: **Rust's affine/borrow floor carries *none* of the
IFC precondition.** It is *matched* to memory safety — where SPSC closes *free*, the floor's affinity/NLL
carrying the precondition, and the basis is closed — and *mismatched* to information flow, where **no seam
closes**: there is no IFC SPSC-analog, the only enforceable cost is a memory-safety axis in disguise, and
the real coordinate (quantitative leakage) needs a *quantitative / effect-tracking* floor (Jif, FlowCaml,
QIF). So the deepest form of the result is layered:

> **The Seam Trilemma is domain-general** (it recurs in every soundness domain). **The cost-basis is
> per-domain** (memory safety has its nine closed axes; information flow has its own — declassification
> dimensions, quantitative leakage). **And *which seams close* depends on the floor-domain match** — a
> floor closes a domain's seams only when it carries that domain's precondition. Rust's affine floor
> closes memory-safety seams and *no* information-flow seam. Map a domain by first asking: *is this floor
> the right floor for it?*

Memory safety is mapped here (basis closed, on a matched floor). Information flow is sketched — a real,
distinct domain whose seams *all break on Rust's floor*, pointing at the floor it would need instead.

#### The match is GRADED — a third domain (liveness) lands *partway*

The IFC probes gave the *negative* half of the floor-domain match (no seam closes). A fifteenth primitive
(`seam/src/ordered_locks.rs`, type-level lock ordering) tested the **positive** half in a **third
domain — liveness / progress**, whose danger is **deadlock** (neither UB nor a leak: a deadlocked program
is memory-safe and confidential). The witness is a type-level lock *rank* `L` plus an affine `Floor<F>`
token, with `with` bounded `L: Below<F>` — so a lock-order inversion *should* be unconstructible, the
floor (a type-level order) seemingly carrying the precondition (an acyclic global order). The cold panel
ruled **SEAM-BREAKS [horn 1]**, and the break is the richer result: it shows the floor-domain match is
**not binary but graded**, because a floor can *under-carry* a domain's precondition along **two
independent axes**:

> **Granularity.** `Floor::root` is a re-entrant safe call, so two sibling roots mint two independent
> `Floor<TopRank>` lineages and safe code types a low-then-high inversion (it compiles). The discipline is
> *per-token*; the precondition is *per-thread-global*. Rust's floor has **no ambient per-thread one-shot
> resource** — so even the *static* fragment does not close without bolting on a thread-local consumed
> capability the language does not provide.
>
> **Coverage (static vs dynamic).** Even granting that fix, the canonical deadlock-free idioms —
> bank-transfer `min(i,j)` ordering, hand-over-hand list/tree coupling, address-ordered bucket locks,
> IRQ/cross-stack ordering — are **inexpressible** (same static rank + irreflexive `Below` + compile-time
> order). The **dynamic-acyclicity** coordinate (what `lockdep` checks at runtime) **escapes the static
> floor**, *exactly as IFC's quantitative coordinate escaped the affine floor*. Prior art forces this:
> it is **Boyapati–Lee–Rinard OOPSLA'02 minus their dynamic tree-order extension**, and **Kobayashi's
> natural-number levels** — both abandoned pure static levels for precisely this reason.

So the floor-domain match has **three grades**: **full** (memory safety on the affine floor — basis
closed), **partial** (liveness on the static-type-order floor — the static distinct-rank single-stack
fragment carried, the granularity + dynamic fragments escaped), **none** (information flow on the affine
floor — both seams break). What closes here is **lock-order-*inversion* freedom**, never "deadlock-freedom"
(condvar / channel / join cycles remain). And the affine floor gets *no* special credit: the order-carrying
floor predates Rust's affinity by two decades in two non-affine settings (BLR/Java, Kobayashi/π-calculus);
affinity buys only token non-duplication — and, per the granularity break, not even that.

#### The floor-upgrade is NON-MONOTONE — a sixteenth primitive walks one rung up the granularity ladder

If the match is graded by *how much of the precondition the floor carries*, the falsifiable next step is to
**upgrade the floor** and watch the match lift. A sixteenth primitive (`seam/src/lock_registry.rs`, feature
`lockdep`) keeps the liveness domain and the same danger but swaps the static type-level order for a
**runtime** one: a `thread_local!` held-set of runtime ranks, checked on each acquire (panicking on a
detected inversion). The cold panel (r22) ruled **SEAM-BREAKS [horn 1]** again — and the *way* it breaks is
the result: **upgrading the floor is non-monotone.**

> **Coverage lifts, at the predicted price.** Runtime ranks express the dynamic idioms the static seam
> could not — bank-transfer `min(i,j)`, hand-over-hand coupling — so the *coverage* fragment closes. But it
> is **bought, not free**: a runtime check per acquire (**zero-cost #6** — the slab's place in a new
> domain), OS thread-local storage (**bare-metal #5** — it literally *cannot be `no_std`*, which is why it
> is feature-gated, making the cost visible in the build graph), and a **modality shift** from static
> *prevention* (the inversion was a compile error) to runtime *detection* (it now compiles and is trapped
> only on paths that execute — soundness is path-coverage-dependent). The graded match is thus **coupled to
> the closed cost-basis**: you pay #5 + #6 + a modality downgrade to lift it.
>
> **Granularity recurs one rung finer.** The held-set is keyed on the OS *thread*, but the unit of
> sequential control in async is the *task*. A guard held across `.await` migrates (work-stealing) to a
> thread whose held-set is empty, so a lower rank is accepted with **no panic** — an executed, undetected
> inversion (the registry is *thread-blind*; verified by a two-thread reproduction). A single-threaded
> executor *false-panics* on two independent tasks sharing one thread; `!Send` blocks the migration but
> leaves the false-panic. So the **granularity axis is a containment-scope *ladder*** — *token < thread <
> task < …* — and each ambient Rust floor reaches exactly one rung (affine token #15, thread-local #16);
> there is no ambient *task-local* (async is a library). Prior art forecloses the static-rank design
> outright: **FreeBSD WITNESS / lockdep / Abseil** all abandoned static ranks for dynamically-*learned*
> acquires-before graphs, because hand-assigned global ranks do not compose.

So a partial match is **not lifted to full by upgrading the floor**: the upgrade trades one gap for a finer
one (coverage fixed, granularity recurred), spends cost-basis axes doing it, and the granularity dimension
turns out to be an *unbounded ladder* no ambient floor tops out. The match stays **partial**.

#### The match-unit is the PRECONDITION, not the domain — a seventeenth primitive breaks on *wrong structural logic*

The two liveness probes above both attacked *deadlock-freedom*. A seventeenth primitive
(`seam/src/linear.rs`, a `MustRun<F: FnOnce()>` cleanup guard) stayed in the **liveness** domain but took a
**different precondition** — *must-run-cleanup* ("this finalizer runs exactly once before the scope is
abandoned") — built **breadth-wise** rather than climbing the ladder. It is the *inverse* of the tenth seam
`drain`: `drain` **closed** by making a skipped `Drop` *harmless* (leak-**tolerance**); this needs a skipped
`Drop` to be *impossible* (leak-**intolerance**). The cold panel ruled **SEAM-BREAKS [horn 1]** across **two
independent rounds** (r23 + r24, breaking attackers included) — the **fifth** broken seam — and the break
adds two things the deadlock probes could not:

> **The match-unit is the *precondition*, not the *domain*.** The same liveness domain grades
> deadlock-freedom *partial* (#15/#16) and must-run-cleanup a clean **none**. So the Phase-M
> "domain-parameterized" result sharpens: *which seams close* is set per **precondition**, not per domain —
> a single domain hosts preconditions of different grades. The "partial-via-`#[must_use]`-lint" texture was
> **rejected**: `scopeguard` ships exactly that warning as a *cosmetic patch over a non-guarantee*, so the
> lint is a non-type advisory (silenced by `let _ =`, blind to `mem::forget`), not a floor fragment.
> Stipulating "`must_use` ∈ floor" only *manufactures* a false partial. **N is unchanged** — this is the
> *enforcement* face of the ninth axis (leak-freedom), where `drain` was the *tolerance* face.
>
> **A NEW verdict mode: *wrong-structural-logic*, not domain-mismatch.** `mem::forget` is safe (its
> unbounded `fn<T>(T)` consumes the guard by one legal **affine** move), so `Drop` never fires — the
> leakpocalypse (RFC 1066; `thread::scoped` removal, RFC 1084). "Runs *exactly* once" is **linear**; the
> floor is **affine** ("*at most* once"). Closing the gap needs either an *undecidable* finalizer-
> reachability pass (`Rc`-cycle escape ≅ halting) or a *viral `Leak`/linear bound* — every sound
> **decidable** enforcement collapses to the latter — so *no decidable boundary price* exists. The
> obstruction is the floor being the wrong **structural logic** (affine vs linear), not the wrong *domain*
> (the IFC sense); sharpened to **Boats' scoped-task trilemma**, of which Rust *deliberately rejected* the
> linear horn. And the meta-lesson the two FAILED rounds carry: a seam **designed to break** cannot be
> given a coherent "usable" spec — *the incoherence IS the break* (both rounds FAILED on spec-coherence, not
> on the conceded `forget` skip), which is itself positive evidence for the verdict.

#### The match is COMPONENT-WISE — an eighteenth primitive HOLDS with a cost, and the same gap lands on a different horn

The four out-of-(memory-safety) probes above all *broke*. An eighteenth — a **session-typed channel**
(`Chan<P>`, an affine typestate handle whose type parameter is the *remaining protocol*) — was the first
to **hold with a cost** instead: the cold panel (r25) ruled **SEAM-COSTS [horn 3]** (TRADEOFF, survived).
It is the crucible for the two prior liveness findings — #15 (typestate carries static *order*) and #17
(the floor cannot compel *completion*) — and it asks whether the floor matches a *composite* precondition
**component by component**.

> **Protocol fidelity decomposes, and the floor matches it component-wise — the components orthogonal.**
> The soundness attacker exhausted the surface (forge a `Chan` at a wrong state → blocked by nominal
> non-subtyping + private fields; skip/reorder → `E0599`; transport-aliasing → `E0499`; and the sharp
> cross-component move — can `mem::forget` shorten `'a` to start a second session? **no**, leaking a value
> holding a `&'a mut` does not shorten `'a`), so **completion cannot infect safety**. The *order-safety*
> component **closes** — sound, decidable, concentrated, and (unlike #15) with **no `root`-hole**, a
> genuine closing sub-seam. The *completion* component is an undischargeable **horn-3 concentration** cost:
> the "don't abandon" obligation recurs at every `?`/panic/early-return, de-concentrating the completion
> half — but no horn fully drops, so the seam holds.
>
> **The same linearity gap, a different horn.** The *same* safe `mem::forget` that **broke** #17's horn 1
> only **costs** here. Completion was #17's *whole* soundness promise (so the gap **unsounds** it); here it
> is *one component beside an independently-sound safety core* (so the gap merely **de-concentrates**).
> **Which horn a linearity gap threatens is not a property of the gap but of how the re-import rule
> *packages* it.** Deflations the panel forced, all accepted: the witness has no duality (`core`/no-`unsafe`
> forecloses two live endpoints, `E0499`), so it is a single-handle *walk*, not a binary session, and the
> abandonment is *self*-abandonment; "usable" holds only for *infallible straight-line* protocols (a
> fallible transport forces `?`, which *is* the abandonment); and the order-closes/completion-breaks split
> is **documented folklore** (Jespersen–Munksgaard–Larsen 2015; RFC 1066; ECOOP 2022 *Stay Safe Under
> Panic*, Cutner–Yoshida–Vassor), so the contribution is **locating** the known split on the lattice. N
> stays 9; the five broken seams are unchanged — this one is the arc's first SEAM-COSTS *hold* outside
> memory safety.

#### The packaging dial, deliberately turned — a nineteenth primitive confirms it BUT names its fence

Phase Q's "which horn depends on packaging" was an *observation*: #17 and #18 were different primitives, so
the horn-move could be incidental. A nineteenth — a **value-owning scope-restore guard** (`Restore<T, F>`,
the real `scopeguard::guard(value, f)` + `into_inner` — the shape #17's `MustRun` deliberately declined) —
turns it into a **controlled experiment**: hold #17's *identical* un-compellable-`Drop` danger fixed
(`mem::forget` skips the restore), change only how the rule packages it (wrap it in #18's structure — a
sound affine-ownership *core* beside which the restore is a side-obligation), and watch the verdict move.
The cold panel (r26) ruled **SEAM-COSTS [horn 3]** (TRADEOFF, survived): three attackers conceded the
ownership core is sound (no safe double-free/alias/double-consume; `mem::swap` strands a value but yields no
UB; trivially decidable), and the *same* `mem::forget` gap that **broke** #17's horn 1 here only **costs**
horn 3 (the restore strands a *logical invariant* — a lost lease, a stale global — recurring at every
`?`/panic/return). **The dial is real.**

> **But the panel BOUNDED the thesis three ways — all accepted, and they sharpen it past the prediction.**
> (1) **Confounded, not single-variable.** "Only the packaging changed" is false: to package a danger as a
> *side*-obligation you must introduce a sound core for it to sit *beside*, so #19 also adds the
> value-ownership core #17 lacked — the value-ownership model **co-varies**. A better-controlled probe than
> the incidental #17-vs-#18 pair, not a clean manipulation. (2) **Bounded, not universal — the horn is
> FORCEABLE.** The *same* skippable-`Drop` danger in a **borrowed-slot / `take_mut`** packaging forces
> **horn-1 UB** (a mid-restore panic double-drops the borrowed slot — the ninth-axis `take` danger), and
> `Pin`'s drop guarantee and the post-leakpocalypse `thread::scope` restructure (the closure shape that
> *forces* the join) exhibit horns that **cannot** be repackaged. So *which* horn depends on **where safety
> rests** — *partly danger-intrinsic*. (3) **Not novel — confirm-by-restaging, no new mode.** `JoinGuard`
> broke horn 1 and `scopeguard` costs horn 3 already in 2015 (RFC 1066, the leakpocalypse); the contribution
> is the **method** (Phase Q's observation made a *deliberate* probe) and **locating the dial**, not a new
> verdict structure. **The refined finding: which horn a danger threatens is a design choice *within a
> structural envelope* set by whether soundness can be DECOUPLED from the un-compellable closure** — owned ⇒
> costs (this seam, `scopeguard`); borrowed-across-unwind / `Pin` / `thread::scope` ⇒ forces horn 1. Phase Q
> said the horn moves; Phase R says it moves *within a fence, and names the fence*. N stays 9; the five
> broken seams are unchanged — this one holds.

#### The fence, hit from the other side — a twentieth primitive proves it FORCED (the sixth break)

Phase R's "movable within a fence" carried a **confound** the panel itself flagged: #17→#19 co-varied the
packaging *and* the value-ownership model, so the cost-3 verdict could not be cleanly attributed to either. A
twentieth — a **take-and-reseat guard over a *borrowed slot*** (`Reseat<'a, T>`, the real `take_mut` in guard
form) — runs the single-variable experiment that confound forbade: it keeps #19's packaging **byte-identical**
(a guard with `Deref`/`DerefMut`/`into_inner`/Drop-restore) and flips **only** owned value → borrowed slot.
The cold panel (r27) ruled **SEAM-BREAKS [horn 1], FAILED — the *sixth* broken seam.**

> **Two findings, both decisive.** (1) **The horn flip is clean — Phase R's confound is repaired.** All five
> attackers agreed: #19 (owned + `into_inner`) *cost* horn 3; this seam (borrowed + `into_inner`), packaging
> byte-identical, *breaks* horn 1. The soundness attacker landed a 4-line *safe* witness — `let mut s =
> String::from("hello"); let mut g = Reseat::new(&mut s); g.push_str(" world"); g.into_inner()` — where
> `into_inner` returns the owned `String` while the slot keeps the `ptr::read` duplicate (`E0509` forces the
> `mem::forget(self)` that suppresses the reseat), so both own overlapping/realloc'd heap and **double-free,
> with no `unsafe` in user code.** Where #17→#19 co-varied two variables, #19→#20 co-varies **none** — the
> cost-3 → break-1 flip is attributable to the **decoupling property alone**. (2) **The fence is FORCED, not
> movable.** The spec posed forced-vs-movable and leaned *movable* (the `into_inner`-less subset holds — it is
> unwind-safe, and `mem::forget` is a leak not a double-free); the ergonomics and expressiveness attackers
> **rejected** this: the sound subset is **degenerate** — it deletes the *by-value move-out* that *defines*
> `take_mut`/`replace_with` (the `T → T`-through-a-non-`T` transform), leaving a primitive **strictly
> dominated by plain `&mut T`**. So every *usable* packaging of the borrowed-slot take breaks horn 1; the
> fence sits exactly at Phase R's **owned-vs-borrowed** line — not pushed inward. (Deflation, pre-registered:
> no novel mechanism — `take_mut`'s `ptr::read`/`write` + `scopeguard`'s `into_inner` defuse + RFC 1066's
> forget-skips-`Drop`, reassembled; contribution is **analytical** — the clean controlled pair and *locating*
> the fence.) **Closing the packaging trilogy: the horn a danger threatens is movable *iff* the value-ownership
> model lets soundness DECOUPLE from the un-compellable closure** — an *owned* value can (#19 ⇒ horn-3 cost);
> a *borrowed* slot cannot, for any usable packaging (#20 ⇒ forced horn-1 break). N stays 9 verdict modes; the
> broken-seam count rises to **six**.

## The recursion: each primitive un-bundled one folded axis

```
unified (negative result)   7 properties, none-of-all; #4 the universal antagonist, #2/#3/#6 forcing functions
        │  project onto the seam
Seam Trilemma               3 horns: {sound, #4 decidable, #2 concentration}
        │                   — bundling #6 INTO "concentrated & proof-free"
        │                   — bundling API-locality INTO "concentration"
        │                   — bundling solver-freedom INTO "decidable"
   slab │  un-bundles #6              ⇒ 3-of-4  (zero-cost is its own axis)
 branded│  un-bundles API-locality    ⇒ 3-of-N  (the type-face of "no-seam" ≠ the code-face)
        │                              + surfaces #7 (legibility), dissociable from #5
 refined│  un-bundles horn 2          ⇒ decidable ✓ vs solver-free ✗; CLOSES #5⟂#7 (off-diagonal)
        │                              (solver-freedom = the `unified` "keep" horn, NOT a new axis)
  shared│  recovers seed #5 bare-metal ⇒ 3-of-≥7  (runtime-manager axis; ⟂ #6 via slab/heapless)
        │                              (= Rust's core/alloc boundary, RFC 2480 — real but pre-existing)
typednat│  PROBES checker-tractability ⇒ REJECTED (off-diagonal confounded by expressiveness;
        │                              cost encoding-relative unary/binary; mislabeled = terminating-
        │                              resolution, Paterson/UndecidableInstances). N stays 7. METHOD STOP.
   take │  recovers unwind-safety       ⇒ 3-of-≥8  (exception-safety-guarantee-level; clean off-diagonal
        │                              vs SPSC — splits on it ALONE; = Abrahams/noexcept/UnwindSafe,
        │                              pre-existing; carries a DISCHARGE MENU: abort / repair / poisoning)
  drain │  recovers leak-freedom        ⇒ 3-of-≥9  (leakpocalypse; pre-pessimization; clean off-diagonal
        │                              vs SPSC AND vs take — leak ⟂ unwind, both directions; = mem::forget-
        │                              is-safe/RFC 1066, pre-existing; the THINNEST axis but genuinely distinct)
 selfref│  PROBES move-stability        ⇒ INSEPARABLE (a distinct danger — a value's address — with NO
        │                              isolable cost: stack-pin→API-locality, heap-pin→bare-metal; cost is
        │                              immovability. N stays 9. NEW VERDICT CATEGORY)
validity│  PROBES bit-level validity    ⇒ PROJECTS ONTO THE BASIS (a NEW danger class, NO tenth axis —
        │                              unanimous) + 2nd INSEPARABLE (per-read→#6, zero-copy view→API-locality).
        │                              THE COST-BASIS IS CLOSED (for memory safety). Arc's conclusion.
 secret │  PROBES a DIFFERENT DOMAIN    ⇒ info-flow is OUT-OF-FRAME (not UB; timing). BREAKS [horn1] in
        │  (constant-time / non-interf.)  the IFC sense (declassification = etiquette; needs undecidable
        │                              whole-program IFC). The TRILEMMA RECURS, sharper. Closure is PER DOMAIN.
 labeled│  PROBES a CLOSING IFC seam   ⇒ ALSO BREAKS (5/5): map/zip hands the closure bare plaintext (no
        │  (capability-gated declassify)  effect system); the "budget" counts events not BITS (affinity bounds
        │                              uses, not amount). NO closing IFC seam on Rust's floor. THE FLOOR is
        │                              matched to memory-safety, MISMATCHED to info-flow (needs Jif/QIF).
ordlocks│  PROBES the POSITIVE half   ⇒ BREAKS [horn1] (FAILED, 3/5). A THIRD domain (liveness/deadlock).
        │  (type-level lock ordering)    Two under-carryings: GRANULARITY (re-entrant Floor::root → 2 lineages
        │                              → typable inversion; per-token ≠ per-thread) + COVERAGE (dynamic
        │                              acyclicity — runtime/same-rank locks — escapes the static floor, like
        │                              IFC's amount escaped the affine one). The floor-match is GRADED: full /
        │                              partial / none. = BLR'02 minus dynamic ext.; Kobayashi nat-levels.
        ▼
registry│  UPGRADES the floor (runtime)  ⇒ BREAKS [horn1] (FAILED, r22). Floor-upgrade is NON-MONOTONE: it
        │  (thread-local lock registry)  FIXED coverage (dynamic orderings close) + paid the price (#5 bare-metal
        │                              — can't be no_std; #6 runtime check; prevention→detection modality), but
        │                              GRANULARITY RECURRED one rung finer — HELD keyed on OS thread, async TASKS
        │                              are the control unit → work-stealing migration launders an undetected
        │                              inversion (thread-blind). Granularity is a LADDER: token < thread < task <
        │                              … ; no ambient Rust floor tops out. = FreeBSD WITNESS/lockdep (learned graphs).
        ▼
 linear │  PROBES a NEW PRECONDITION   ⇒ BREAKS [horn1] (FAILED, r23+r24). Must-run cleanup = LINEARITY,
        │  (must-run cleanup guard)      breadth-wise (a precondition, not a new domain/rung). Affine floor =
        │                              clean NONE (the #[must_use] "partial" REJECTED — scopeguard ships it
        │                              cosmetic). Match-unit is the PRECONDITION not the domain (liveness:
        │                              deadlock PARTIAL, must-run NONE). N unchanged (ENFORCEMENT face of axis
        │                              9 leak-freedom; drain = TOLERANCE face). mem::forget skips Drop (RFC
        │                              1066/1084). NEW MODE wrong-structural-logic: affine≠linear (Boats'
        │                              scoped-task trilemma, linear horn rejected), not domain-mismatch.
        ▼
session │  COMPOSITE precondition       ⇒ HOLDS WITH COST: SEAM-COSTS [horn3] (TRADEOFF, survived r25). FIRST
        │  (session-typed channel)       out-of-domain probe to HOLD, not break. Matched COMPONENT-WISE
        │                              (orthogonal — mem::forget can't shorten 'a to start a 2nd session):
        │                              order-SAFETY CLOSES (affine typestate, no root-hole) / COMPLETION COSTS
        │                              horn-3 (the #17 linearity gap; recurs at every ?/early-exit — de-
        │                              concentrates, doesn't unsound). SAME gap, DIFFERENT horn: it BROKE #17's
        │                              horn1 (completion = whole promise) but only COSTS here (one component
        │                              beside a sound core) — which horn depends on how the rule PACKAGES it.
        │                              Deflations: single-handle WALK not a binary session (E0499); "usable"
        │                              infallible-straight-line only; FOLKLORE (JML'15; ECOOP'22 Stay Safe
        │                              Under Panic) → contribution = LOCATING it. N stays 9; 5 broken unchanged.
        ▼
restore │  CONTROLLED PACKAGING probe  ⇒ HOLDS WITH COST: SEAM-COSTS [horn3] (TRADEOFF, survived r26). #17's
        │  (value-owning scope-restore   IDENTICAL un-compellable-Drop danger wrapped in #18's structure (sound
        │   guard = real scopeguard)     affine-ownership CORE + restore as side-obligation). SAME mem::forget gap
        │                              that BROKE #17's horn1 only COSTS horn3 here → the packaging dial is REAL.
        │                              BUT BOUNDED 3 ways: (1) CONFOUNDED — #19 also adds the value-ownership core
        │                              #17 lacked, so ownership co-varies (better-controlled, not single-variable);
        │                              (2) FORCEABLE — same danger in a borrowed-slot/take_mut packaging FORCES
        │                              horn1 UB (the #9 take danger); Pin's drop-guarantee + thread::scope show
        │                              un-repackageable horns → which horn depends on WHERE SAFETY RESTS, partly
        │                              danger-intrinsic; (3) NOT NOVEL — JoinGuard horn1 / scopeguard horn3 already
        │                              in 2015 (RFC 1066). FINDING: the horn is a design choice WITHIN A STRUCTURAL
        │                              ENVELOPE set by whether soundness DECOUPLES from the closure (owned ⇒ costs;
        │                              borrowed-across-unwind/Pin/thread::scope ⇒ forces). No new mode; N stays 9.
        ▼
 reseat │  FORCING-THE-HORN probe       ⇒ BREAKS [horn1] (FAILED, r27). The SIXTH broken seam, and the controlled
        │  (take-and-reseat guard over   counterpart to #19: keep #19's packaging BYTE-IDENTICAL, flip ONLY owned→
        │   a BORROWED slot = take_mut)  borrowed (the single-variable experiment #19's confound forbade). A 4-line
        │                              SAFE String witness (new/push_str/into_inner) double-frees with no unsafe in
        │                              user code: into_inner returns owned while the slot keeps the ptr::read
        │                              duplicate (E0509 forces mem::forget(self)) → two owners → double-free. TWO
        │                              findings: (1) the horn flip is CLEAN — #19 owned+into_inner cost-3 → #20
        │                              borrowed+into_inner break-1, packaging byte-identical → Phase R's confound
        │                              REPAIRED, flip attributable to the decoupling property ALONE; (2) the fence is
        │                              FORCED not movable — the into_inner-less "sound" subset is DEGENERATE (deletes
        │                              the by-value move-out that DEFINES take_mut, dominated by plain &mut T), so
        │                              every USABLE packaging breaks; the fence sits at the owned-vs-borrowed line.
        │                              Deflation (pre-registered): no new mechanism (take_mut + scopeguard-into_inner
        │                              + RFC 1066). CLOSES THE PACKAGING TRILOGY: the horn is movable IFF the
        │                              ownership model lets soundness DECOUPLE from the closure (owned ⇒ cost; borrowed
        │                              ⇒ forced break). No new mode; N stays 9; broken-seam count → SIX.
        ▼
   crdt │  FOURTH-DOMAIN probe          ⇒ candidate domain DEFLATED. Floor-relatively BREAKS [floor-mismatch]
        │  (state-based CRDT merge =     (TRADEOFF survival, r28): T: JoinSemilattice is a SYNTACTIC PROXY not a
        │   Replica<T:JoinSemilattice>)  witness — a safe non-commutative/non-idempotent join type-checks and
        │                              DIVERGES replicas that saw the same updates (Horn 1, in safe code; closing
        │                              it undecidable by Rice). No unsafe (semantic danger, like secret). BUT the
        │                              candidate FOURTH domain (convergence) DEFLATED, 3 corrections: (1) blessed-
        │                              CRDT TCB rescue NOT MMIO-shaped — lattice space OPEN-ENDED, law-trust recurs
        │                              per user CRDT, never amortizes; (2) 'dischargeable' only at MODEL level (no
        │                              Rust pass checks model→shipped-code); (3) DECISIVE — the gap is the GENERIC
        │                              unchecked-typeclass-law phenomenon (Rust checks SIGNATURES of Ord/Eq/Hash/
        │                              Monoid/semilattice, none of their LAWS) → convergence PROJECTS ONTO #19
        │                              restore's application-correctness basis, NOT a fourth domain. Domain map
        │                              stays at THREE + app-correctness. No new mode; N stays 9; broken-seam count
        │                              stays SIX (this survived/deflated).
        ▼
   wcet │  SECOND FOURTH-DOMAIN probe    ⇒ candidate domain DEFLATED again, and this one FAILED. BREAKS [horn 1],
        │  (cost-typed computation =     FAILED (r29), the SEVENTH broken seam: step charges +1 DECOUPLED from its
        │   Timed<C,T>, a graded cost    closure, so a 1M-iteration loop is cost 1 and within::<1> admits it — the
        │   monad reusing typed_nat)     op-count is no bound on cycles (my §4 over-claimed the deadline guarantee →
        │                              FAILED, not a TRADEOFF like crdt). All 5 attackers landed: (1) soundness —
        │                              op-count ≠ cycles; (2) ergonomics — Horn 3 FALSE (non-unifying if-branches
        │                              force step-padding); (3) impl — unary Peano is O(C), UNCOMPILABLE at real
        │                              10³–10⁷-cycle budgets, only compilable path IS the break; (4) expressiveness
        │                              — no type-level addition, no compositional sequencing; (5) prior-art — this
        │                              is Danielsson's 2008 cost monad; "op-count ≠ cycles" is WCET's FOUNDING
        │                              PREMISE → pick a lane (abstract cost XOR physical deadline). DEFLATES: static
        │                              cost → Danielsson/typed_nat #8 (doesn't even scale); physical claim → an mmio-
        │                              shaped #2 axiom NEVER INSTALLED (worse than MMIO); data-dependent → liveness
        │                              partial-match. Unsound proxy, SHARPER than secret. META: two fourth-domain
        │                              candidates, NEITHER opens a fourth domain — the three domains look SPECIAL.
        │                              No new mode; N stays 9; broken-seam count → SEVEN (this FAILED).
        ▼
 binary │  LIVENESS DEEPENING (NOT a    ⇒ HOLDS. SEAM-COSTS [horn3], TRADEOFF (r30): #18's component-wise
session │  fourth-domain bid): a REAL    grading SURVIVES removing the single-handle limitation. Two dual affine
        │  binary session — two dual     endpoints (Chan<P>/Chan<P::Dual>), BOTH live, over Rc<RefCell<Transport>>
        │  endpoints, both live, over    (alloc + interior mut); E0499 forbade this in #18. Every attacker granted
        │  Rc<RefCell<Transport>>        Horn 1 / no-UB (worst case a starved memory-safe handle). Order CLOSES,
        │                              per-session relative progress CLOSES (HasDual duality), decidable (Θ(depth),
        │                              no SMT), completion COSTS horn-3 (now PEER-observable). BUT the "real binary
        │                              session" framing DEFLATED, 5 corrections: (1) sharpest — relative progress
        │                              closes only PER SESSION (two interleaved session()s type-check a mutual-wait
        │                              deadlock, Kobayashi/Padovani; global deadlock-freedom needs MULTIPARTY
        │                              typing); (2) single-threaded witness → "peer waits forever" self-abandonment-
        │                              flavored, only the STATIC two-live-endpoint limitation lifted (not session_
        │                              types-2015's real concurrency); (3) the fallible recv's Err is NON-ACTIONABLE
        │                              — mem::forget LEAKS the Rc, so the leakpocalypse DOUBLE-BIND (the gap that
        │                              costs completion also denies an actionable abandonment signal); (4) only
        │                              straight-line i32 protocols (no choice/delegation/recursion); (5) citation fix
        │                              (ECOOP'22 = Lagaillardie–Neykova–Yoshida). Net NEW content = two liveness-map
        │                              sharpenings: binary duality gives only PER-SESSION progress; the double-bind
        │                              deepens the #17/#18 linearity gap. No new mode; N stays 9; broken-seam count
        │                              STAYS SEVEN (this survived as TRADEOFF). Liveness DEEPENED, partial-match now
        │                              bounded to per-session.
        ▼
 deleg- │  HIGHER-ORDER SESSION step    ⇒ SOUND, but the gauntlet FAILED on the §6 DELIVERABLE — the REJECTION
 ation  │  (channel-over-channel, the    mode (2nd after typed_nat #8). Horn-classifier read the MECHANISM as
        │  feature #23 lacked): send_    SEAM-COSTS [horn3] (no UB — soundness attacker confirmed no wrong-Q
        │  chan/recv_chan over Rc<Ref    re-typing, duality pins Q; decidable), but the JUDGE ruled FAILED: my
        │  Cell<Transport>>; HasDual     pre-registered "delegation adds NO NEW COORDINATE, projects onto leak-
        │  recurses through payload Q    freedom (#9/#10), a Phase-K closed-basis confirmation" was REFUTED from
        │                              my OWN cited lineage. Delegation's signature hazard (B mem::forgets the
        │                              received Chan<Q>, STARVING C) is "deadlock at the other end" (Jespersen–
        │                              Munksgaard–Larsen, Session Types for Rust, WGP 2015 — this exact proposal
        │                              11yr early) = PROGRESS / DEADLOCK-FREEDOM, a coordinate DISTINCT from
        │                              completion AND leak-freedom. Delegation is THE recognized feature that
        │                              breaks first-order sessions' automatic deadlock-freedom (Coppo–Dezani–
        │                              Yoshida–Padovani MSCS'16 "the communication topology changes because of
        │                              delegation"; Kobayashi; deadlock-free GV — Lindley–Morris, Padovani,
        │                              Multiparty GV PACMPL'22, Rusty Variation'19). CATEGORY ERROR: I saw the Rc
        │                              hazard's memory-leak FACE and mislabeled the progress AXIS; the single-
        │                              threaded witness hides deadlock by construction → "no new coordinate" was
        │                              an artifact of scoping. The symmetric delegation CYCLE is type-obstructed
        │                              (inexpressible infinite type) — rescues BEHAVIOR, not CLASSIFICATION.
        │                              Tradeoffs: no mid-protocol abort verb + Result<_,Self> bans `?` (completion
        │                              undischargeable by the PROGRAMMER); Horn-2 is Θ(depth²) not Θ(depth)
        │                              (HasDual involution normalizes dual(dual(carrier))); finite-type single-
        │                              thread witness can't express the recursive accept-loop server. The sound
        │                              order + don't-use-after-delegate core CLOSES (affine MOVE carries don't-
        │                              use-after-delegate — the one linearity delegation needs). LIVENESS now has
        │                              PROGRESS as a third coordinate (lock seams #15/#16 = its lock face,
        │                              delegation = its session face). N stays 9 (rejection mode, 2nd firing);
        │                              broken-seam count STAYS SEVEN (sound — a deliverable rejection, not a
        │                              soundness break). Probe earned its keep by being instructively WRONG.
        ▼
 multi- │  MULTIPARTY (n-party global  ⇒ SOUND, but the gauntlet FAILED the §6 DELIVERABLE AGAIN — the
 party  │  progress; NOT a fourth-     REJECTION mode's THIRD firing (after typed_nat #8 + delegation #24).
        │  domain bid): a 3-party      Horn-classifier read the MECHANISM as SEAM-COSTS [horn3] (no UB — a
        │  session from #23's binary   memory-safe recv-Err hang; decidable), but the JUDGE ruled FAILED:
        │  edges; a RecvThenSend role  my pre-registered "global progress lives ONLY whole-protocol, in a
        │  enforces recv-before-send;  GENERATIVE global-type projection; no per-role discipline can reject
        │  ring() type-checks a        the ring" was a FALSE DICHOTOMY — refuted (3rd time) from my OWN
        │  source-less deadlock        cited lineage. The omitted MIDDLE RUNG: per-channel PRIORITY/
        │                              OBLIGATION typing (Kobayashi linear-π; Padovani; Dardha–Gay FoSSaCS
        │                              2018; Kokke–Dardha) rejects the source-less ring COMPOSITIONALLY,
        │                              with NO global type, via prio(recv) < prio(send) → a contradictory
        │                              p < p on the cycle — and I MIS-FILED Kobayashi as SUPPORT when he is
        │                              the COUNTEREXAMPLE. THREE-RUNG taxonomy: (a) per-edge HasDual floor
        │                              [doesn't carry]; (b) per-channel priority typing + whole-program
        │                              acyclicity [no global type]; (c) generative MPST projection.
        │                              SURVIVES (weaker truth): global progress is NOT concentrable per-
        │                              call, needs strictly more than the per-edge floor — but it is WHOLE-
        │                              PROGRAM, not necessarily GENERATIVE. Tradeoffs: horn-3 "concentration
        │                              confirmed" rests on a degenerate single-shot fixed-N=3 single-thread
        │                              witness (the "deadlock" is "no source → recv-Err", not a concurrent
        │                              hang). The sound role-local order + linearity core CLOSES (Lean
        │                              multiparty_role_order_core_closes, like typed_nat/delegation).
        │                              LIVENESS keeps PROGRESS as its third coordinate (lock #15/#16 = lock
        │                              face, delegation = session face, multiparty = global-topology face).
        │                              N stays 9 (rejection, 3rd firing); broken-seam count STAYS SEVEN
        │                              (sound — deliverable rejection, not a soundness break). 25 primitives;
        │                              instructively WRONG a third time.
        ▼
priority│  PRIORITY TYPING (the rung    ⇒ SOUND-intent, but BREAKS [horn1], FAILED (r33) — the EIGHTH
        │  #25 named): per-channel       broken seam, the FIRST soundness break since wcet #22 (FAILED tally
        │  priority/obligation typing    7→8). I predicted SEAM-COSTS [horn3]; the horn-classifier moved it to
        │  = Relay<Pi,Po>, recv-then-    horn1. Built over #23's binary edges + #8's sealed typed_nat::Lt (Peano
        │  sends under Pi: Lt<Po>;       <, no reflexive Lt<N>) to CONFIRM #25's weaker thesis humbly — and broke
        │  Kobayashi linear-π /          it instead. THE FLAW: the "ring is untypable" argument (p_CA < p_AB <
        │  Dardha-Gay FoSSaCS 2018       p_BC < p_CA collapsing to a contradictory p: Lt<p>) SECRETLY ASSUMES
        │                              EDGE-PRIORITY CONSISTENCY — that the Nat A SENDS on edge AB is the Nat B
        │                              RECEIVES on AB. Rust does NO transitive reasoning; it checks each Pi:
        │                              Lt<Po> in ISOLATION, and the contradictory chain forms only when ONE Nat
        │                              is shared per edge — which only the coherent relay() enforces. So the
        │                              PUBLIC role::<Pi,Po> lets three role::<N1,N2> relays (every bound N1:
        │                              Lt<N2>, e.g. 1<2, discharging locally) wire into the source-less ring:
        │                              it COMPILES, no unsafe, and DEADLOCKS — falsifying §4.3's "deadlock-
        │                              freedom of any well-typed assembly." THE STING: I had DESCRIBED this
        │                              exact flaw in §4 ("edge consistency is the constructor's invariant, not
        │                              a per-endpoint type fact") and shipped role() public anyway; the
        │                              soundness attacker quoted my own words back, by a passing test. It
        │                              CONFIRMS #25 BY BREAKING, not costing: the per-channel Lt bound carries
        │                              NOTHING GLOBAL (never names the dual endpoint), deadlock-freedom rests
        │                              on a TRUSTED edge-consistent constructor (a mini-projection) — salvaging
        │                              to SEAM-COSTS [horn3] only if role() is HIDDEN. UNIFICATION: the SAME
        │                              break as ordered_locks #15/#16, one soundness sub-domain over — type-
        │                              level order carries the LOCAL bound, whole-program consistency/
        │                              acyclicity ESCAPES (the wrong-granularity gap); progress's LOCK face
        │                              (#15/#16) and SESSION face (#26) break IDENTICALLY. Liveness stays the
        │                              third domain, progress its third coordinate (lock #15/#16 = lock face,
        │                              delegation = session face, multiparty = global-topology face, priority
        │                              = the per-channel-order face that BREAKS); N stays 9 (the BREAKS [horn1]
        │                              verdict is one of the nine modes); broken-seam count → EIGHT (Lean
        │                              priority_typed_discipline_factors, a ..._factors like ordered_locks).
        │                              26 primitives; the probe earned its keep by being instructively WRONG
        │                              about its OWN predicted verdict.
        ▼
project-│  SALVAGE of #26 (priority):  ⇒ SOUND, but the gauntlet FAILED the §6 DELIVERABLE — the REJECTION mode's
ed      │  SEAL worker() module-priv,  FOURTH firing (after typed_nat #8, delegation #24, multiparty #25; #26 was a
        │  vend only trusted relay /   BREAK, not a rejection). The SALVAGE of #26 WORKS: sealing worker() module-
        │  relay4 builders threading   private and vending only trusted relay/relay4 builders (threading strictly-
        │  strictly-increasing Nats    increasing Nats per edge) makes the #26 ring UNCONSTRUCTIBLE twice over — (1)
        │  per edge                    ACCESS (no public worker() to wire freely) and (2) STRUCTURE (a monotone chain
        │                              cannot close: the closing edge needs a DECREASING Lt, no instance). Horn-
        │                              classifier read the MECHANISM as SEAM-COSTS [horn3] (Horn 1 conceded), BUT the
        │                              JUDGE ruled FAILED on the §6 deliverable. THE OVERCLAIM: §4 staked "deadlock-
        │                              freedom by construction" + "the consumer carries NOTHING GLOBAL." The builder
        │                              proves only that a deadlock-free SCHEDULE EXISTS (a static Nat witness); affine
        │                              Rust never FORCES the runtime to follow it — a run-all/in-order/no-drop PROGRESS
        │                              RESIDUE stays on the consumer. Honest: "SCHEDULABILITY by construction, GIVEN
        │                              all roles execute." THE GIVEAWAY: Scribble/mpstthree endpoints are deadlock-free
        │                              ONLY assuming every participant executes — I imported that guarantee while
        │                              claiming the consumer carries nothing. CONCENTRATION PARTIAL: the constructor
        │                              DOES carry TOPOLOGY (no cycle — the #26 fix is real), but COMPLETION/EXECUTION
        │                              never left the consumer (#17/#18/#23's residue, recurring). SECONDARY: the
        │                              expressiveness cost was OVERstated — acyclic generality is recoverable via one
        │                              generic combinator (mpstthree). REFINED FINDING: the PROGRESS COORDINATE
        │                              DECOMPOSES — deadlock-freedom/topology (the trusted constructor carries) vs
        │                              completion/execution (the consumer carries) — sharpening the #24–26 arc with a
        │                              cleaner split. The sound topology-carrying core CLOSES (Lean projected_schedul-
        │                              ability_closes, a ..._closes like typed_nat/delegation/multiparty). Liveness
        │                              stays the third domain, progress its third coordinate (lock #15/#16 = lock face,
        │                              delegation = session face, multiparty = global-topology face, priority = per-
        │                              channel-order break face, projected = the SCHEDULED-NETWORK face that salvages-
        │                              but-residues); N stays 9 (rejection, 4th firing); broken-seam count STAYS EIGHT
        │                              (sound — a deliverable rejection, not a soundness break). 27 primitives; the
        │                              probe earned its keep by COMPLETING the salvage and EXPOSING what it still
        │                              cannot carry.
        ▼
choice  │                              ⇒ SOUND, but the gauntlet FAILED the §6 DELIVERABLE — the REJECTION mode's
        │                              FIFTH firing (after typed_nat #8, delegation #24, multiparty #25, projected
        │                              #27). The BRANCHING connective (offer/&, select/⊕) — the LAST unprobed core
        │                              session feature — turned back INSIDE liveness. THE BREAK: §4 guarantee #2
        │                              "handle every label" is VACUOUS — Rust exhaustiveness gives SCRUTINEE
        │                              COVERAGE, not run-to-End, so a SAFE match offer() { Left(c) => .., _ => {} }
        │                              type-checks, no unsafe, and DROPS Chan<R>, starving that branch. "Handle-
        │                              all" collapses into the conceded #17/#18/#23/#27 COMPLETION RESIDUE. FOUR
        │                              corrections: (1) branch-safety is TWO-LEGGED (label-continuation + branch-
        │                              set duality), order is the generic typestate leg; (2) deflation is onto
        │                              BINARY Either, NOT n-ary NAMED sums; (3) Horn 2 is Θ(type-size), not
        │                              Θ(depth); (4) completion is AMPLIFIED per-branch and PEER-CHOICE-REACHABLE.
        │                              SURVIVES: choice adds NO NEW COORDINATE — the last core session feature
        │                              deflates onto sum types; completion stays the residue, now fed by TWO safe
        │                              paths (mem::forget AND the dropped branch). Lean choice_branch_typing_closes
        │                              (a ..._closes — branch TYPING closes, not completion). N stays 9 (rejection's
        │                              5th firing); broken-seam count STAYS EIGHT (sound — a deliverable rejection);
        │                              liveness the third domain. 28 primitives; the LAST core session connective
        │                              opens no new ground.
        ▼
must-cmp│                              ⇒ SOUND, but the gauntlet FAILED the §6 DELIVERABLE — the REJECTION
        │                              mode's SIXTH firing (after typed_nat #8, delegation #24, multiparty
        │                              #25, projected #27, choice #28) — and the MOST INSTRUCTIVE: it REVERSES
        │                              my finding. A Session<P> affine handle with a Drop COMPLETION BOMB
        │                              (cf. take #9) attacks the COMPLETION RESIDUE itself (#17/#18/#23/#27).
        │                              I pre-registered SEAM-BREAKS — completion an IRREDUCIBLE linear gap,
        │                              the session face of linear #17. THE MECHANISM break is real (a free
        │                              handle's bomb is mem::forget-bypassed, starving the peer) but only
        │                              shows the WITNESS wrong. THE DECISIVE refutation: my "UNIVERSAL NONE"
        │                              is FALSE — a scoped-closure/CPS combinator with_session(|s| …), the
        │                              std::thread::scope ANALOGUE, COMPELS completion modulo divergence, no
        │                              unsafe, by moving the obligation to the RE-IMPORT BOUNDARY (no
        │                              forgettable free handle). THE STING: I cited thread::scoped's REMOVAL
        │                              (RFC 1084) but ignored its FIX, thread::scope (stable 1.63). THE
        │                              REVERSAL: completion is NOT irreducible — it is CONCENTRABLE at a
        │                              horn-3 API-LOCALITY cost (surrender the free handle); the #17–#28
        │                              residue was the PRICE OF THE FREE-HANDLE API. Sessions ≠ #17's cleanup
        │                              — a delimited session CAN be scoped, open-ended cleanup cannot
        │                              (refuting deflation (a)). FURTHER: ergonomics (the bomb turns ? into
        │                              process-abort, unusable on fallible code); prior-art (peer starvation
        │                              fixable by CANCELLATION PROPAGATION — Mostrous–Vasconcelos Affine
        │                              Sessions; ECOOP 2022 Stay Safe Under Panic; MultiCrusty; EGV).
        │                              SURVIVES: decidability (Θ(depth); leak-detection NONE conceded). META
        │                              — this INVERTS the usual error: I overstate the EXTREMES (total-carry/
        │                              irreducible) when the truth is a cost BETWEEN. Lean must_complete_
        │                              scoped_completion_closes (a ..._closes — the SCOPED witness closes
        │                              completion at an API-locality cost; the free-handle witness only
        │                              FACTORED). N stays 9 (rejection's 6th firing); broken-seam count STAYS
        │                              EIGHT (SEAM-COSTS, not the break I predicted); liveness the third
        │                              domain. 29 primitives; the scoped salvage with_session is the natural
        │                              #30 (mirroring #26→#27).
        ▼
Realizability table         MEMORY-SAFETY domain (on a MATCHED affine floor): N ≥ 9 CLOSED axes (sound,
                            decidable, concentration, API-locality, #6, #7, bare-metal, unwind-safety,
                            leak-freedom) + INSEPARABLE (move-stability, validity) + 2 outside-the-axes
                            (MMIO TCB; intrusive horn-drop). INFORMATION-FLOW domain (on a MISMATCHED floor):
                            its own basis (declassification, quantitative leakage) but NO closing seam — both
                            IFC seams break. LIVENESS domain: on a static-order floor a PARTIAL match (the
                            static fragment closes, granularity + dynamic escape); UPGRADING to a runtime
                            registry floor is NON-MONOTONE — coverage lifts (paying #5 + #6 + modality) but
                            granularity RECURS thread→task (a containment-scope LADDER no ambient floor tops);
                            and a DIFFERENT liveness precondition — must-run-cleanup — grades a clean NONE and
                            BREAKS on wrong-structural-logic (affine≠linear), so the match-unit is the
                            PRECONDITION, not the domain. And a COMPOSITE precondition (session-typed channel)
                            is matched COMPONENT-WISE and HOLDS WITH COST (SEAM-COSTS [horn3]): order-safety
                            CLOSES, completion COSTS horn-3 — the same linearity gap that BROKE #17 only de-
                            concentrates here, because which horn it threatens depends on how the rule PACKAGES
                            it. And a CONTROLLED PACKAGING probe (value-owning scope-restore guard) turned that
                            into a deliberate experiment — HOLDS WITH COST (SEAM-COSTS [horn3], r26), the dial
                            REAL but BOUNDED: the move is confounded (ownership co-varies), the horn is forceable
                            (borrowed-slot/take_mut → horn1 UB; Pin / thread::scope un-repackageable), so the horn
                            is a design choice WITHIN A STRUCTURAL ENVELOPE set by whether soundness DECOUPLES from
                            the un-compellable closure. And the FORCING-THE-HORN counterpart (take-and-reseat guard
                            over a BORROWED slot, r27) hit the fence from the other side — BREAKS [horn1], the SIXTH
                            broken seam: the clean single-variable flip (#19 owned cost-3 → #20 borrowed break-1,
                            packaging byte-identical) REPAIRS the confound, and the fence is FORCED not movable (the
                            into_inner-less subset is degenerate, dominated by &mut T). The packaging trilogy closes:
                            the horn is movable IFF the ownership model lets soundness DECOUPLE from the closure
                            (owned ⇒ cost; borrowed ⇒ forced break). And a FOURTH-DOMAIN probe (state-based CRDT
                            merge, r28) looked for a fourth soundness domain — distributed CONVERGENCE — and found
                            NONE: floor-relatively it BREAKS [floor-mismatch] (the JoinSemilattice bound is a
                            syntactic proxy; a safe lawless join diverges replicas), but the candidate domain
                            DEFLATED — the gap is the GENERIC unchecked-typeclass-law phenomenon (Rust checks the
                            signatures of Ord/Eq/Hash/Monoid/semilattice, none of their laws), so convergence
                            PROJECTS ONTO restore's application-correctness basis, NOT new ground. And a SECOND
                            FOURTH-DOMAIN probe (cost-typed computation, r29) looked for real-time / WCET and found
                            NONE — but FAILED where convergence deflated: BREAKS [horn 1], FAILED, the SEVENTH broken
                            seam (step charges +1 decoupled from its closure, so within::<1> admits a million-cycle
                            computation — §4 over-claimed the deadline). Five-fold break (Horn 3 FALSE; unary cost
                            O(C)-uncompilable at real budgets; no additive composition; Danielsson 2008 cost monad —
                            "op-count ≠ cycles" is WCET's founding premise). DEFLATES: static cost → typed_nat #8
                            (doesn't scale); physical claim → an mmio-shaped #2 axiom never installed; data-dependent
                            → liveness partial-match. The op-count is an UNSOUND proxy, sharper than secret's. So the
                            domain map STILL stays at THREE (memory / IFC / liveness) + the application-correctness
                            basis — TWO fourth-domain candidates probed, NEITHER opens one; the three look SPECIAL.
                            And a LIVENESS-DEEPENING probe (a REAL binary session — two dual affine endpoints, both
                            live, over Rc<RefCell<Transport>>, r30) removed #18's single-handle limitation and
                            HOLDS WITH COST (SEAM-COSTS [horn3], TRADEOFF): #18's component-wise grading SURVIVES —
                            order CLOSES, per-session relative progress CLOSES (HasDual), completion COSTS horn-3,
                            now PEER-observable. But the "real binary session" framing DEFLATED to "#18 with a
                            duplicated handle and a RefCell between the copies": relative progress closes only PER
                            SESSION (two interleaved sessions type-check a mutual-wait deadlock — global deadlock-
                            freedom needs MULTIPARTY/Kobayashi typing), the single-threaded witness makes "peer waits
                            forever" self-abandonment-flavored (only the STATIC two-live-endpoint limitation is
                            lifted), and the fallible recv's Err is NON-ACTIONABLE because mem::forget leaks the Rc
                            (the leakpocalypse DOUBLE-BIND: the gap that costs completion also denies an actionable
                            abandonment signal). Net new content = two liveness-map sharpenings (binary duality gives
                            only per-session progress; the double-bind deepens the #17/#18 linearity gap). Liveness
                            stays the THIRD domain, now DEEPENED; N stays 9; broken-seam count STAYS SEVEN (survived).
                            The Seam Trilemma is DOMAIN-GENERAL, the cost-basis
                            PER-DOMAIN, and WHICH SEAMS CLOSE depends on the FLOOR-DOMAIN MATCH — GRADED (full /
                            partial / none), set per PRECONDITION not per domain, and not monotonically liftable.
                            Nine method modes:
                            confirm / sharpen / reject / inseparable / projects-onto-basis / out-of-frame /
                            floor-mismatch / partial-match / wrong-structural-logic.
```

Each constructive primitive is the **witness that two trilemma-identical configurations differ** —
the only way to prove a folded axis was ever really there. The slab proved a concentrated seam can
still cost runtime; the branded index proved a concentrated, erasing seam can still cost the type
surface; the refined index proved legibility (#7) is its own axis (the brand-free-yet-legible
off-diagonal the slab-vs-branded pair could not reach) **and** that horn 2 was a bundle of
*decidability* and *solver-freedom*; the shared seam recovered seed #5 (bare-metal), the
runtime-manager axis, distinct from runtime *cost* (#6).

**The honest meta-pattern.** *Every* axis the method recovers turns out to be an **already-named PL
concern**: zero-cost abstraction (#6), generativity/API-locality, refinement-legibility (#7), the
`core`/`alloc` bare-metal split (RFC 2480), exception-safety (Abrahams' basic/strong/nothrow;
`noexcept`; `UnwindSafe`), and now leak-freedom (the leakpocalypse; `mem::forget`-is-safe; linear-vs-
affine types). The constructive phase did not discover new physics. What it found is that the trilemma's
three horns silently *bundle together a set of independently known PL design axes*, and that one rubric
— **"what does the safe witness cost?"** — unifies them. The contribution is the unification and the
method that recovers each axis (build the dissociating witness, let a cold panel adjudicate), not the
novelty of any single axis. The adjudicator showed **five distinct modes**, which is exactly what makes
it an instrument and not a rubber stamp: it **confirmed-but-deflated** three new dimensions (bare-metal,
unwind-safety, leak-freedom → "real, but pre-existing", growing N to 9); it **sharpened without
extending** once (solver-freedom = horn 2's "keep" face, not a new axis); it **rejected outright** once
(checker-tractability — off-diagonal confounded with expressiveness); it found two **real but
structurally inseparable** dangers (move-stability, validity — distinct dangers with no isolable cost; a
recurring category); and — the capstone — it ruled a new danger class **projects entirely onto the
existing basis** (validity adds no tenth axis), the evidence that **the cost-basis is closed**. The seam
tension is where *known* axes meet; the panel recovers the clean ones, refuses the confounded, recognises
the inseparable, and certifies when a new danger adds nothing. A discovery procedure with only a "yes" is
a rubber stamp; this one has "yes", "yes but pre-existing", "no — confounded", "sharpens not extends",
"real but inseparable", and "real but it projects onto what's already here" — each on the record, the
last one closing the basis.

## Structure among the axes (not just a count)

The axes are not independent toggles you can set freely — **two trades and a discharge menu** show up:

- **#5 ↔ #6 (the branded index, and again the refined index).** The range-branding that cashes in
  zero-cost (#6) *deepens* the API-locality tax (#5). The refined index hits the *same* trade from
  the other side — the cold panel's **"R2 trilemma"**: a refined index can be *brand-free + sound*
  (pay a runtime `bound ≤ len` check — not zero-cost) or *zero-cost + sound* (tie the length in the
  type = a viral length-index brand), but not both. So #5 and #6 do not just dissociate, they
  *exchange* — a design exchange-rate, not a free lunch.
- **horn 2 = decidability ⊗ solver-freedom.** The refined index is decidable yet not solver-free;
  pushing the predicate fragment up to discharge real index math (bitwise/modular/stride) only
  deepens the solver dependence. Note the honesty guard: solver-freedom is **not a new 7th axis** —
  it is the `unified` result's "keep the invariant faithfully → external prover" horn, the
  documented founding tradeoff of DML (1998) / Liquid Types (2008) / Flux (2023), now seen at the
  seam. The map gains *resolution*, not a dimension.
- **The unwind-safety *discharge menu* (the take-and-replace seam).** Unlike the first seven axes,
  whose cost lands in one fixed place, the eighth axis can be *discharged in more than one place* — the
  panel's mandatory correction. The obligation (survive an unwind through a broken invariant) is one
  axis; *where you pay it* is a free design variable: **abort** (this seam — sound, but downgrades the
  basic guarantee, non-recoverable), **placeholder-repair** (a `replace_with` sentinel — sound *and*
  recoverable, pays **API-surface / `T: Default`**), or **poisoning** (a lock — sound, pays a
  one-way-latch on availability). So the axis is the *guarantee-level*; the abort is one coordinate on
  it. This is a third kind of inter-axis structure — not a two-axis exchange, but one axis with a menu
  of discharge points that each spend a *different other* resource.

## Honest bounds

- N ≥ 9 is a **lower bound**: nine gauntlet-confirmed live axes — sound, decidable, concentration,
  API-locality, zero-cost (#6), AI-legibility (#7), bare-metal (#5, shared phase), unwind-safety
  (exception-safety-guarantee-level, take phase), and leak-freedom (drain phase). More may exist; the
  method (build a dissociating witness, let the cold panel adjudicate) is open-ended, not exhausted.
  **But "open-ended" is not "everything is an axis":** the eighth axis took *two* probes. The first
  (type-level index → checker-tractability) was built, attacked, and **rejected** — confounded with
  expressiveness, encoding-relative, a mislabeled re-name of a pre-existing terminating-resolution
  concern (Paterson / `UndecidableInstances`). The second (take-and-replace → unwind-safety) supplied a
  *clean* off-diagonal and was **confirmed**; the ninth (drain-and-yield → leak-freedom) likewise. So N
  reached 9 only on witnesses that hold every other axis fixed — the rejection of the confounded
  candidate is what makes the confirmations trustworthy. A tenth probe (self-ref cell → move-stability)
  found the **inseparable** category (a distinct danger with no isolable cost; N stays 9), and an
  eleventh (validity → bit-level validity) found *both* a **second** inseparable danger *and* the
  **closed-basis** result: a brand-new danger class that adds **no tenth axis**, every witness spending a
  pre-existing coordinate. So the honest bound is now: **≥ 9 clean axes + an inseparable category (≥ 2
  members) + 2 outside-the-axes (TCB axiom, horn-drop), and the basis is *closed* under the dangers
  probed** — "more may exist" remains true (it is a lower bound), but the *evidence* of a new danger
  projecting cleanly onto the basis, and inseparability recurring, is what closure looks like. The four
  shapes a *within-domain* probe can take are all attested: a clean axis, a confound, an inseparable, or a
  plain projection onto the basis — **plus three cross-domain shapes: *out of frame* (the secret seam — a
  different soundness domain), *floor-mismatch* (the labeled seam — a domain whose seams none close on this
  floor), and *partial-match* (the lock-ordering seam — a domain that closes *partway*).** The closure is
  therefore **per memory-safety domain**; information-flow has its own basis (declassification discipline,
  …) and its own — sharper — trilemma, with *no* seam closing on Rust's floor; liveness has its own basis
  (lock-order-inversion freedom, dynamic acyclicity) with a *partial* closure. The deepest bound is not
  "N ≥ 9 closed" but "**each soundness domain has its own basis, the seam tension is the same shape in
  every one, and *which seams close* is set by the floor-domain match — which is graded full / partial /
  none**" (memory safety mapped on a matched floor, information flow sketched on a mismatched one, liveness
  sketched on a partially-matched one).
- The #5⟂#7 dissociation **is witnessed** (refined index = brand-free + legible; slab = brand-free +
  illegible). Bare-metal ⟂ #6 **is witnessed** (slab = bare-metal + runtime-cost; Shared = global-heap
  + runtime-cost). Unwind-safety ⟂ all seven **is witnessed** (take-and-replace vs SPSC: the cleanest
  single-coordinate flip). Leak-freedom ⟂ unwind-safety **is witnessed both directions** (take =
  unwind-sensitive/leak-safe; drain = leak-sensitive/unwind-safe) — the two destructor-execution
  siblings are genuinely distinct, not one merged axis. The refined phase surfaced that horn 2
  un-bundles into decidability + solver-freedom — that face is the original "keep" horn, so it sharpens
  the map rather than extending N.
- **Novelty is bounded, and that is the honest headline.** Every recovered axis the cold panel deflated
  to "real but *pre-existing*" — solver-freedom (Liquid-Types' solver-in-TCB), bare-metal (RFC 2480's
  `core`/`alloc`), unwind-safety (Abrahams' exception-safety; RFC #1508 rejected the very `take_mut`
  scored here), and leak-freedom (Rust's leakpocalypse; RFC 1066; the `Vec::drain` leak-amplification
  fix). The lattice's value is *unification under one cost rubric*, not novel dimensions. Treat "N ≥ 9"
  as "at least nine *known* PL axes are all faces of the seam tension," not "nine discoveries." A note
  on *thinness*: the panel flagged leak-freedom as the thinnest, most-deflatable axis (its cost is
  invisible in non-leak code), yet still real — a reminder that "deflatable" and "not an axis" are
  different verdicts (the rejected checker-tractability probe is the latter; leak-freedom the former).
- The Lean kernel (`../proof/SeamGap.lean`) sees only whether a precondition factors through a
  Boolean witness — so it certifies *closure* (`spsc`/`branded`/`refined`/`shared`/`typed_nat`/`take`/`drain`/`selfref`/`validity` — and `session_order_safety_closes`, the eighteenth seam, whose *order-safety* component closes like a holding seam (it HOLDS with a horn-3 completion cost the kernel cannot see), and `restore_ownership_core_closes`, the nineteenth seam, whose *ownership* core closes the same way (it HOLDS with a horn-3 restore cost — the controlled-packaging probe: the same `mem::forget` gap that broke #17 only costs here, but bounded — the horn is forceable in a borrowed-slot/`take_mut` packaging), and `binary_session_progress_core_closes`, the twenty-third seam — the real-binary-session liveness deepening — whose *order + per-session relative-progress* core closes the same way (it HOLDS with a horn-3 completion cost the kernel cannot see; the panel deflated the "real binary session" framing — per-session-only progress since interleaved sessions deadlock, a non-actionable `Err` from the leaked `Rc`, a single-threaded straight-line witness — but that lives in the prose, not the checked sub-fact), and `delegation_handoff_core_closes`, the twenty-fourth seam — the higher-order channel-over-channel step — whose *order + don't-use-after-delegate* core closes the same way (the affine MOVE carries don't-use-after-delegate; SOUND, like `typed_nat`), but whose **§6 deliverable the gauntlet REJECTED** (the rejection mode's 2nd firing): delegation opens **progress / deadlock-freedom** as a coordinate distinct from completion and leak-freedom — "deadlock at the other end" (Jespersen et al. 2015; Coppo–Dezani–Yoshida–Padovani; Kobayashi) — which I mislabeled as a leak-freedom projection, an error the single-threaded witness hid; that classification correction lives in the prose, not the checked sub-fact, and `multiparty_role_order_core_closes`, the twenty-fifth seam — the multiparty n-party-progress step — whose *role-local order + linearity* core closes the same way (sound, decidable, like `typed_nat`/`delegation`), but whose **§6 deliverable the gauntlet REJECTED** (the rejection mode's 3rd firing): I staked a FALSE DICHOTOMY — that global progress lives *only* whole-protocol in a *generative* global-type projection — refuted from my own miscited lineage (per-channel priority/obligation typing — Kobayashi, Padovani, Dardha–Gay FoSSaCS 2018 — rejects the source-less `ring()` compositionally with no global type, restoring the three-rung taxonomy: per-edge floor / per-channel priority typing / generative MPST); that correction lives in the prose, not the checked sub-fact, and `projected_schedulability_closes`, the twenty-seventh seam — the SALVAGE of #26 (priority) — whose *topology* core closes the same way (sealing `worker()` module-private and vending only trusted `relay`/`relay4` builders that thread strictly-increasing `Nat`s per edge makes the #26 ring unconstructible: no public `worker()`, and a monotone chain cannot close — the closing edge needs a decreasing `Lt` with no instance; SOUND, like `typed_nat`/`delegation`/`multiparty`), but whose **§6 deliverable the gauntlet REJECTED** (the rejection mode's 4th firing): I over-claimed "deadlock-freedom by construction" + "the consumer carries nothing global" when the builder proves only that a deadlock-free *schedule* EXISTS — affine Rust never FORCES the runtime to follow it, so a run-all/in-order/no-drop progress residue stays on the consumer (the honest claim is "schedulability by construction, given all roles execute"; the progress coordinate DECOMPOSES — deadlock-freedom/topology the trusted constructor carries, completion/execution the consumer carries); that correction lives in the prose, not the checked sub-fact, and `choice_branch_typing_closes`, the twenty-eighth seam — the branching connective (`offer`/`select`, `&`/`⊕`), the last unprobed core session feature — whose *order + branch-typing* core closes the same way (label–continuation correlation + branch-set duality concentrate in the floor's sum types; SOUND, like `typed_nat`/`delegation`/`multiparty`/`projected`), but whose **§6 deliverable the gauntlet REJECTED** (the rejection mode's 5th firing): guarantee #2 "handle every label" is vacuous — exhaustiveness gives *scrutinee coverage*, not run-to-`End`, so a safe wildcard `match` drops a branch, collapsing handle-all into the #17/#18/#23/#27 completion residue; branch-safety is two-legged, and choice adds no new coordinate (deflating onto binary `Either`); that correction lives in the prose, not the checked sub-fact, and `must_complete_scoped_completion_closes`, the twenty-ninth seam — the must-complete session handle (a `Session<P>` affine handle with a `Drop` completion bomb, cf. `take` #9, attacking the completion residue itself after the connectives were exhausted at #28) — whose *completion* core closes via the **SCOPED witness** (a `with_session(|s| …)` scoped-closure / CPS combinator, the `std::thread::scope` analogue, COMPELS completion modulo divergence with no `unsafe` by moving the obligation to the re-import boundary; the free-handle `Drop`-bomb witness only *factored*, being `mem::forget`-bypassable), but whose **§6 deliverable the gauntlet REJECTED** (the rejection mode's 6th firing, the most instructive — it REVERSES my finding): I pre-registered SEAM-BREAKS ("completion is an IRREDUCIBLE linear gap, the session face of `linear` #17"), and the r36 panel REFUTED my "universal none" — completion is NOT irreducible but CONCENTRABLE at a horn-3 API-locality cost (surrender the free handle; the #17–#28 residue was the price of the FREE-HANDLE API), so the verdict is SEAM-COSTS [horn3-concentration], *not* the break I predicted, and the FAILED tally STAYS EIGHT; the sting was citing `thread::scoped`'s removal (RFC 1084) while ignoring its fix `thread::scope` (stable 1.63); that correction lives in the prose, not the checked sub-fact — (+ `secret`/`labeled`/`ordered_locks`/`lock_registry`/`linear`/`reseat`, which certify only the trivial typed sub-fact — the two IFC seams break, three liveness-domain seams break (the static lock-ordering one partially-matches, the runtime registry shows the floor-upgrade is non-monotone, and the must-run-cleanup one is a clean NONE breaking on wrong-structural-logic, affine≠linear), and `reseat_typed_discipline_factors`, the twentieth seam, breaks on a forced horn-1 double-free — the controlled counterpart to #19 that proves the packaging fence FORCED at owned-vs-borrowed: `into_inner` over a borrowed slot hands out the value while the slot keeps the `ptr::read` duplicate, so the same packaging that *held* for #19's owned value breaks here), and `crdt_typed_discipline_factors`, the twenty-first seam, breaks *floor-relatively* — the first probe at a fourth domain (distributed convergence), where a safe lawless `join` diverges replicas because `T: JoinSemilattice` is a syntactic proxy not a witness; but the candidate fourth domain DEFLATED — convergence is the generic unchecked-typeclass-law phenomenon, projecting onto `restore`'s application-correctness basis, not new ground (the domain map stays at three + application-correctness; the FAILED broken-seam count stays six — this one survived as a TRADEOFF), and `wcet_typed_discipline_factors`, the twenty-second seam, breaks *and FAILED* — the second probe at a fourth domain (real-time / WCET), where `step` charges one unit decoupled from its closure's work so `within::<1>` admits a million-cycle computation (the op-count is an unsound proxy for real time, sharper than `secret`'s); this candidate fourth domain DEFLATED too — onto Danielsson's 2008 cost monad / `typed_nat` #8 (which does not scale), an `mmio`-shaped #2 axiom never installed, and liveness's partial-match — so two fourth-domain candidates yield no fourth domain, and `priority_typed_discipline_factors`, the twenty-sixth seam, breaks on a forced horn-1 deadlock — the per-channel priority-typing rung #25's rejection named, where a *public* `role::<Pi, Po>` constructor lets three locally-valid `Pi: Lt<Po>` relays wire into a source-less ring that COMPILES, no `unsafe`, and DEADLOCKS (the same break as `ordered_locks` #15/#16, one soundness sub-domain over: type-level order carries the local bound, whole-program edge-consistency/acyclicity escapes) (the FAILED broken-seam count rises to eight; the breaking `..._factors` family is now nine, but `crdt` is the deflation-survival, so the FAILED tally is eight)
  all close, like SPSC) but is blind to *which axis a closing seam spends* — or whether that "axis" is
  even separable (the self-ref cell closes in the kernel exactly like the rest; its inseparability is
  pure modeling-dictionary). Every column past
  "concentration" lives in this modeling-dictionary, by the same honest caveat the slab's `Vrt` carries.
- One artifact caveat the panel found and we fixed: the reference model's R2 used `bound == len`
  (strictly stronger than safety's `bound ≤ len`); relaxed to `≤` (predicate subtyping). The shipped
  `seam/src/refined.rs` is faithful to the floor it models on this point.
