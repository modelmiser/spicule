# Build-and-break the seam — A→B→C synthesis

The negative half of this project proved keeping all seven properties is impossible and
distilled the **Seam Trilemma**: a seam cannot simultaneously have (1) sound re-import,
(2) a decidable, proof-free safe fragment, and (3) seam concentration. The constructive
phase asked the next question: *take the trilemma as a map and actually build a seam.* The
stateful builder authored each seam spec + a real `rustc`-checked implementation; the cold
adversarial harness (5 attackers + judge + an independent horn-classifier) red-teamed the
**re-import rule only**, never the author. Eighteen primitives, chosen so their preconditions have
eighteen different *shapes* — and so that each later one becomes the witness un-bundling an axis the
trilemma had folded (zero-cost, API-locality, AI-legibility, solver-freedom, bare-metal, unwind-safety,
leak-freedom). The discriminating moments exercise **nine** adjudication modes: a type-level-proof
index pitched as *checker-tractability* was **rejected** (off-diagonal confounded with expressiveness);
take-and-replace pitched as *unwind-safety* was **confirmed** (N ≥ 8, clean); drain-and-yield pitched as
*leak-freedom* was **confirmed** (N ≥ 9, distinct from unwind-safety); a self-referential cell pitched as
*move-stability* was ruled **real but structurally inseparable** (a distinct danger with no isolable
cost, N stays 9); a validity-checked bytes→enum read — the capstone — asked whether the **cost-basis is
closed** and got back, unanimously, **yes** (a genuinely new danger class projects entirely onto the
basis, no tenth axis); two information-flow probes stepped **outside memory safety** to find that the
closure is **per domain** *and floor-relative* — a constant-time secret showed info-flow is a *different
soundness domain*, and a capability-gated declassification showed **no IFC seam closes on Rust's affine
floor** (it bounds resource *uses*, not information *bits*); and a fifteenth — type-level lock ordering, in
a *third* domain (**liveness/progress**) — tested the *positive* half of the floor-domain match and
**broke**, grading the match from binary to **full / partial / none**; and a sixteenth — a *runtime* lock
registry, the same liveness danger on an **upgraded** floor — asked whether upgrading the floor *lifts* the
match, and **broke** too, showing the upgrade is **non-monotone**: it fixed the dynamic-coverage gap but
**granularity recurred one rung finer** (the held-set is keyed on the OS *thread*, but async *tasks* are
the control unit), so the "granularity" axis is a **containment-scope ladder** — token < thread < task —
no ambient Rust floor tops out; and a seventeenth — a *must-run cleanup guard* — probed **breadth-wise** (a
new *precondition*, "this finalizer MUST run", not a new domain or ladder rung) and **broke** on a clean
**none** grade, sharpening the match-unit from *domain* to *precondition* (the *same* liveness domain grades
deadlock-freedom *partial* but must-run-cleanup *none*) and adding the **ninth** mode —
*wrong-structural-logic*: a *safe* `mem::forget` skips the finalizer, and no decidable fix buys it back
(affine "at most once" vs linear "exactly once"), so the floor is the wrong structural *logic*, not the
wrong *domain*. **The cost-basis is closed (per domain), *which seams close* is GRADED by the
floor-domain–precondition match, and that grade is *not monotonically liftable*** — memory safety a **full**
match (basis closed), information flow **no** match (both seams break), liveness a **partial-to-none** match
that an upgraded floor relocates rather than fills (deadlock-freedom partial, must-run-cleanup none) — the
constructive arc's conclusion and its generalizations. And an eighteenth — a *session-typed channel* — is
the **first** out-of-(memory-safety) probe to **hold with a cost** rather than break (SEAM-COSTS [horn 3]):
it shows the floor matches a *composite* precondition **component-wise** — protocol **order** *closes* via
affine typestate (sound, no `root`-hole), while **completion** is a horn-3 *concentration* cost — and the
*same* `mem::forget` linearity gap that **broke** #17's horn 1 only **costs** here, because completion is one
component beside a sound safety core, not the whole promise: *which horn a danger threatens is set by how the
rule packages it.* And a nineteenth — a *value-owning scope-restore guard* — turned that observation into a
*controlled* experiment (carry #17's identical danger in #18's structure) and held with a cost too
(SEAM-COSTS [horn 3]), confirming the dial is **real** but **bounding** it: the move is *confounded*
(value-ownership co-varies with packaging), and the horn is *forceable* (the same danger in a
borrowed-slot/`take_mut` packaging forces horn-1 UB; `Pin` and `thread::scope` exhibit un-repackageable
horns), so the dial moves the horn only **within a structural envelope** set by whether soundness can be
*decoupled* from the un-compellable closure (owned ⇒ costs; borrowed-across-unwind ⇒ forces) — and the dial
itself restages the 2015 leakpocalypse (`JoinGuard` horn-1 / `scopeguard` horn-3, RFC 1066), so the
contribution is the *method* and *locating the fence*, not a new mode. And a twentieth — a *take-and-reseat
guard over a borrowed slot* — ran the **clean single-variable** experiment #19's confound forbade: keep #19's
packaging byte-identical (a guard with `Deref`/`DerefMut`/`into_inner`/Drop-restore) and flip **only**
owned → borrowed. It **broke** (SEAM-BREAKS [horn 1], FAILED — the *sixth* broken seam): the `into_inner` that
was *sound* for #19's owned value is a safe-code **double-free** over a borrowed slot (a 4-line `String`
witness — it returns the owned value while the slot keeps the `ptr::read` duplicate). All five attackers
confirmed the flip is clean (cost-3 → break-1, co-varying *neither* variable), so Phase R's confound is
**repaired**; and the panel ruled the fence **FORCED, not movable** — the `into_inner`-less "sound" subset is
*degenerate* (it deletes the by-value move-out that defines `take_mut`, strictly dominated by `&mut T`), so
every *usable* packaging breaks. The trilogy closes: *the horn is movable iff the value-ownership model lets
soundness **decouple** from the un-compellable closure* — owned can (⇒ cost), borrowed cannot (⇒ forced
break), and the fence sits at the **owned-vs-borrowed** line. And a twenty-first — a *state-based CRDT merge*
— aimed at a **fourth soundness domain** (distributed/replicated **convergence**), and the candidate domain
**deflated**: the supply-side break is real floor-relatively (`T: JoinSemilattice` is a syntactic proxy, so a
safe lawless `join` diverges replicas), but the gap is the *generic* unchecked-typeclass-law phenomenon (Rust
checks the *signatures* of `Ord`/`Eq`/`Hash`/`Monoid`/semilattice, none of their *laws*), so convergence
**projects onto `restore`'s application-correctness basis** rather than opening a fourth domain — the
soundness-domain map stays at **three** (memory / information-flow / liveness) plus application-correctness.
N stays **9** modes; the FAILED broken-seam tally stays **six** (this *survived* as a TRADEOFF). And a
twenty-second — a *cost-typed computation* (`Timed<C,T>`, a graded cost monad) — aimed at the **second**
fourth-domain candidate, **real-time/WCET**, and **FAILED** (SEAM-BREAKS [horn 1], the **seventh** broken
seam): `step` charges one unit decoupled from its closure, so `within::<1>` admits a million-cycle
computation — the op-count is an **unsound** proxy for real time, and the construct is **Danielsson's 2008
cost monad** (op-count ≠ cycles is WCET's *founding premise*). It deflates too — to `typed_nat` (#8, which
does not even scale), an uninstalled `mmio`-shaped axiom (#2), and liveness's partial-match. So **two**
fourth-domain candidates have now deflated (`crdt` survived, `wcet` FAILED), **neither opening a fourth
domain** — the three confirmed domains look **special**. FAILED tally now **seven**. And a twenty-third — a
*real binary session* (`session::<P>()` vending two dual affine endpoints, both live, over
`Rc<RefCell<Transport>>`) — turned back **inside** liveness to *deepen* it, removing #18's single-handle
limitation. It **HELD** (SEAM-COSTS [horn 3], TRADEOFF, r30): #18's component-wise grading **survives** — order
and *per-session* progress close, completion costs horn 3 (now peer-observable). But the "real binary session"
framing was **deflated** to "#18 with a duplicated handle and a `RefCell` between the copies," yielding two
genuine sharpenings of the liveness map: binary duality gives only **per-session** progress (two interleaved
sessions type-check a mutual-wait deadlock — global deadlock-freedom needs multiparty typing), and the
**leakpocalypse double-bind** (a peer's `mem::forget` leaks the `Rc`, so the fallible `recv`'s `Err` is
**non-actionable**) deepens the #17/#18 linearity gap. FAILED tally stays **seven** (this survived). And a
twenty-fourth — **delegation** (channel-over-channel, the higher-order session feature #23 lacked) — went
deeper still, and the method **caught me mislabeling a coordinate using my own citations**. The seam is
**sound** (SEAM-COSTS [horn 3]; the floor even carries *don't-use-after-delegate* for free, by the affine move),
but the gauntlet **FAILED on the §6 deliverable** — the *rejection mode*'s 2nd firing (after `typed_nat` #8). My
"delegation adds no new coordinate, it projects onto leak-freedom" claim was refuted from my cited lineage:
delegation's signature hazard (B forgets `Chan<Q>`, **starving C**) is "*deadlock at the other end*" (Jespersen
et al. 2015) — **progress / deadlock-freedom**, a coordinate *distinct* from completion and leak-freedom, and
delegation is the *recognized* feature that breaks first-order sessions' automatic deadlock-freedom
(Coppo–Dezani–Yoshida–Padovani; Kobayashi; deadlock-free GV). I saw the `Rc` hazard's leak-face and mislabeled
the progress axis; "no new coordinate" was an artifact of a single-threaded witness that cannot deadlock. So the
liveness domain gains **progress as a third coordinate** (the lock seams #15/#16 were its *lock* face;
delegation its *session* face); the seam is sound, so the FAILED tally **stays seven**; N stays **9** (rejection
is one of the nine modes). The probe earned its keep by being **instructively wrong**. And a twenty-fifth —
**multiparty** (an n-party session assembled from #23's binary edges) — built the very feature #23's deflation #1
named (*global deadlock-freedom needs multiparty typing*), and the method caught me **a third time**. A
`RecvThenSend` role enforces recv-before-send; `ring()` — three perfectly-dual binary edges in a source-less
cycle — **type-checks a globally-deadlocked system** (the per-edge floor cannot range over global topology),
while the coherent `relay()` completes. The seam is **sound** (SEAM-COSTS [horn 3]; no UB — a memory-safe
recv-`Err` hang; decidable), but the gauntlet **FAILED the §6 deliverable** — the *rejection mode*'s **3rd
firing** (after `typed_nat` #8 and `delegation` #24), again refuted from my own cited lineage. My deliverable
staked a **false dichotomy**: that global progress lives *only* whole-protocol in a *generative* global-type
projection, no per-role discipline able to reject the ring. **False** — the omitted middle rung, **per-channel
priority/obligation typing** (Kobayashi linear-π; Padovani; **Dardha–Gay FoSSaCS 2018**; Kokke–Dardha), rejects
the source-less ring *compositionally, with no global type*, via `prio(recv) < prio(send)` chaining to a
contradictory `p < p` on the cycle — and I had **mis-filed Kobayashi as *support* when he is the
*counterexample***. The honest taxonomy has **three rungs**: (a) the per-edge `HasDual` floor [doesn't carry];
(b) per-channel priority typing + a whole-program acyclicity check [no global type]; (c) generative MPST
projection. What survives is the *weaker* truth: global progress is **not concentrable per-call** and needs
strictly more than the per-edge floor — but it is a **whole-program** obligation, *not* necessarily a generative
one. The sound role-local order + linearity core CLOSES (Lean `multiparty_role_order_core_closes`); liveness
stays the third domain, **progress** its third coordinate (multiparty = its *global-topology* face); N stays
**9** (rejection's 3rd firing); the FAILED tally **stays seven**; 25 primitives — instructively **wrong** a third
time. And a twenty-sixth — **priority** (per-channel priority/obligation typing) — built the *very discipline*
that #25's rejection named as its omitted middle rung (b), to **confirm #25's weaker thesis humbly**, and the
method **broke the seam instead** — the **first soundness break since `wcet` #22**. Over #23's binary edges and
#8's sealed `typed_nat::Lt`, a forwarder `Relay<Pi, Po>` recv-then-sends under a per-channel priority bound `Pi:
Lt<Po>` (Kobayashi linear-π; **Dardha–Gay FoSSaCS 2018**), and the source-less ring is *supposed* to be untypable
because `p_CA < p_AB < p_BC < p_CA` chains to a contradictory `p: Lt<p>`. The cold panel ruled **SEAM-BREAKS [horn
1], FAILED** (r33), the **eighth** broken seam (tally 7→8) — and I had predicted SEAM-COSTS [horn 3], moving my own
classification to horn 1. The flaw: that chain **secretly assumes edge-priority consistency** — that the `Nat` A
*sends* on AB is the `Nat` B *receives* on AB — but Rust does **no transitive reasoning**, checking each `Pi:
Lt<Po>` in isolation; the contradiction forms only when one `Nat` is shared per edge, which only the coherent
`relay()` constructor threads. The **public** `role::<Pi, Po>` lets three `role::<N1, N2>` relays (every local `1 <
2` bound discharges) wire into the ring: it **compiles, carries no `unsafe`, and deadlocks** — falsifying §4.3,
demonstrated by a passing test. The sting: I had **described this exact flaw in §4** ("edge consistency is the
constructor's invariant, not a per-endpoint type fact") and shipped `role()` public anyway — the soundness attacker
quoted my own words back. It **confirms #25's weaker thesis by breaking**: the per-channel `Lt` bound carries
nothing global (it never names the dual endpoint), so deadlock-freedom rests on a **trusted edge-consistent
constructor**, and the seam salvages to SEAM-COSTS [horn 3] *only if `role()` is hidden*. And it **unifies progress's
two faces**: this is the **same** wrong-granularity break as `ordered_locks` #15/#16 one soundness sub-domain over —
type-level order carries the *local* bound while *whole-program* acyclicity escapes — so progress's **lock** face
(#15/#16) and its **session** face break identically. Lean `priority_typed_discipline_factors` (the break's trivial
sub-fact, like the other broken seams); liveness stays the third domain, **progress** its third coordinate; N stays
**9**; the FAILED broken-seam tally **rises to eight**; 26 primitives — instructively **wrong** a fourth time. And a twenty-seventh — **projected** (the sealed salvage of #26's priority typing) — built the very fix #26's break named (*it salvages to SEAM-COSTS [horn 3] only if `role()` is hidden*), and the method **rejected the deliverable a fourth time** — the salvage holding all the while. The repair **seals `worker()`** (module-private) and vends only trusted builders — `relay` / `relay4` threading strictly-increasing `Nat`s per edge — so #26's deadlocking source-less ring is now **doubly unconstructible**: by **access** (no public `worker()` to hand-wire a cycle) and by **structure** (a monotone priority chain cannot close — the returning edge would need a *decreasing* `Lt`, and no such instance exists). The horn-classifier **conceded Horn 1** (the ring is unconstructible), so the salvage **WORKS** — SEAM-COSTS [horn 3], **SOUND** (r34). But the judge ruled **FAILED on the §6 deliverable** — the *rejection mode*'s **4th firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25; #26 was a *break*, not a rejection). My §4 over-claimed **"deadlock-freedom by construction"** and **"the consumer carries nothing global"**: the builder proves a deadlock-free schedule *exists* (a static `Nat` witness of an acyclic topology), but affine Rust never *forces* the runtime to follow it — a **run-all / in-order / no-drop progress residue** stays on the consumer (the #17/#18/#23 completion-and-execution gap, again). The honest deliverable is **"schedulability by construction, *given all roles execute*"** — concentration is **partial**: the trusted constructor carries **topology** (no cycle — exactly the #26 fix), but **completion / execution never left the consumer**. The giveaway came from my own citations: Scribble / mpstthree projected endpoints are deadlock-free *only assuming every participant executes*, and I imported that guarantee while claiming the consumer carries nothing. (Secondary, accepted: the expressiveness cost was **over**stated — the acyclic-topology generality is recoverable with one generic combinator, as mpstthree shows.) The refined finding: **the progress coordinate decomposes** — *deadlock-freedom / topology* (a trusted constructor *can* carry it) versus *completion / execution* (the consumer carries it irreducibly) — sharpening the #24–26 arc rather than overturning it. The sealed role-order + acyclic-topology core CLOSES (Lean `projected_schedulability_closes`); liveness stays the third domain, **progress** its third coordinate (now split into a *topology* face and an *execution* face); N stays **9** (rejection's 4th firing); the FAILED broken-seam tally **stays eight** (this is a deliverable rejection, not a soundness break — #26's break already counted); 27 primitives — instructively **wrong** a fifth time on the deliverable, but the salvage held. And a twenty-eighth — **choice** (the branching session connective: external choice `offer`/`&` — receive a label and branch — plus internal choice `select`/`⊕` — send a label) — the **last unprobed core session-type feature**, turned back **inside** liveness to finish the connective set, and the method **rejected the deliverable a fifth time**. The seam is **sound** (SEAM-COSTS [horn 3]; no UB — worst case a *starved or leaked* memory-safe session; decidable), but the gauntlet **FAILED on the §6 deliverable** — the *rejection mode*'s **5th firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25, `projected` #27 — all **sound** seams whose deliverable was rejected). THE BREAK: §4 guarantee #2, **"handle every label"**, is **vacuous**. Rust exhaustiveness buys **scrutinee coverage**, *not* run-to-`End` — a *safe* `match offer() { Ok(Branch::Left(c)) => .., _ => {} }` type-checks, carries no `unsafe`, and **drops `Chan<R>`**, starving that branch; "handle-all" **collapses into the conceded #17/#18/#23/#27 completion residue**. Four panel corrections, all accepted: **(1)** branch-safety is **two-legged** (label–continuation correlation + branch-set duality), not four — "handle every label" is the *false* leg, and **order** is the generic typestate guarantee already carried since #18, not branch-specific; **(2)** the deflation is onto **binary `Either` + exhaustiveness**, *not* n-ary **named** sum types (labeled n-ary choice needs nesting that drops the label *names*, or `offer!`/`choose!` macros); **(3)** horn 2 is **Θ(type-size)**, not Θ(depth); **(4)** completion is **amplified per-branch** and **peer-choice-reachable** (adversary-triggered), *not* "unchanged by branching." The **core thesis survives, sharpened**: choice adds **no new coordinate** — the last core session-type feature **deflates onto binary `Either` + exhaustiveness**, opening no new ground; completion stays the #17/#18/#23/#27 residue, now fed by **two** safe paths (`mem::forget` **and** the dropped wildcard branch). No new domain (liveness stays third), no new coordinate (progress, from #24), no new mode (N stays **9**, rejection's 5th firing). The FAILED broken-seam tally **stays eight** (sound — a deliverable rejection, not a soundness break). Prior art **confirms** (Honda 1993's `&`/`⊕`; the `session_types` / Jespersen–Munksgaard–Larsen 2015 affine-vs-linear gap **is** this completion residue). Lean `choice_branch_typing_closes`; 28 primitives — instructively **wrong** a fifth time on the deliverable. The meta-arc closes its session sweep: **the core session-type features are now exhausted — order, channel-passing, multiparty topology, projection, and now choice — and none opened a new coordinate beyond progress (#24)**; the connective set deflates onto the affine floor that #18 first matched. And a twenty-ninth — **must-complete** (a `Session<P>` affine handle carrying a `Drop` **completion bomb** — cf. `take` #9 — so a dropped-mid-protocol session aborts rather than silently starving) — with the connectives exhausted (#28), this seam stopped probing *features* and turned on the **completion residue itself**: the leftover every session seam **#17–#28** has carried and charged to the affine floor. I pre-registered it (humbly) as **SEAM-BREAKS**, the "session face of `linear` #17" — completion an **irreducible linear gap** no affine floor could force — and the method **REVERSED** me, the *rejection mode*'s **6th firing** and the most instructive of the arc because it **inverts** my finding rather than relabelling it. The mechanism break is **real** (the free-handle bomb is bypassed by `mem::forget` — a memory-safe leak, no `unsafe`), but it only shows the **witness** wrong, not the thesis: the decisive move is that **"UNIVERSAL NONE" is REFUTED** — a **scoped-closure / CPS combinator** (`with_session(|s| …)`, the **`std::thread::scope` analogue**) **compels completion modulo divergence, with no `unsafe`**, by moving the obligation to the **re-import boundary** — there is no forgettable *free* handle to leak, because the session never escapes the closure. THE STING: my own §3 cited `thread::scoped`'s **removal** (RFC 1084, the `JoinGuard` leak) and stopped there, ignoring its **fix** — `thread::scope`, stable since 1.63 — which is exactly this discipline. THE REVERSAL, then: completion is **NOT irreducible — it is CONCENTRABLE**, at a horn-3 **API-locality** cost (the protocol must be run *inside* a scope, not handed out as a free value). The **#17–#28 completion residue was the price of the FREE-HANDLE API**, not an inherent gap in the floor; and sessions are **not** #17's open-ended cleanup (a *delimited* protocol **can** be scoped; unbounded resource cleanup cannot — the distinction #20 missed). META, and it **INVERTS the usual error**: across the rejections I have overclaimed concentration is *total*; here I overclaimed it *irreducible*; the truth sits **between** — a real cost, paid in API shape, not a wall. SURVIVES untouched: **decidability**. BOOKKEEPING: the FAILED broken-seam tally **STAYS eight** — this is **SEAM-COSTS [horn 3]**, the concentration I *failed to predict*, not the break I registered; N stays **9** (rejection's 6th firing); liveness stays the **third** domain; 29 primitives. Lean `must_complete_scoped_completion_closes` (a `..._closes`). The arc-level beat is the sharp one: **the completion residue — blamed on the affine floor for twelve seams (#17–#28) — was the cost of the free-handle API all along**, and the scoped salvage that pays it (a `with_session` discipline, #30) is the natural next move — the same projected-salvage rhythm #27 ran for #26's topology. And a thirtieth — **must_scoped** (the scoped salvage of #29, mirroring #26→#27's break→salvage) — built the very `with_session` discipline #29's reversal promised, and the panel **CONFIRMED** it. There is no free `start`; only `session_scope(a, b)` (the self-contained two-endpoint form of #29's `with_session(|s| …)`, the **`std::thread::scope` analogue**) vends two **dual brand-trapped** endpoints, each **required** to return a `Done<'scope>` mintable **only** by `finish()` at `End` — so #29's *forgettable* `Drop`-destructor obligation becomes an **unforgeable return-type** obligation, no free handle to leak (the invariant `'scope` brand lifted **verbatim** from branded-index #5, generativity = `thread::scope`'s trick; no `Drop` bomb). The cold panel ruled **SEAM-COSTS [horn 3], HELD** (TRADEOFF survival like `restore` #19 / `binary_session` #23, r37) — *not* a rejection, *not* a break. THE FINDING, confirmed: **completion IS concentrable**, closing #29's reversal — every attacker **conceded** no safe `Done` is **forged, smuggled, or forgotten** (a *return-type*, not a destructor, defangs `mem::forget`), horns 1 (no UB) & 2 (Θ(depth), no SMT) intact; the **#17–#28 completion residue WAS the price of the FREE-HANDLE API**, bought back by surrendering the free handle. THE SHARPENING (the merits-strongest attacker point — and the builder hedged it correctly, so *the caveat is the finding*): concentration is **per-endpoint, not whole-session** — §2 over-claimed "every session runs to End," but `session_scope` forces only each *returning* closure's own endpoint, so under run-a-then-b `a` finishes while `b` **diverges**, starving the peer — **global completion escapes via peer divergence** (the #23/#27 per-session-vs-global gap, one step on); secondary, the **synchronous trap** is understated (sequential run-a-then-b cannot express concurrent/interleaved sessions). META, consistent with #29: I over-claimed the *extreme* again — #29 "irreducible," #30 "whole-session global" — the truth is the **per-endpoint** cost between. DEFLATION (conceded): no new mechanism — `thread::scope` + generativity (branded #5) + #29's `finish`/`End` typestate. BOOKKEEPING: the FAILED broken-seam tally **STAYS eight** — this **HELD** (SEAM-COSTS [horn 3]), it did **not** increment; the rejection-mode count **stays six** (this is neither a break nor a rejection); N stays **9**; liveness stays the **third** domain; **30 primitives**. Lean `must_scoped_completion_closes` (a `..._closes`). The arc-level beat: #29 proved the completion residue was the free-handle API's price; #30 **pays it** — the scoped salvage works, and the cost it leaves is not a wall but the **per-endpoint** seam between completion-per-closure and completion-global. And a thirty-first — **recursion** (`Rec<P>` / `Var`, the `µX.T` looping connective) — the **last unbuilt structural session feature**, built to probe the **boundary of #30** at unbounded depth, and the method **rejected the deliverable a seventh time**. MECHANISM: single-level `Rec<P>`/`Var` over an env-threaded `Session<P, E>` (default `NoRec`); `enter()` unfolds, `recur()` fires **only** for `Session<Var, Rec<P>>` so a top-level `Var` is **inert** (sound, the **branded-#5 analogue**); `HasDual` threads recursion; without choice, single-level recursion is necessarily **non-terminating** (no reachable `End`). The cold panel ruled **SEAM-COSTS [horn 3]** — **SOUND** (a *deliverable rejection*, **not** a break, r38): Horn 1 holds (the `Var`/environment encoding is UB-free), Horn 2 holds (decidable, Θ(type-size)), and the **#30-bounding finding** lands — no reachable `End` makes #30's `Done`-forcing obligation **vacuous**, so recursion exposes #30's *unstated precondition* (a reachable terminal). THE REJECTION, and **I over-conceded**: I filed **productivity / guardedness** as an undecidable NONE *beside* termination — but it is a **DECIDABLE cost**. `Rec<Var>` is the non-contractive `µX.X` every cited system (Coq / Agda **guarded corecursion**, contractive recursive types) rejects **syntactically**, and a sealed **`Guarded` marker** (impl for Tx/Rx/End/Rec, **not** `Var`) carries it in Θ(type-size) with no SMT — the **same machinery as order**. So guardedness **CONCENTRATES (horn 3)**; **only TERMINATION is the genuine undecidable NONE** (halting) — refuted from my **own cited guarded-corecursion lineage** again (#24/#25/#29). META, the **inverse** of the usual over-claim, the same shape: prior rejections over-claimed concentration *total*, #29/#30 over-claimed the danger *irreducible / global*; here I **over-conceded** — put a decidable cost at the **undecidable extreme** — so the unifying tic is that I **flee the MIDDLE** toward whichever extreme is cleaner. CORRECTED: order + duality + **guardedness** CLOSE; **termination alone** is the NONE. BOOKKEEPING: the FAILED broken-seam tally **STAYS eight** (sound — a deliverable rejection, it did **not** increment); the rejection-mode count is now **seven**; N stays **9**; liveness stays the **third** domain; **31 primitives**. Lean `recursion_guarded_order_closes` (a `..._closes`, renamed from `recursion_order_unfolding_closes`). The natural **#32 salvage** is the sealed `Guarded` marker so `Rec<Var>` won't form — the same rejection→salvage rhythm #27 ran for #26 and #30 ran for #29. And a thirty-second — **guarded** (the sealed-marker salvage of #31's recursion, mirroring #26→#27 and #29→#30) — built the very fix #31's rejection prescribed (*the sealed `Guarded` marker so `Rec<Var>` won't form*), and the panel **CONFIRMED** it — the **cleanest result in the arc**. MECHANISM: over #31's `Rec<P>`/`Var`, a sealed **`Guarded` marker** (a `sealed::Sealed` supertrait; impl for `Tx<C>`/`Rx<C>`/`End`, **not** `Var`) bounds `enter()`/`recur()`, so the unguarded `Rec<Var>` = µX.X **cannot unfold** (Θ(type-size), no SMT). KEY: for single-level recursion a **shallow head-check IS full contractivity** — the action head guards everything inside, `End` is vacuous, a bare `Var` head is unguarded — the **same machinery as order**. The cold panel ruled **SEAM-COSTS [none]** — **SURVIVED** (1/1, r39): the salvage **HELD**, and `[none]` is the **cleanest result in the arc** — all three horns hold, no horn dropped, **stronger than #30's [horn 3]** (no safe unfold of `Rec<Var>`, no UB, zero-ceremony concentration, decidable no-SMT, no false positive) — *not* a rejection, *not* a break. It **CLOSES #31's over-concession**: guardedness **IS concentrable** (horn 3), not the undecidable NONE I filed it as. And I **avoided the #27 salvage-over-claim trap** — claiming **only productivity**, conceding **termination** up front. WHAT THE PANEL MARKED (tradeoffs, **not** horn-drops): **completeness bought by restriction** — the head-check equals full contractivity *only* because the single-level choice-free grammar collapses it to "head ≠ `Var`"; richer grammar (nested `Rec`, choice-in-recursion) needs the real recursive walk, **scoped out, not escaped**; and my "folklore" deflation slightly overstated generality. It **buys productivity, NOT termination** (`Rec<Tx<Var>>` is guarded and still infinite — **halting stays the one conceded undecidable NONE**). The final split: **order + duality + guardedness/productivity all CLOSE; termination alone is the NONE.** BOOKKEEPING: the FAILED broken-seam tally **STAYS eight** (this **SURVIVED** — neither a break nor a rejection, it did **not** increment); the rejection-mode count **stays seven** (a survival, not a rejection); N stays **9**; liveness stays the **third** domain; **32 primitives**. Lean `guarded_recursion_productivity_closes` (a `..._closes`). The arc-level beat: #31 over-conceded guardedness to the undecidable extreme; #32 **closes it** — guardedness concentrates, and the lone NONE that remains is termination alone. And a thirty-third — **divisor** (total-ish integer division) — is the **first pivot OFF the session family** (connectives #18–#28, completion #29/#30, recursion #31/#32 now fully mapped), turning to a fresh danger class: **panic-freedom / totality** (SPARK's "absence of run-time errors"). The cold panel ruled **SEAM-COSTS [horn 3]** — **SOUND**, but the judge **FAILED the §6 deliverable** (r40), the *rejection mode*'s **8th firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25, `projected` #27, `choice` #28, `must_complete` #29, `recursion` #31). THE HEADLINE SYNTHESIS RESULT: **panic-freedom FOLDS into the application-correctness basis** (#19/#21), **NOT a fourth soundness domain** — a panic is *defined behaviour* (a clean abort, never UB), so a division seam removes a *correctness* defect, not an unsoundness. With this, **all THREE fourth-domain probes have now deflated** (`crdt` #21 folds into application-correctness, `wcet` #22 breaks unsound, `divisor` #33 folds into application-correctness) — **none** opening a fourth domain, so the confirmed **three**-domain map (memory safety / information flow / liveness) looks **genuinely special / possibly complete**. Prior art confirms the fold as the historical default (SPARK's `division_check`/`overflow_check` split; Liquid-Haskell's nonzero-div refinement; `core::NonZero`); expressiveness granted it. THE REJECTION, and it is the **SAME tic as #31, EXACTLY INVERTED onto a new domain**: #31 over-conceded **guardedness** as an undecidable NONE when it was a decidable marker; #33 over-conceded the `i32::MIN / -1` **overflow** as undecidable range-analysis when it is a **decidable point-exclusion**. Both instantiate the project's unifying failure — **I flee the MIDDLE (a decidable partial cost) toward whichever EXTREME is cleaner** — here in the **over-concession** direction (toward undecidable), the *same* direction as #31, and both refuted **from my own cited framing** (#33 from its own §2, which lumped division overflow with arbitrary `+`/`*`), strengthening the synthesis claim that **calibration is the recurring axis**. THE TECHNICAL CORE: `divide` overflows on **exactly** `a == i32::MIN && d == -1`; the divisor-side conjunct is on the *refined* side, so `d ∉ {0, -1}` makes division **total for every `a`** via the same Θ(1) mechanism — "two sources, one closes, one escapes" is **WRONG for division: BOTH close**, and returning `Result` forces `.unwrap()` on provably-total divisions, **reintroducing the very panic the seam exists to remove**. MECHANISM: `Divisor(NonZeroI32)` closes on the **REAL floor** (`NonZeroI32` is the one refinement type stock Rust ships — no SMT), unlike refined #6's *hypothetical* Flux/LH floor; it overlaps refined #6 in **mechanism**, but the **DOMAIN** finding is the contribution. The natural **#34 salvage** (**SETTLED** by #34, mirroring #26→#27, #29→#30, #31→#32): a `TotalDivisor` (`d ∉ {0, -1}`) vending total `-> i32` division, proving the overflow residue **concentrates too** — leaving only arbitrary `+`/`*` range analysis as the genuine undecidable NONE. BOOKKEEPING: the FAILED broken-seam tally **STAYS eight** (sound — a deliverable rejection, it did **not** increment); the rejection-mode count is now **eight** (firings #8/#24/#25/#27/#28/#29/#31/#33); N stays **9**; the confirmed soundness-domain map stays **three**; **33 primitives**. Lean `divisor_nonzero_division_closes` (a `..._closes`, like #31's rejection). The arc-level beat: the session family exhausted, the first fresh danger class **deflates onto application-correctness** — the three soundness domains hold, and calibration (over-claim vs over-concession) is confirmed the recurring axis across the rejections. And a thirty-fourth — **total_div** (total integer division, Phase AG) — is the **sealed-total salvage of #33's `divisor`**, the **FOURTH** reject→salvage pair (after #26→#27, #29→#30, #31→#32), built to prove #33's overflow residue **concentrates too**, and the panel **CONFIRMED** it — the **cleanest verdict in the whole project**. MECHANISM: a `TotalDivisor(i32)` carrying the **finite point-exclusion** `d ∉ {0, -1}` vends **total** `-> i32` division with **no panic path** — `divide` overflows on **exactly** `a == i32::MIN && d == -1`, and excluding `-1` costs nothing safe (`a / -1 == -a`, whose sole overflow `i32::MIN` **coincides with negation's own**), so both the zero-divisor and the `MIN/-1` conjuncts close via one Θ(1) check. The cold panel ruled **SEAM-HOLDS [none]** — the judge graded **TRADEOFF, SURVIVED 1/1 (r41)**: soundness, implementation, and expressiveness **all ARGUED-PAST**, no horn even **costed** — **stronger than #32's SEAM-COSTS [none]** (itself the previous cleanest), the **cleanest result in the whole project**. THE SYNTHESIS RESULT: it **CLOSES #33's over-concession** — division's overflow was a **DECIDABLE point-exclusion** (`d ∉ {0, -1}` makes `a/d` total for every `a`), **not** the undecidable range-analysis #33 mis-filed; "two sources — one closes, one escapes" was **WRONG: BOTH close**, and `TotalDivisor` vends total `-> i32` division with no panic path. IMPORTANT for the synthesis: this does **NOT** move the #33 **DOMAIN** finding — panic-freedom still **FOLDS into application-correctness**, and **all three** fourth-domain probes (`crdt` #21, `wcet` #22, `divisor` #33) **stay deflated**, the confirmed **three**-domain map intact; #34 corrects the **RESIDUE LINE within division**, not the domain map. THE META THREAD (central to the synthesis): #34 is the **fourth salvage**, and like #32 it **HELD by DODGING the #27 over-claim trap** — pre-conceding the **three non-claims** (no fourth domain; no `+`/`*` generalization; value-threaded). BUT the panel still marked the **recurring calibration shape in a MILDER form**: I made the win look bigger by benchmarking against a **raw-`/` STRAWMAN** and **OMITTING the floor's own total `checked_div`/`checked_rem`** from prior-art, so the seam's genuine advantage is **NARROW** — only the **amortized reusable RUNTIME divisor**. Same "flee toward the cleaner framing" tic, now landing as a **marked tradeoff BELOW the horn-deciding threshold** (as #32) — strengthening the synthesis claim that **calibration is the recurring axis**, AND that it is **improving** (from horn-deciding rejections #8–#31 down to marked tradeoffs #32/#34). The two-salvage calibration arc: #33 rejected (over-concession) → #34's salvage **HELD cleanly** — the reject→salvage rhythm **converged in one step**, as #31→#32 did. BOOKKEEPING: the FAILED broken-seam tally **STAYS eight** (this **SURVIVED** — neither a break nor a rejection, it did **not** increment); the rejection-mode count **stays eight** (a survival, not a rejection — firings still #8/#24/#25/#27/#28/#29/#31/#33); N stays **9**; the three fourth-domain probes stay **three** (all deflated); the confirmed soundness-domain map stays **three**; **34 primitives**. Lean `total_div_both_sources_closes` (a `..._closes` — the total-division core closes). The arc-level beat: #33 over-conceded division's overflow to the undecidable extreme; #34 **closes it** — the overflow residue concentrates via a finite point-exclusion, leaving only arbitrary `+`/`*` range analysis as the genuine undecidable NONE, and the reject→salvage rhythm carries cleanly out of the session family into the totality class. And a thirty-fifth — **sync_cell** (shared mutability across threads: a `Cell`-like container re-exported as safe to *share between threads*), Phase AH — is the **FOURTH fourth-domain probe**, aimed at **data-race freedom / concurrency**, and the panel **BROKE the seam** — the **ninth** broken seam (tally 8→9), the **FIRST concurrency break** and the **first soundness increment since the survivals** (#32/#34), but a **CONCEDED FOLKLORE break**: the seam was a **strawman probe vehicle**, safe `thread::scope` racing a non-atomic write into UB with **zero consumer `unsafe`**, exactly as designed. The cold panel ruled **SEAM-BREAKS [horn 1]** (r42) and the **judge FAILED the contribution**. THE SYNTHESIS RESULT (the substance — the classification, not the break): my staked claim — **"no value-adding concurrency seam exists; data-race freedom is FLOOR-CLOSED; a re-import is CATEGORICALLY an unchecked axiom"** — is **REFUTED** by **`SpinMutex<T>`**, a sound `core`-only concurrency seam whose **`unsafe impl<T: Send> Sync`** is **DISCHARGED by the lock invariant** (mutual exclusion serializes access, so no data race can form). CORRECTION: distinguish a **DISCHARGED** `unsafe impl` from a **FALSE** one, and **reclassify data-race freedom (DRF)** from "floor-closed-special" to **"floor-PROPAGATES-but-does-not-close,"** exactly **ANALOGOUS to borrowck / memory-aliasing** — closure comes not from the floor alone but from **concentrating seams** (locks / channels / atomics), and **`SpinMutex` is the concurrency slab**. THE DOMAIN-MAP UPDATE (the synthesis payload): my prior implicit **TRICHOTOMY** — IFC (floor closes nothing → seams break), memory-aliasing (floor concentrates), concurrency (floor-closed-special) — **COLLAPSES to a DICHOTOMY**: **IFC is the special one** (floor closes nothing), while **memory-aliasing AND concurrency are BOTH "floor-propagates + concentrating seams close."** So the domain map is now **memory safety** (with **aliasing AND data-race** as *two* concentrate-behind-a-seam coordinates) / **IFC** (floor-closes-nothing) / **liveness** / + the **application-correctness basis** (#19) — still **THREE** soundness domains, because **DRF is NOT a fourth** (a data race **IS UB**, inside memory safety), so the map **SURVIVES**, but its internal structure is now clearer. THE META THREAD (central to the synthesis): #35's failure is the **SAME clean-framing tic as #33** — I made the framing **cleaner than reality by omitting the obvious counterexample** (#33 omitted `checked_div`; #35 omitted `SpinMutex`). The CRUCIAL nuance for the calibration arc: #32/#34 **trimmed the tic BELOW the horn-deciding threshold** (marked tradeoffs), but #35 was a **HORN-DECIDING FAILURE** — **moving to a FRESH domain, over-confidence about the classification reasserted the tic at FULL strength**. So the calibration arc is **NOT monotonic improvement**: the tic **recedes on familiar ground but re-triggers full-strength on novel ground** — a sharper, more honest synthesis claim than "calibration steadily improves." SALVAGE #36 (OWED, mirroring #26→#27's break→salvage, **not yet built**): the **`SpinMutex<T>`** (discharged `unsafe impl`) proving DRF is a **concentrate-behind-a-seam** domain, closing the over-claim. BOOKKEEPING: the FAILED broken-seam tally **RISES to nine** (8→9 — this **BREAKS**, the **first increment since the survivals**, but a **conceded folklore break**, a strawman probe vehicle); the rejection-mode count **STAYS eight** (this is a **break, not a sound-seam rejection** — firings still #8/#24/#25/#27/#28/#29/#31/#33); N stays **9**; the four reject/break→salvage pairs settled (#26→#27, #29→#30, #31→#32, #33→#34) gain a **fifth OWED** (#35→#36, a break→salvage like #26→#27); the fourth-domain probes are now **four** (crdt #21 fold, wcet #22 break, divisor #33 fold, sync_cell #35 break+reclassify), **none** opening a fourth domain; the confirmed soundness-domain map stays **three** (memory / IFC / liveness) + application-correctness (#19); **35 primitives**. Lean `sync_cell_datarace_factors` (a `..._factors` — it breaks, like `priority`/`wcet`). The arc-level beat: the first concurrency probe **breaks by design**, but its *classification* over-reaches — DRF is not floor-closed but **floor-propagates-and-concentrates** like aliasing, collapsing the domain trichotomy to a dichotomy — and the calibration tic proves **non-monotonic**, receding on familiar ground and re-firing at full strength on the novel concurrency domain, with the `SpinMutex` #36 salvage owed. And a thirty-sixth — **spin_mutex** (`SpinMutex<T>`, a `core`-only spinlock: an `AtomicBool` flag + `UnsafeCell<T>`, `lock()` spinning on `compare_exchange` to vend a `Deref`/`DerefMut` `SpinGuard`, `unsafe impl<T: Send> Sync`), Phase AI — is the **salvage of #35** owed (mirroring #26→#27's break→salvage): the sound, value-adding, `core`-only concurrency seam #35's break claimed could not exist, built to **discharge** the `unsafe impl` and prove DRF concentrates behind a seam — and the method **broke the salvage itself**, the **first time a salvage has broken**. The cold panel ruled **SEAM-BREAKS [horn 1]**, **FAILED** (r43), the **tenth** broken seam (tally **9→10**) and a **DISCOVERED** break (the **9th discovered**; #35 stays the lone **conceded folklore** one). THE BREAK (real, verified compiles under `#![forbid(unsafe_code)]`): a concurrency `unsafe impl` has **two conjuncts** — cross-thread exclusion (the lock, which I **discharged**) AND the guard's no-`&T`-leak bound (`T: Sync`, which I **OMITTED**). With no manual guard impl, `SpinGuard` auto-derives `Sync` for every `T: Send` via my own too-loose `unsafe impl<T: Send> Sync for SpinMutex`, so picking `T = Cell<i32>` (`Send + !Sync`, slipping past the `Rc: !Send` filter) lets a safe `thread::scope` share `&guard` and race `set`/`get` into **data-race UB with zero consumer `unsafe`**, lock never contended — where `std` writes the stricter `unsafe impl<T: Sync> Sync for MutexGuard` (+ `MutexGuard: !Send`). WHAT SURVIVES THE BREAK (the judge is explicit): the **#35-refuting existential stands on its own** — a *correctly-bounded* `SpinMutex<T: Sync>` IS sound (`spin::Mutex`; **RustBelt POPL 2018** proves it) — so the **concentrate-behind-a-seam CLASSIFICATION** (data-race freedom closes by concentrating seams like locks, the same bucket as memory-aliasing; a **discharged** `unsafe impl` is categorically distinct from a **false** one) is **NOT refuted by the break**: my *artifact* had a hole, the *thesis* holds, and DRF is still **not** a fourth soundness domain (a data race **IS UB**). THE META, the **sharpest instance of the arc's tic**: the "**omit a conjunct / flee toward the cleaner framing**" tic (the same shape as #35 deleting the exclusion obligation, #33 omitting `checked_div`) recurred **INSIDE the very salvage meant to prove I'd corrected it** — I omitted the guard's `Sync` bound. So **calibration is NOT converging on the concurrency domain**: the concurrency `unsafe impl`'s full obligation set (exclusion + guard-`Sync`) only got mapped **under adversarial pressure**, sharpening #35's non-monotonic-calibration finding into *concurrency is a domain where the tic fires **repeatedly** until the gauntlet exhausts the obligation set*. BOOKKEEPING: the FAILED broken-seam tally **RISES to ten** (9→10 — the **9th discovered** break + #35's one conceded folklore); the rejection-mode count **STAYS eight** (this is a **break**, not a sound-seam rejection — firings still #8/#24/#25/#27/#28/#29/#31/#33); N stays **9**; the four reject/break→salvage pairs settled (#26→#27, #29→#30, #31→#32, #33→#34) do **not** gain a fifth — **#35→#36 does NOT settle** (the salvage broke; the chain **extends to #37**, the correctly-bounded `SpinGuard<T: Sync>` + `!Send`); the fourth-domain probes stay **four** (#36 is a salvage, not a probe); the confirmed soundness-domain map stays **three** (memory / IFC / liveness) + application-correctness (#19); **36 primitives**. Lean `spin_mutex_guard_sync_factors` (a `..._factors` — it breaks, like `sync_cell` / `priority` / `wcet`). The arc-level beat: the salvage owed for #35's over-claim **broke the same way #35 did** — an omitted conjunct in the concurrency `unsafe impl` — proving the calibration tic does **not converge** on the novel concurrency domain but must be exhausted obligation-by-obligation under the gauntlet; yet the classification it was built to defend **survives the break** (a correctly-bounded `SpinMutex` is sound), so DRF stays a concentrate-behind-a-seam coordinate of memory safety, with the correctly-bounded **#37** now owed. And a thirty-seventh — **spin_lock** (`SpinLock<T>`, a `core`-only spinlock with the guard's `Sync` bound corrected: `AtomicBool` + `UnsafeCell<T>`, `lock()` spinning on `compare_exchange` (Acquire) to vend a `Deref`/`DerefMut`, Release-on-Drop `SpinLockGuard`), Phase AJ — is the **salvage of #36** owed (mirroring #26→#27, #29→#30, #31→#32, #33→#34's break→salvage), the correctly-bounded spinlock #36's break named as owed, built to complete the concurrency `unsafe impl`'s full obligation set — and the method **CONFIRMED it**, the salvage **HELD**, the **first salvage of a broken salvage to hold**. The cold panel ruled **SEAM-HOLDS [none]**, **SURVIVED** (1/1, r44) — the **cleanest verdict tier** (a full HOLD, no horn even costed — like `total_div` #34; I predicted [horn 3] and UNDER-claimed). THE FIX: #36 discharged **three** of a mutex's four auto-trait obligations and dropped the fourth; #37 enumerates and discharges **all four**, the added line `unsafe impl<T: Sync> Sync for SpinLockGuard` (obligation 3 — a shared `&guard` hands `&T` to many threads, sound only when `T: Sync`, opting the guard out of the too-weak auto-`Sync = T: Send`). VERIFIED: the exact r43 laundering path that COMPILED against `SpinMutex` #36 is now REJECTED against `SpinLock` #37 with `error[E0277]: Cell<i32> cannot be shared between threads safely` — **the r43 break is closed**. The four obligations: (1) `SpinLock<T>: Send where T: Send`; (2) `SpinLock<T>: Sync where T: Send` (exclusive access ⇒ T migrates, like `std::Mutex`); (3) `SpinLockGuard<T>: Sync where T: Sync` (THE FIX); (4) `SpinLockGuard<T>: Send where T: Send` (auto; sound for a spinlock — the Release store is thread-agnostic, unlike an OS futex, so a migrated guard is sound; `spin::MutexGuard`'s reasoned choice, vs `std::MutexGuard`'s `!Send`). HOW IT SURVIVED: the pre-registered sharpest attack — hunt a **FIFTH** omitted conjunct (the #36 failure mode) — found **NONE**; the soundness attacker ran **SIX** break attempts (`T: Sync + !Send`; the r43 `Send + !Sync` shape; a Send-migrated guard; variance via the invariant `UnsafeCell`; ordering; poison/forget), all failed; three independent reconstructions (soundness, expressiveness, prior-art) converged; horns 1 (no UB), 2 (decidable), and 3 all intact. THE STAKED CONTRIBUTION CONFIRMED: a sound, value-adding, `core`-only concurrency seam EXISTS (the thing #35 said could not) — so **data-race freedom is a concentrate-behind-a-seam domain** (locks its seams, the concurrency analog of SPSC #1 / the slab #4; the **memory-aliasing bucket**, specifically its **DYNAMIC end**, like `RefCell` — vs SPSC/slab which discharge *statically*); a **discharged** `unsafe impl` (obligation met by lock invariant + `T: Sync`) is categorically distinct from a **false** one (#35 SyncCell); DRF is still **NOT** a fourth soundness domain (a data race **IS UB**), and the **#35→#36→#37 chain SETTLES** it — the **fifth** settled reject/break→salvage lineage and the **only TWO-STEP** one (two steps because #36 broke). TWO RECORDED TRADEOFFS (SURVIVED-*with*-tradeoffs — soundness NOT contingent on them, so **not** TRADEOFF-mode): **(1)** overstated std/`spin::Mutex` parity — I wrote `SpinLock<T>`, not `SpinLock<T: ?Sized>`, so `SpinLock<[u8]>` / `SpinLock<dyn Trait>` (expressible in **both** cited precedents) cannot be written, no `into_inner`/`get_mut`, and the flagship `core`-only IRQ-shared use case is deferred to an unbuilt `irqsave` variant (the calibration tic in its MILDEST form — a marked tradeoff below the horn-deciding threshold, as #34); **(2)** novelty is ~NIL — obligation 3 is VERBATIM **Ralf Jung's named, dated June 2017 "How `MutexGuard` was `Sync` When It Should Not Have Been,"** a famous std bug and its shipped patch (even the *classification* is Jung's 2017 conclusion; RustBelt POPL 2018 proved the semantic soundness) — a famous fix copied, not generic folklore. META, the calibration-arc CODA (the synthesis-level payoff): **concurrency took THREE seams to fully map** — #35 (break — over-claimed floor-closed), #36 (break — omitted the guard conjunct), #37 (HOLD — complete four-obligation enumeration) — and the **omit-a-conjunct tic fired at FULL strength TWICE (#35, #36) then RECEDED to marked-tradeoff on #37**, giving #35's non-monotonic-calibration finding its coda: on a **fresh domain** the tic fires **repeatedly until the obligation set is exhausted by the gauntlet, THEN recedes** — convergence WAS achieved, just over **three** seams instead of one, and the concurrency salvage lineage is now closed. BOOKKEEPING: the FAILED broken-seam tally **STAYS ten** (this **HELD** — neither a break nor a rejection, it did **not** increment); the rejection-mode count **stays eight** (a survival, not a rejection); N stays **9**; the fourth-domain probes stay **four** (a salvage, not a probe); the settled reject/break→salvage lineages are now **five** (the four one-step pairs #26→#27, #29→#30, #31→#32, #33→#34 PLUS the #35→#36→#37 two-step chain); the confirmed soundness-domain map stays **three** (memory / IFC / liveness) + application-correctness (#19); **37 primitives**. Lean `spin_lock_guard_sync_closes` (a `..._closes` — it holds, like the other survivals). The arc-level beat: the salvage owed for #36's broken salvage **HELD cleanly** — the correctly-bounded four-obligation spinlock — so the #35→#36→#37 chain settles concurrency as a concentrate-behind-a-seam coordinate of memory safety, and the calibration tic that fired full-strength twice on the fresh domain finally recedes to a marked tradeoff, closing the non-monotonic arc with convergence over three seams.

## The headline result

**Whether a seam closes is not a property of "seams" — it is a function of the
precondition's shape.** The trilemma bites *selectively*, and exactly where it bites is
predictable from one question: *can every conjunct of the primitive's true precondition
that the caller can observe be carried by the floor's witness vocabulary (affinity, NLL
lifetimes, decidable trait bounds) — and if not, is the missing conjunct an external axiom,
a runtime-checkable fact, or genuinely unrepairable?*

| Primitive | Cold verdict | Horn outcome | Seam width (measured) | Why — precondition shape |
|---|---|---|---|---|
| **SPSC ring buffer** | SEAM-HOLDS (4 cold panels: A + B×3) | none drops | **O(1)/buffer**, 0/element | *Fully encapsulable.* Every escaping conjunct is floor-expressible: cardinality→affinity, storage-lifetime→NLL `'a`, payload safety→decidable `T: Send`. The hard facts (happens-before) leave no caller-visible residue. |
| **MMIO read-to-clear** | SEAM-COSTS [horn 3] | sound + decidable, marked cost | O(1) **iff** foundation adds a `Send` opt-in, else O(ISRs) | *Sound modulo an external axiom.* M1/M4 ("this address really is this register") is a physical-world fact no checker can discharge — a TCB `assume` boundary (Low\*/seL4/CompCert), **not a fourth horn**. The real price is a horn-3 concentration cost around the ISR `Send` opt-in. |
| **Generational slab** | SEAM-COSTS [#6 zero-cost] (after one revision) | **all three trilemma horns hold** | **O(1)**, fully-safe interface | *`Copy`-handle validity — runtime-checkable.* A `Copy` key can outlive its slot, so affinity/NLL can't carry S1–S3; but a per-slot generation + occupancy tag *can*, at run time. Closes the trilemma but spends the **zero-cost property (#6)** the trilemma folded away — `RefCell`'s move. |
| **Intrusive doubly-linked list** | SEAM-BREAKS [horn 1] | horn 1 drops (repair drops usability *or* decidability) | O(accesses) for the branded form; design A forbids early reclaim | *Whole-heap aliasing + liveness that escapes to the caller.* The `GhostCell` brand carries L1/L4 (aliasing) but **not L2** (membership-vs-storage lifetime). Token correctly presented, the rule type-accepts a use-after-free. |
| **Branded index** (sound unchecked indexing) | SEAM-COSTS [other] (1 cold panel) | all 3 horns hold | O(1)/slice — yet a *viral* `'id` in every type | *Cross-object identity, carried by a generative brand.* Closes all three horns (RustBelt-verified lineage), but un-bundles **concentration** itself: pays on **API-locality (#5)** (the `'id` infects consumer types, traps indices in a `scope` closure) and *defers* zero-cost (#6) to range-branding. The brand is a nominal token (#7 illegible). |
| **Refined index** (refinement-typed bounds) | SEAM-COSTS [horn2] (1 cold panel) | all 3 horns hold | O(1)/index — brand-free, the value *escapes* | *In-bounds, carried by a legible predicate (Flux/Liquid Haskell).* The **(brand-free, legible)** corner — it **closes the #5⟂#7 cell** (vs the slab's brand-free-but-illegible handle). Un-bundles **horn 2** into *decidability* ✓ vs *solver-freedom* ✗ (needs SMT in the TCB — the `unified` "keep" horn). The R2 trade: brand-free ⟹ a runtime check, zero-cost ⟹ a length-brand. |
| **Shared (Rc)** (refcounted ownership) | SEAM-COSTS [bare-metal] (1 cold panel) | all 3 horns hold | O(1) — brand-free; needs a heap | *Dynamic shared ownership, carried by a runtime refcount manager.* Recovers seed property **#5 bare-metal**: the sole seam needing a global allocator (compile-enforced — the only `extern crate alloc`). Independent of #6 (slab/`heapless` off-diagonal: fixed pool vs global heap), but the judge deflated novelty — it **= Rust's `core`/`alloc` boundary (RFC 2480)**, real but pre-existing. |
| **Type-level index** (proof by trait resolution) | **FAILED** [axis deliverable] — sound (1 cold panel) | sound; all 3 horns hold; pays a recorded horn-2 cost | O(1) — brand-free, erases | *Compile-time proof `I: Lt<Len>`, carried by the trait resolver.* Sound (sealed `Nat` + orphan rule block forgery, unanimous), zero-cost, solver-free. **But the eighth-axis claim was rejected:** the off-diagonal is confounded with a strict expressiveness loss (static-index-only), the `recursion_limit` cost is encoding-relative (binary numerals erase it), and the axis is mislabeled — `Lt` is *linear*, the real concern is **terminating-resolution** (Paterson / `UndecidableInstances`), pre-existing. No new axis; N stays 7. |
| **Take-and-replace** (move out and back) | SEAM-COSTS [none] — TRADEOFF (1 cold panel) | sound; all 3 horns hold; pays the 8th axis | O(1) — brand-free, zero-cost on the happy path | *Transient-invariant-breaking across an unwind, carried by an abort-bomb (`take_mut`'s pattern).* Moving a value out of a `&mut T` leaves a duplicate in the gap; an unwinding panic drags the caller into a double-drop. Aces all seven prior axes and pays on **unwind-safety (the 8th axis)** alone — the *clean* off-diagonal the type-level index lacked. Confirmed N ≥ 8, deflated to pre-existing (Abrahams; `noexcept`; `UnwindSafe`; RFC #1508). The axis = *exception-safety-guarantee-level*; abort is one **discharge** (vs `replace_with` paying API-surface). |
| **Drain-and-yield** (leaked guard) | SEAM-HOLDS / SURVIVED (1 cold panel) | sound; all 3 horns hold; pays the 9th axis | O(1) — brand-free, no closure | *The leakpocalypse: `mem::forget` is safe, so a guard whose `Drop` does load-bearing cleanup can be leaked → double-free.* Sound via **pre-pessimization** (`Vec::drain`'s trick — empty the buffer up front, so a leaked guard loses the tail, never double-frees). Aces all eight prior axes, pays on **leak-freedom (the 9th axis)** alone. Confirmed N ≥ 9, **distinct from unwind-safety** (off-diagonal both ways: take is unwind-sensitive/leak-safe, drain is leak-sensitive/unwind-safe). Deflated to pre-existing (leakpocalypse / RFC 1066). The *thinnest* axis but genuinely distinct; aces API-locality *because* the guard owns its data (no closure). |
| **Self-ref cell** (`Pin`-guarded) | SEAM-HOLDS / TRADEOFF (1 cold panel) | sound; all 3 horns hold; pays an **inseparable** danger | O(1) — but a viral, `&mut`-revoking `Pin` | *A value whose **address** is load-bearing (a pointer into itself); moving it dangles the self-pointer.* Sound via `Pin` over a `!Unpin` cell (the post-`init` move is unconstructible). A danger no other axis touches — **but ruled REAL-BUT-INSEPARABLE (outcome 3, N stays 9):** no isolable cost. The price is *immovability* (heavier than API-locality), **non-uniform** across witnesses (stack-pin → API-locality, `Box::pin` → bare-metal). The lattice's first *inseparable* entry — a new verdict category. Caveats: inseparability is contingent (RFC 2349 rejected `?Move` immovable types); crisp only for a non-composable toy. |
| **Validity** (bytes→enum) | SEAM-HOLDS / TRADEOFF (3 cold panels: 2 found spec bugs) | sound; all 3 horns hold; **CLOSED-BASIS test** | O(1) per read; a viral sealed ctor for the view | *A **validity invariant** — a `#[repr(u8)]` enum byte that must be a valid discriminant, UB even without a deref.* A genuinely new danger class, built to ask: does it add an axis? Sound via `transmute`-after-**set-membership** (the move from a *range* check — unsound for non-contiguous discriminants — was forced by gauntlet 1). **Unanimous ruling: NO tenth axis — it projects onto the basis.** Its own cost is **(3) inseparable** (2nd example): per-read → #6, the zero-copy `&[u8]→&[Tag]` view → API-locality. **The cost-basis is closed.** Non-novel (`num_enum`/`bytemuck`/`zerocopy`). |
| **Constant-time secret** (different DOMAIN) | SEAM-BREAKS [horn 1, IFC sense] (1 cold panel) | breaks — a horn drops, *in the information-flow domain* | no `unsafe`; the danger is timing | *Branching on a secret leaks it via timing — **not UB**.* The first probe outside memory safety. `Secret<N>` forecloses branching *on* the secret, but `ct_eq(a,b).expose_bool()` then `if b` leaks a relationship — **declassification is etiquette, not type-structure**; closing it needs whole-program **undecidable** IFC. **The intrusive list's horn-drop, re-run in a new domain — the Seam Trilemma recurs, sharper.** Panel ruling: information-flow is **out of frame**, a *different domain* with its **own basis** (declassification discipline, …). **Closure is per domain; the framework is domain-parameterized.** |
| **Capability-gated declassify** (IFC domain) | SEAM-BREAKS [horn 1] — **5/5 unanimous** (1 cold panel) | breaks — *no closing IFC seam on Rust's floor* | an affine `Cap`, but the wall is open | *Probes whether the IFC domain has a **closing** seam.* It does not: `map(self, f)` hands the closure the **bare plaintext** (an uncapped exit a captured-`&mut` closure exfiltrates — **no effect system**), and the affine "budget" counts **uses, not bits** (a loop mints N caps; one cap → a megabyte). What it enforces — release *location* — **projects onto the affine memory-safety axis**; the real IFC coordinate (*amount* — **quantitative info-flow**, Smith min-entropy) an affine floor can't bound. **THE FINDING: a closing seam exists only when the FLOOR matches the DOMAIN.** Rust's affine floor: matched to memory safety, *mismatched* to IFC (needs Jif/FlowCaml/QIF). |
| **Type-level lock ordering** (third DOMAIN: liveness) | SEAM-BREAKS [horn 1] — FAILED, 3/5 broke it (1 cold panel, r21) | breaks — *partial* floor-match in a third domain | a viral `Floor<F>`, but the floor is open | *Acquiring a lock out of a global rank order deadlocks — **neither UB nor a leak**.* Type-level rank `L` + affine `Floor<F>`, `with` bounded `L: Below<F>`. Built to test the *positive* half of the floor-domain match; **broke two ways**, both "the floor under-carries the precondition." **(1) Granularity (fatal):** `Floor::root` is re-entrant, so two sibling lineages let safe code type a low-then-high *inversion* (verified compiles+runs) — per-token discipline vs a per-thread-global precondition; Rust's floor has *no ambient per-thread one-shot resource*. **(2) Coverage:** even patched, the dynamic-acyclicity fragment (runtime-ordered/same-rank locks — bank-transfer `min/max`, hand-over-hand coupling, IRQ ordering) escapes the *static* floor, exactly as IFC's quantitative coordinate escaped the affine one. **THE FINDING: the floor-domain match is GRADED (full/partial/none)** — liveness on a static-order floor is a **partial** match. Name = *lock-order-inversion freedom*, not "deadlock-freedom"; precedent forces it (Boyapati–Lee–Rinard OOPSLA'02 *minus* their dynamic extension; Kobayashi natural-number levels; soundness = Havender 1968). |
| **Runtime lock registry** (liveness; floor-UPGRADE) | SEAM-BREAKS [horn 1] — FAILED (1 cold panel, r22) | breaks — the floor-upgrade is **non-monotone** | a per-acquire runtime check; needs OS thread-local (not `no_std`) | *Same liveness danger as #15, on an **upgraded floor**: a `thread_local!` held-set of **runtime** ranks, checked per acquire (lockdep-in-miniature).* Built to ask whether *upgrading the floor lifts the match*. It fixed #15's **coverage** gap (runtime-chosen-fixed ranks close — bank-transfer `min/max`, hand-over-hand) and paid the predicted price — **#5 bare-metal** (needs OS thread-local → *cannot be `no_std`*, gated behind feature `lockdep` so the cost shows in the build graph), **#6 zero-cost** (a per-acquire check — the *slab's* place in a new domain), and a **prevention→detection modality** (the inversion compiles, is trapped only on *executed* paths; soundness path-coverage-dependent). **But granularity RECURRED one rung finer:** `HELD` is keyed on the OS *thread*, while the unit of control in async is the *task* — work-stealing migration resumes a rank-holding task on a fresh thread with an empty `HELD`, laundering an **undetected inversion** (verified: the registry is thread-blind); a single-threaded executor *false-panics* on independent tasks, and `!Send` closes neither direction fully. **THE FINDING: the floor-upgrade is non-monotone — the "granularity" axis is a containment-scope LADDER (token < thread < task), and no ambient Rust floor tops out** (there is no task-local; async is a library). Prior art forecloses the design: FreeBSD **WITNESS** / lockdep / Abseil all abandoned static ranks for dynamically-*learned* acquires-before graphs. |
| **Per-channel priority typing** (liveness; progress's *session* face, r33) | SEAM-BREAKS [horn 1] — FAILED, the **8th** broken seam (first soundness break since wcet #22; tally 7→8) | breaks — public `role()` types a deadlocking ring; *whole-program* acyclicity escapes the per-edge floor | a per-channel `Pi: Lt<Po>` bound, but the global cycle-check is open | *A source-less cycle of recv-before-send roles deadlocks — neither UB nor a leak, exactly #15's lock-order inversion one soundness sub-domain over (sessions, not locks).* A forwarder `Relay<Pi, Po>` recv-then-sends under a per-channel priority bound `Pi: Lt<Po>` (Kobayashi linear-π / **Dardha–Gay FoSSaCS 2018**) — the middle rung **(b)** #25's rejection named — built over #23's binary edges and #8's sealed `typed_nat::Lt` to **confirm #25's weaker thesis humbly**; it **broke the way `ordered_locks` did.** The "ring is untypable" argument (`p_CA < p_AB < p_BC < p_CA` chains to a contradictory `p: Lt<p>`) **secretly assumes edge-priority consistency** — that the `Nat` A *sends* on edge AB is the `Nat` B *receives* on AB — but Rust does **no transitive reasoning**: each `Pi: Lt<Po>` is checked in isolation, and the contradictory chain forms only when one `Nat` is shared per edge, which only the coherent `relay()` constructor threads. The **public** `role::<Pi, Po>` lets three `role::<N1, N2>` relays (every local `1 < 2` bound discharges) wire into the source-less ring: it **compiles, has no `unsafe`, and deadlocks** — falsifying §4.3, by a passing test. The sting: I had **described this exact flaw in §4** ("edge consistency is the constructor's invariant, not a per-endpoint type fact") and shipped `role()` public anyway — the soundness attacker quoted my own words. It **confirms #25's weaker thesis by breaking**: the per-channel `Lt` bound carries nothing global (it never names the dual endpoint), so deadlock-freedom rests on a **trusted edge-consistent constructor**; it salvages to SEAM-COSTS [horn 3] **only if `role()` is hidden**. The **same** wrong-granularity break as `ordered_locks` #15/#16 — type-level order carries the *local* bound while *whole-program* acyclicity escapes — so progress's **lock** face (#15/#16) and **session** face break identically. Lean `priority_typed_discipline_factors`. N stays 9; liveness stays the third domain. |
| **Projected priority builders** (liveness; progress's *topology* face — #26's sealed salvage, r34) | SEAM-COSTS [horn 3] — **SOUND** (the #26 salvage works), but **FAILED the §6 deliverable** — the *rejection mode*'s **4th firing** (after typed_nat #8, delegation #24, multiparty #25; #26 was a break; tally **stays 8**) | sound — a sealed `worker()` + trusted `relay`/`relay4` builders make the ring unconstructible; but the *execution* residue (run-all / in-order / no-drop) never leaves the consumer | a per-edge `Pi: Lt<Po>` threaded by a **trusted constructor** that carries topology; completion/execution still open | *#26's break, salvaged — and the deliverable still rejected.* **SEAL `worker()`** (module-private) and vend only the coherent `relay`/`relay4` builders, which thread strictly-increasing `Nat`s per edge, so #26's source-less ring is **doubly unconstructible**: by **access** (no public `worker()`) and by **structure** (a monotone priority chain can't close — the returning edge needs a *decreasing* `Lt`, no instance). Horn 1 conceded, the salvage holds (SOUND, r34). But §4's **"deadlock-freedom by construction"** + **"consumer carries nothing global"** over-claimed: the builder proves a deadlock-free schedule *exists* (a static `Nat` witness), yet affine Rust never *forces* the runtime to follow it — a run-all / in-order / no-drop **progress residue** stays on the consumer (the #17/#18/#23 completion gap). Honest: **"schedulability by construction, *given all roles execute*."** Concentration is **partial** — the trusted constructor carries **topology** (no cycle, the #26 fix), but **completion/execution never left the consumer**. Giveaway from my own citations: Scribble/mpstthree projected endpoints are deadlock-free *only if every participant runs*. The **progress coordinate decomposes** — deadlock-freedom/topology (constructor carries) vs completion/execution (consumer carries) — sharpening the #24–26 arc. Lean `projected_schedulability_closes`; N stays 9; liveness the third domain. |
| **Scoped session** (`must_scoped`; liveness; completion's *concentration* face — #29's scoped salvage, r37) | SEAM-COSTS [horn 3] — **HELD** (TRADEOFF survival like `restore` #19 / `binary_session` #23, **not** a rejection or a break; the concentration #29 *failed to predict*; tally **stays 8**; rejection-count **stays 6**) | sound — `session_scope(a,b)` vends two dual brand-trapped endpoints, each owing an unforgeable `Done<'scope>` (mintable **only** by `finish()` at `End`); no safe `Done` is forged/smuggled/forgotten — a **return-type** obligation, not a forgettable destructor, so `mem::forget` is **defanged**; horns 1 (no UB) & 2 (Θ(depth), no SMT) intact | no free `start` handle — the protocol runs *inside* `session_scope`; the `'scope` brand is lifted verbatim from branded #5 (generativity = `thread::scope`'s trick); completion concentrable but **per-endpoint** | *The scoped salvage of #29 — completion IS concentrable, closing #29's reversal.* #29's forgettable `Drop` bomb becomes an **unforgeable return-type** obligation: `session_scope(a, b)` (the self-contained two-endpoint form of the verdict's scoped `with_session` combinator) vends two brand-trapped endpoints, each **required** to return a `Done<'scope>` mintable only by `finish()` at `End`, so no safe `Done` is **forged, smuggled, or forgotten** (a *return-type*, not a destructor, defangs `mem::forget`). **HELD as a TRADEOFF** (like #19/#23, *not* a rejection or a break — rejection-count stays 6). The **#17–#28 completion residue was the price of the FREE-HANDLE API**, bought back by surrendering the free handle. THE SHARPENING (the merits-strongest attacker point, and the builder hedged it correctly — *the caveat is the finding*): concentration is **per-endpoint, not whole-session** — §2 over-claimed "every session runs to End," but `session_scope` forces only each *returning* closure's own endpoint, so under run-a-then-b `a` finishes while `b` **diverges**, starving the peer — **global completion escapes via peer divergence** (the #23/#27 per-session-vs-global gap, one step on). Secondary: the **synchronous trap** is understated (sequential run-a-then-b cannot express concurrent/interleaved sessions). META (as #29): I over-claimed the *extreme* again — #29 "irreducible," #30 "whole-session global"; the truth is the **per-endpoint** cost between. DEFLATION (conceded): no new mechanism — `thread::scope` + generativity (branded #5) + #29's `finish`/`End` typestate. Lean `must_scoped_completion_closes`; N stays **9**; liveness the **third** domain. |
| **Recursive session** (`recursion`; liveness; the looping connective — #30's boundary, r38) | SEAM-COSTS [horn 3] — **SOUND, DELIVERABLE REJECTED** (the *rejection mode*'s **7th firing** — after typed_nat #8, delegation #24, multiparty #25, projected #27, choice #28, must_complete #29; **not** a break; tally **stays 8**; rejection-count **now 7**; N stays **9**) | sound — single-level `Rec<P>`/`Var` over an env-threaded `Session<P, E>` (default `NoRec`); `enter()` unfolds, `recur()` fires only for `Session<Var, Rec<P>>` so a top-level `Var` is **inert** (the **branded-#5 analogue**, no UB — Horn 1); decidable, Θ(type-size), no SMT — Horn 2; without choice single-level recursion is necessarily **non-terminating** (no reachable `End`) | `HasDual` threads recursion; the **#30-bounding finding** — no reachable `End`, so #30's `Done`-forcing is **vacuous** (recursion exposes #30's unstated precondition, a *reachable terminal*); guardedness carried by a sealed `Guarded` marker in Θ(type-size); only **termination** stays the undecidable NONE | *The last unbuilt structural session feature, built to probe #30's boundary at unbounded depth — sound, but the deliverable **rejected**, the rejection mode's **7th** firing.* WHAT HELD: the `Var`/environment encoding is sound (Horn 1, the **branded-#5 analogue** — a top-level `Var` is **inert**, no UB), decidable (Horn 2, Θ(type-size)), and the **#30-bounding finding** — single-level recursion without choice has **no reachable `End`**, so #30's `Done`-forcing obligation is **vacuous**: recursion exposes #30's unstated precondition, a *reachable terminal*. THE OVER-CONCESSION (I filed it against myself): I put **productivity / guardedness** at the **undecidable** extreme beside termination — but it is a **DECIDABLE cost**. `Rec<Var>` is the non-contractive `µX.X` that every cited system (Coq / Agda **guarded corecursion**, contractive recursive types) rejects **syntactically**, and a sealed **`Guarded` marker** (impl for Tx/Rx/End/Rec, **not** `Var`) carries it in Θ(type-size) with no SMT — the **same machinery as order**. So guardedness **CONCENTRATES (horn 3)**, and **only TERMINATION is the genuine undecidable NONE** (halting). Refuted from my **own cited guarded-corecursion lineage** again (#24/#25/#29). THE META, the **inverse** of the usual over-claim, the same shape: prior rejections over-claimed concentration *total*, #29/#30 over-claimed the danger *irreducible / global*; here I **over-conceded** — put a decidable cost at the undecidable extreme — fleeing the **middle** toward whichever extreme is cleaner. CORRECTED: order + duality + **guardedness** CLOSE; **termination alone** is the NONE. Lean `recursion_guarded_order_closes` (the sound `Var`-order + guardedness core *closes*, renamed from `recursion_order_unfolding_closes`); N stays **9** (rejection's 7th firing); liveness the **third** domain; the natural **#32 salvage** is the sealed `Guarded` marker so `Rec<Var>` won't form. |
| **Guarded recursion** (`guarded`; liveness; productivity — #31's salvage, r39) | SEAM-COSTS [none] — **SURVIVED** (1/1, r39; the salvage **HELD**, the **cleanest result in the arc** — all three horns hold, no horn dropped, **stronger than #30's [horn 3]**; tally **stays 8**; rejection-count **stays 7**; N stays **9**) | sound — a sealed **`Guarded` marker** (a `sealed::Sealed` supertrait; impl for `Tx<C>`/`Rx<C>`/`End`, **not** `Var`) bounds `enter()`/`recur()`, so the non-contractive `Rec<Var>` = µX.X **cannot unfold**; **all three horns hold** — Horn 1 (no UB), Horn 2 (decidable, Θ(type-size), no SMT), Horn 3 (zero-ceremony concentration) — no horn dropped | O(1) — a **shallow head-check** that for single-level recursion **IS full contractivity** (the action head guards everything inside, `End` vacuous, a bare `Var` head unguarded — the **same machinery as order**); richer grammar (nested `Rec`, choice-in-recursion) needs the real recursive walk, **scoped out, not escaped** | *The sealed-marker salvage of #31 — guardedness **IS concentrable**, closing #31's over-concession; the cleanest result in the arc.* #31's rejection prescribed exactly this fix (*the sealed `Guarded` marker so `Rec<Var>` won't form*), mirroring #26→#27 and #29→#30, and the panel **CONFIRMED** it: `Rec<Var>` = µX.X is the non-contractive loop every cited system (Coq / Agda **guarded corecursion**, contractive recursive types) rejects **syntactically**, and the `Guarded` bound rejects it in Θ(type-size) with no SMT. **SURVIVED [none]** — all three horns hold, no horn dropped, **stronger than #30's [horn 3]** (no safe unfold of `Rec<Var>`, no UB, zero-ceremony concentration, decidable no-SMT, no false positive). It **CLOSES #31's over-concession** (guardedness was filed as the undecidable NONE; it **concentrates** at horn 3), and **avoided the #27 salvage-over-claim trap** by claiming **only productivity** and conceding **termination** up front. THE MARKED TRADEOFF (a tradeoff, **not** a horn-drop): **completeness bought by restriction** — the head-check equals full contractivity *only* because the single-level choice-free grammar collapses it to "head ≠ `Var`"; richer grammar needs the recursive walk, **scoped out, not escaped**, and my "folklore" deflation slightly overstated generality. It **buys productivity, NOT termination** — `Rec<Tx<Var>>` is guarded and still infinite, so **halting stays the one conceded undecidable NONE**. The final split: **order + duality + guardedness/productivity all CLOSE; termination alone is the NONE.** Lean `guarded_recursion_productivity_closes` (a `..._closes`); tally **stays 8**; rejection-count **stays 7**; N stays **9**; liveness the **third** domain. |
| **Integer division** (`divisor`; panic-freedom → application-correctness; totality — the **first pivot off the session family**, r40) | SEAM-COSTS [horn 3] — **SOUND, DELIVERABLE REJECTED** (the *rejection mode*'s **8th firing** — firings #8/#24/#25/#27/#28/#29/#31/#33; **not** a break; tally **stays 8**; rejection-count **now 8**; N stays **9**) | sound — `Divisor(NonZeroI32)` vends division on the **real floor** (`NonZeroI32`, the one refinement type stock Rust ships — no SMT); `divide` overflows on **exactly** `a == i32::MIN && d == -1`, and the divisor-side conjunct `d ∉ {0, -1}` makes division **total for every `a`** via the same Θ(1) mechanism — Horn 1 (no UB — a panic is *defined behaviour*, a clean abort), Horn 2 (decidable point-exclusion, no SMT), Horn 3 (concentration) | O(1) — a `NonZeroI32` newtype; overlaps refined #6 in **mechanism**, but the **domain** finding is the contribution | *The first pivot off the exhausted session family — panic-freedom FOLDS onto application-correctness, no fourth domain; the deliverable rejected, the rejection mode's **8th** firing.* THE HEADLINE: **panic-freedom folds into the application-correctness basis** (#19/#21), **not a fourth soundness domain** — a panic is *defined behaviour* (a clean abort, never UB), so with `crdt` #21 (fold) and `wcet` #22 (break) this is the **third** fourth-domain probe to deflate, and **all three** have now deflated with **none** opening a fourth domain — the confirmed **three**-domain map (memory safety / information flow / liveness) looks **genuinely special / possibly complete**. Prior art confirms the fold (SPARK `division_check`/`overflow_check` split; Liquid-Haskell nonzero-div; `core::NonZero`); expressiveness granted it. THE REJECTION, the **SAME tic as #31, EXACTLY INVERTED onto a new domain**: #31 over-conceded **guardedness** as an undecidable NONE (a decidable marker); #33 over-conceded the `i32::MIN / -1` **overflow** as undecidable range-analysis (a **decidable point-exclusion**) — both **flee the MIDDLE toward the cleaner EXTREME**, here **over-concession** toward undecidable (the #31 direction), both refuted **from my own cited framing** (#33 from its §2, which lumped division overflow with arbitrary `+`/`*`), so **calibration is the recurring axis**. THE TECHNICAL CORE: "two sources — one closes, one escapes" is **WRONG for division: BOTH close**; returning `Result` forces `.unwrap()` on provably-total divisions, **reintroducing the panic the seam exists to remove**. The natural **#34 salvage** (**SETTLED** by #34, mirroring #26→#27, #29→#30, #31→#32): a `TotalDivisor` (`d ∉ {0, -1}`) vending total `-> i32` division, proving the overflow residue **concentrates too**, leaving only arbitrary `+`/`*` range analysis as the genuine undecidable NONE. Lean `divisor_nonzero_division_closes` (a `..._closes`, like #31's rejection); tally **stays 8**; rejection-count **now 8**; N stays **9**; soundness-domain map stays **three**. |
| **Total division** (`total_div`; panic-freedom → application-correctness; totality — #33's sealed-total salvage, r41) | SEAM-HOLDS [none] — **SURVIVED** (1/1, r41; the salvage **HELD**, the **cleanest verdict in the whole project** — a full HOLD, no horn even **costed**, **stronger than #32's [none]**; soundness + implementation + expressiveness all **ARGUED-PAST**; tally **stays 8**; rejection-count **stays 8**; N stays **9**) | sound — a `TotalDivisor(i32)` carrying the **finite point-exclusion** `d ∉ {0, -1}` vends **total** `-> i32` division with **no panic path**; overflow is **exactly** `a == i32::MIN && d == -1`, and excluding `-1` costs nothing safe (`a / -1 == -a`, whose sole overflow `i32::MIN` **coincides with negation's own**) — **all three horns hold**, no horn dropped: Horn 1 (no UB), Horn 2 (decidable point-exclusion, no SMT), Horn 3 (concentration) | O(1) — a `TotalDivisor(i32)` newtype; the genuine (narrow) advantage is the **amortized reusable runtime divisor** | *The sealed-total salvage of #33 — the overflow residue **concentrates too**, closing #33's over-concession; the cleanest verdict in the whole project.* #33's rejection prescribed exactly this fix (a `TotalDivisor` `d ∉ {0, -1}` vending total `-> i32` division), mirroring #26→#27, #29→#30, #31→#32 — the **fourth** reject→salvage pair — and the panel **CONFIRMED** it: division overflows on **exactly** `a == i32::MIN && d == -1`, so `d ∉ {0, -1}` makes `a/d` total for every `a` via one Θ(1) check — "two sources, one closes, one escapes" was **WRONG: BOTH close**. **SURVIVED [none]** — soundness, implementation, and expressiveness all **ARGUED-PAST**, no horn even costed, **stronger than #32's [none]**. It **CLOSES #33's over-concession** (the `i32::MIN / -1` overflow was a **decidable point-exclusion**, not undecidable range-analysis). It does **NOT** move #33's **DOMAIN** finding — panic-freedom still **folds into application-correctness**, all **three** fourth-domain probes (`crdt` #21 / `wcet` #22 / `divisor` #33) **stay deflated**, the **three**-domain map intact; #34 corrects the **residue LINE within division**, not the domain map. THE META (as #32): it **HELD by dodging the #27 over-claim trap** — pre-conceding **three non-claims** (no fourth domain, no `+`/`*` generalization, value-threaded) — but the panel marked the **calibration tic in a milder form**: a raw-`/` **strawman** benchmark and **omitting the floor's own total `checked_div`/`checked_rem`** from prior-art overstated the win, so the genuine advantage is **narrow** (the amortized reusable runtime divisor). A **marked tradeoff below the horn-deciding threshold** (as #32) — calibration the recurring axis, and **improving** (horn-deciding rejections #8–#31 → marked tradeoffs #32/#34). The reject→salvage rhythm **converged in one step** (#33 rejected → #34 held cleanly, as #31→#32). Lean `total_div_both_sources_closes` (a `..._closes`); tally **stays 8**; rejection-count **stays 8**; N stays **9**; soundness-domain map stays **three**. |
| **Shared mutability** (`sync_cell`; concurrency / data-race → memory safety; the **FOURTH** fourth-domain probe, Phase AH, r42) | SEAM-BREAKS [horn 1] — **FAILED**, the **ninth** broken seam (the **first concurrency break**; tally **8→9**, the **first increment since the survivals** — but a **conceded folklore break**, a strawman probe vehicle; rejection-count **stays 8** — a break, not a rejection; N stays **9**) | breaks — safe `thread::scope` races a non-atomic write into UB with **zero consumer `unsafe`**; but the *classification* over-reaches — DRF is **not floor-closed** but **floor-propagates-and-concentrates** (like memory-aliasing) | no `unsafe` in the break witness; the danger is a data race — and `SpinMutex<T>`'s **discharged** `unsafe impl<T: Send> Sync` is the concentrating seam that closes it (the **concurrency slab**) | *The first concurrency probe — it **breaks by design** (a strawman vehicle: safe `thread::scope` races a non-atomic write into UB, zero consumer `unsafe`), but the **synthesis is the reclassification**, not the break.* THE STAKED CLAIM REFUTED: "no value-adding concurrency seam exists; DRF is **FLOOR-CLOSED**; a re-import is **CATEGORICALLY an unchecked axiom**" is **false** — **`SpinMutex<T>`**, a sound `core`-only concurrency seam, carries an `unsafe impl<T: Send> Sync` **DISCHARGED by the lock invariant** (mutual exclusion serializes access — the concurrency **slab**). CORRECTION: distinguish **DISCHARGED** from **FALSE** `unsafe impl`s, and **reclassify DRF** as **"floor-PROPAGATES-but-does-not-close,"** **ANALOGOUS to borrowck / memory-aliasing** — closure comes via **concentrating seams** (locks / channels / atomics), not the floor alone. THE DOMAIN-MAP UPDATE: the prior implicit **trichotomy** (IFC closes-nothing / aliasing concentrates / concurrency floor-closed) **COLLAPSES to a DICHOTOMY** — **IFC** is the special floor-closes-nothing one, while **aliasing AND concurrency** *both* **floor-propagate + concentrating seams close** — so memory safety now carries **two** concentrate-behind-a-seam coordinates (**aliasing AND data-race**); **still three** soundness domains (DRF is **NOT** a fourth — a data race **IS UB**, inside memory safety), the map **survives**, its internal structure clearer. THE META: the **same clean-framing tic as #33** — omitting the obvious counterexample (#33 `checked_div`; #35 `SpinMutex`) — but a **HORN-DECIDING FAILURE**, not #32/#34's marked tradeoff: on a **fresh domain**, over-confidence reasserted the tic at **full strength**, so calibration is **NOT monotonic** (recedes on familiar ground, re-fires full-strength on novel). The natural **#36 salvage** (OWED, **not yet built**, mirroring #26→#27's break→salvage): the `SpinMutex<T>` (discharged `unsafe impl`) proving DRF **concentrates behind a seam**, closing the over-claim. Lean `sync_cell_datarace_factors` (a `..._factors`, like `priority` / `wcet` — it breaks); tally **rises to 9**; rejection-count **stays 8**; N stays **9**; soundness-domain map stays **three**. |
| **Spinlock mutex** (`spin_mutex`; concurrency / data-race → memory safety; the **salvage of #35**, Phase AI, r43) | SEAM-BREAKS [horn 1] — **FAILED**, the **tenth** broken seam (the salvage **itself broke** — the first to; tally **9→10**, the **9th DISCOVERED** break — #35 the lone conceded folklore; rejection-count **stays 8** — a break, not a rejection; N stays **9**) | breaks — an **omitted conjunct**: I discharged the lock's exclusion but dropped the guard's `T: Sync`, so `SpinGuard` auto-derives `Sync` for every `T: Send` and a safe `thread::scope` shares `&guard` over `T = Cell<i32>` (`Send + !Sync`) into a data race, **zero consumer `unsafe`**, lock never contended | too-loose `unsafe impl<T: Send> Sync` (std writes `T: Sync` + guard `!Send`); but a **correctly-bounded** `SpinMutex<T: Sync>` **IS sound** (`spin::Mutex`; RustBelt POPL 2018) — the **classification survives the break** | *The `SpinMutex<T>` salvage owed for #35's over-claim (mirroring #26→#27's break→salvage) — and the method **broke the salvage itself**, the **first** salvage to break, the same **omitted-conjunct** way #35 did.* THE BREAK (DISCOVERED, verified): a concurrency `unsafe impl` carries **two conjuncts** — cross-thread exclusion (the lock, **discharged**) AND the guard's no-`&T`-leak bound (`T: Sync`, **OMITTED**). With no manual guard impl, `SpinGuard` auto-derives `Sync` for every `T: Send` via my own `unsafe impl<T: Send> Sync for SpinMutex`, so `T = Cell<i32>` (`Send + !Sync`) lets a safe `thread::scope` share `&guard` and race `set`/`get` into UB, **zero consumer `unsafe`**, lock never contended — where `std` writes the stricter `unsafe impl<T: Sync> Sync for MutexGuard` (+ `!Send`). WHAT SURVIVES: the **#35-refuting existential** — a *correctly-bounded* `SpinMutex<T: Sync>` **IS sound** (`spin::Mutex`; RustBelt POPL 2018) — so the **concentrate-behind-a-seam CLASSIFICATION** (DRF closes via concentrating seams, the same bucket as memory-aliasing; **discharged** ≠ **false** `unsafe impl`) is **NOT refuted**: the *artifact* had a hole, the *thesis* holds, DRF still **no** fourth domain. THE META, the sharpest instance of the tic: the **omit-a-conjunct** tic (as #35 deleted the exclusion obligation, #33 omitted `checked_div`) recurred **INSIDE the very salvage meant to prove I'd corrected it**, so **calibration is NOT converging on the concurrency domain** — the concurrency `unsafe impl`'s full obligation set only got mapped **under the gauntlet**, sharpening #35's non-monotonic finding: on the novel concurrency domain the tic **fires repeatedly** until the obligations are exhausted. The **#35→#36 pair does NOT settle** — the salvage broke, the chain **extends to #37** (the correctly-bounded `SpinGuard<T: Sync>` + `!Send`). Lean `spin_mutex_guard_sync_factors` (a `..._factors` — it breaks, like `sync_cell`); tally **rises to 10**; rejection-count **stays 8**; N stays **9**; fourth-domain probes stay **four** (a salvage, not a probe); soundness-domain map stays **three**. |
| **Spinlock, corrected** (`spin_lock`; concurrency / data-race → memory safety; the **salvage of #36**, Phase AJ, r44) | SEAM-HOLDS [none] — **SURVIVED** (1/1), the salvage **HELD** — the **cleanest verdict tier** (a full HOLD, no horn even costed, like `total_div` #34; I predicted [horn 3] and UNDER-claimed); tally **stays 10** (a HOLD — no increment); rejection-count **stays 8** — a survival, not a rejection; N stays **9** | holds — enumerates **all four** of the mutex's auto-trait obligations and discharges each; the added `unsafe impl<T: Sync> Sync for SpinLockGuard` (obligation 3) closes #36's omitted conjunct, so a shared `&guard` demands `T: Sync` and `T = Cell<i32>` (`Send + !Sync`) is rejected; VERIFIED — the exact r43 laundering path that COMPILED against `SpinMutex` #36 is now REJECTED (`error[E0277]: Cell<i32> cannot be shared between threads safely`) | correctly-bounded `unsafe impl<T: Sync> Sync` on the guard (+ auto-`Send`, sound for a thread-agnostic Release store, `spin::MutexGuard`'s reasoned choice, vs `std::MutexGuard`'s `!Send`); the pre-registered hunt for a **FIFTH** omitted conjunct found **NONE** (six break attempts failed) | *The correctly-bounded spinlock salvage owed for #36's broken salvage (mirroring #26→#27, #29→#30, #31→#32, #33→#34's break→salvage) — and the method **CONFIRMED it**, the salvage **HELD**, the **first salvage of a broken salvage to hold**.* THE FIX: #36 discharged **three** of a mutex's four auto-trait obligations and dropped the fourth; #37 enumerates and discharges **all four**, the added line `unsafe impl<T: Sync> Sync for SpinLockGuard` (obligation 3 — a shared `&guard` hands `&T` to many threads, sound only when `T: Sync`, opting the guard out of the too-weak auto-`Sync = T: Send`); the r43 break closed, verified `E0277`. The four obligations: (1) `SpinLock<T>: Send where T: Send`; (2) `SpinLock<T>: Sync where T: Send` (exclusive access ⇒ T migrates, like `std::Mutex`); (3) `SpinLockGuard<T>: Sync where T: Sync` (THE FIX); (4) `SpinLockGuard<T>: Send where T: Send` (auto; sound — the Release store is thread-agnostic, unlike an OS futex, so a migrated guard is sound; `spin::MutexGuard`'s choice, vs `std::MutexGuard`'s `!Send`). HOW IT SURVIVED: the pre-registered sharpest attack — hunt a **FIFTH** omitted conjunct (the #36 failure mode) — found **NONE**; six break attempts (`T: Sync + !Send`; the r43 `Send + !Sync` shape; a Send-migrated guard; variance via the invariant `UnsafeCell`; ordering; poison/forget) all failed; three independent reconstructions converged; all three horns intact. THE STAKED CONTRIBUTION CONFIRMED: a sound, value-adding, `core`-only concurrency seam EXISTS (the thing #35 said could not) — so **DRF is a concentrate-behind-a-seam domain** (locks its seams, the concurrency analog of SPSC #1 / the slab #4; the **memory-aliasing bucket**, its **DYNAMIC end**, like `RefCell` — vs SPSC/slab which discharge *statically*), a **discharged** `unsafe impl` categorically distinct from a **false** one (#35 SyncCell), DRF still **NOT** a fourth domain (a data race **IS UB**) — and the **#35→#36→#37 chain SETTLES** it, the **fifth** settled reject/break→salvage lineage and the **only TWO-STEP** one (two steps because #36 broke). TWO RECORDED TRADEOFFS (soundness NOT contingent — **not** TRADEOFF-mode): overstated `?Sized`/IRQ parity (`SpinLock<T>` not `T: ?Sized`, so `SpinLock<[u8]>` / `SpinLock<dyn Trait>` unwritable, no `into_inner`, IRQ use deferred to an unbuilt `irqsave` — the tic's MILDEST form, a marked tradeoff below the horn-deciding threshold, as #34); novelty ~NIL (obligation 3 is VERBATIM **Ralf Jung's June 2017 `MutexGuard`-`Sync` std bug** + shipped patch; even the *classification* is Jung's; RustBelt POPL 2018 proved it — a famous fix copied, not folklore). THE META CODA: **concurrency took THREE seams to map** (#35 break, #36 break, #37 hold); the **omit-a-conjunct** tic fired **full-strength TWICE (#35, #36) then RECEDED to marked-tradeoff on #37** — #35's non-monotonic finding's coda: on a fresh domain the tic fires **repeatedly until the obligation set is exhausted, THEN recedes** — convergence over three seams, not one; the concurrency salvage lineage is now closed. Lean `spin_lock_guard_sync_closes` (a `..._closes` — it holds); tally **stays 10**; rejection-count **stays 8**; N stays **9**; fourth-domain probes stay **four** (a salvage, not a probe); settled lineages now **five**; soundness-domain map stays **three**. |

### The deeper result: the Seam Trilemma is a 3-of-N projection (N ≥ 9)

The slab forced the first un-bundling. It keeps *all three* trilemma horns yet still pays — on
**zero-cost erasure (#6)**, a property the trilemma had abstracted into "concentrated & proof-free."
So the three horns were never the whole story: they project a four-property tension whose fourth
axis (zero runtime cost) only becomes visible when the dangerous conjunct is a *runtime-checkable*
fact about a `Copy` handle — exactly the `RefCell`/`slotmap` move.

The **branded index** then forced a second, sharper un-bundling — of *concentration itself*. It
aces seam-width (one `scope` per slice, O(1) code crossings) and is RustBelt-verified sound, yet the
cold panel located a real cost "outside the three horns": the brand `'id` is a **viral generative
type parameter** — it never appears in consumer *code* as a crossing, but it appears in every
consumer *type*, and traps every index inside a `scope` closure. That **API-locality** cost is *not*
seam-width: the `runST` (PLDI'94) / lightweight-capabilities (PLPV'06) lineage pays the identical
tax in languages with **no seam or `unsafe` at all** — a cost that exists where seam-width is
undefined cannot *be* seam-width. So "concentration" was a bundle of two faces, a *code* face
(seam-width) and a *type* face (API-locality), and the branded index pulls them apart. The same
primitive surfaced **#7 (AI-legibility)** as a further axis: a brand is a nominal token (the
in-bounds proof lives in the foundation `unsafe`, not in any readable type), and the panel argued it
dissociates from #5 — a refinement type `{v:Int | v < len}` is brand-free yet legible.

The **refined index** then *witnessed* that argued dissociation and forced a third un-bundling — of
*horn 2 itself*. It is the **(brand-free, legible)** corner: a refined value carries a visible
predicate and *escapes its constructor* (a compile error for the branded `Idx<'id>`), so paired
against the slab — reclassified **brand-free yet illegible** (two attackers endorsed it; a `Copy`
`Key`'s validity is not reconstructible from its type) — it holds #5 fixed and flips #7. The cold
judge ruled **#5⟂#7 closes — yes**: AI-legibility is a confirmed independent axis. And horn 2 turns
out to be a bundle of **decidability** (the refined fragment terminates) and **solver-freedom** (it
needs an SMT solver in the trusted loop) — the latter being the `unified` "keep" horn, not a new
axis, so the map gains resolution there rather than a dimension.

The **shared-ownership seam** (a minimal `Rc`) then forced a fourth un-bundling that *does* add a
dimension: it recovers seed property **#5 bare-metal**. Dynamic shared ownership — a live-owner set
fixed at run time — is a precondition shape affinity, NLL, and the slab's single-owner pool all
structurally cannot express; its only sound witness is a runtime refcount **manager**, which needs a
global heap. The judge confirmed bare-metal is a genuine seventh axis (dissociates from zero-cost via
the slab/`heapless` off-diagonal: a fixed pool is bare-metal, a global-heap `Rc` is not) **but
deflated its novelty** — it is Rust's own `core`/`alloc` boundary (RFC 2480), not a discovery.

The **type-level-proof index** then probed for an *eighth* axis — and **failed**, which is recorded
as a result rather than buried. Pitched as recovering **checker-tractability** (does the native
checker halt without an arbitrary `recursion_limit` knob?), it is sound (unanimous — sealing plus the
orphan rule block every forgery) but recovers no clean axis, on three convergent grounds the cold
panel converged on: (1) the off-diagonal is **confounded** — `get::<I>` takes a turbofish *type*, so
the seam is static-index-only, a strict expressive subset of the branded/refined indices; the claimed
one-coordinate flip silently also flips expressiveness, and that confound is *intrinsic* (a runtime
index cannot be carried by a compile-time proof); (2) the cost is **encoding-relative** — unary Peano
hits the limit (E0275), binary type-level naturals (typenum) are O(log N) and do not; (3) it is
**mislabeled** — `Lt` resolution is *linear*, cheaper than refined's QF-LIA, so the real concern is a
**terminating-resolution** fragment (Paterson conditions / GHC `UndecidableInstances`), pre-existing.
That probe's worth is the falsification — the discipline can refuse.

The **take-and-replace seam** then supplied the *clean* off-diagonal the type-level index lacked, and
**confirmed an eighth axis (N ≥ 8): unwind-safety.** Its danger is *time*, not bounds — moving a value
out of a `&mut T` and back leaves a duplicate in the gap, and an unwinding panic drags the caller into
a double-drop; the sound witness is an **abort-bomb** (`take_mut`'s pattern). It **aces all seven prior
axes and pays on unwind-safety alone** — the panel explicitly contrasted this clean isolation with the
type-level index's confound. Deflated to pre-existing (Abrahams basic/strong/nothrow; C++ `noexcept`;
Rust `UnwindSafe`; RFC #1508 rejected this exact `take_mut` from std on this very cost). The panel's
mandatory relabel: the axis is the **exception-safety-guarantee-level**, and the abort is *this seam's*
discharge of it — a `replace_with` sentinel discharges the same obligation soundly-and-recoverably by
paying **API-surface** instead. So the axis carries a **discharge menu** (abort / repair / poisoning),
a third kind of inter-axis structure beyond the two exchange-trades.

The **drain-and-yield seam** then confirmed a **ninth axis (N ≥ 9): leak-freedom**, distinct from its
sibling unwind-safety. Its danger is the Rust **leakpocalypse**: `mem::forget` is a *safe* function, so
a `Drain` guard whose `Drop` does load-bearing cleanup can be leaked, and a naive impl double-frees the
moved-from slots. The sound witness is **pre-pessimization** (`Vec::drain`'s real fix — empty the buffer
up front, so a leaked guard loses the un-yielded tail but never double-frees). It **aces all eight prior
axes and pays on leak-freedom alone**, and crucially **aces API-locality** — a free-standing `Drain<'a>`
with *no closure* — which is why it avoided the confound that would have sunk the obvious leak-freedom
witness (a scoped-thread guard, whose closure *is* an API-locality cost). The panel ruled it **distinct
from unwind-safety**: the off-diagonal is populated both ways (take is unwind-sensitive/leak-safe, drain
is leak-sensitive/unwind-safe — "`Drop` never runs" vs "`Drop` runs at a bad time", codified separately
in Rust as *Leaking* vs *Exception Safety*). Deflated to pre-existing (leakpocalypse / PPYP / leak
amplification / RFC 1066), and flagged the **thinnest, most-deflatable** axis — its cost (leaked-tail
data loss) is invisible in non-leak code — yet a genuinely populated off-diagonal, not a non-axis.

The **self-referential-cell seam** then probed for a *tenth* axis — **move-stability** (a value whose
address is load-bearing) — and the panel returned the project's most novel verdict: **real but
structurally inseparable (N stays 9).** The danger is genuinely distinct (no other axis concerns a
value's address) and soundly handled (`Pin` over a `!Unpin` cell makes the post-`init` move
unconstructible). But it is **not a clean tenth axis**, because it has *no isolable cost*: `Pin<&Self>`
is a viral wrapper that **revokes `&mut`** (and that revocation *is* the soundness mechanism, so the
seam cannot ace API-locality), and the cost is **non-uniform across witnesses** (stack-pin spends
API-locality, `Box::pin` spends bare-metal). The honest price is **immovability** — heavier than
API-locality, and inseparable from the danger. So move-stability is neither a clean axis (like
unwind-safety/leak-freedom) nor a confounded non-axis (like checker-tractability): a **fourth category**.
Two caveats the panel extracted: the inseparability is *contingent* (RFC 2349's rejected `?Move`
immovable types were a designed free-standing witness Rust chose not to adopt), and the crisp `core`-only
reading holds only for a *non-composable* single-frame toy (real intrusive nodes / futures force
`unsafe` pin-projection or `Box::pin`).

The **validity seam** then asked the question all the probes were circling — **is the cost-basis
closed?** — and supplied the constructive arc's conclusion. It re-imports a danger no prior primitive
touched (a **validity invariant**: a `#[repr(u8)]` enum byte that must be a valid discriminant, UB even
without a deref) — a genuinely new danger *class*. The cold panel's **unanimous** ruling: it adds **no
tenth axis**; every witness for it spends a *pre-existing* coordinate. Its own cost is **(3) inseparable**
(a second example after move-stability): a single checked read spends **zero-cost #6** (a runtime tag
check, the slab's axis via a different predicate), but the zero-copy `&[u8]→&[Tag]` view that restores
concentration spends either eager #6 or a sealed constructor's **API-locality**. This is the **dual** of
the `get_unchecked` family: there *one* danger had *many* witnesses spending *different* axes; here *many*
dangers converge on the *same* axis. Read together, the two directions say the cost-space is a **small
fixed basis** that dangers *project onto*. (The adversary earned its keep to the last: two of three
gauntlets found real spec bugs — a *range* check unsound for non-contiguous discriminants, then a
residual stale `b<=2` — before the corrected seam survived; and the result is non-novel, the shipped
design of `num_enum`/`bytemuck`/`zerocopy`.)

So the trilemma is a low-dimensional shadow of a **closed object** — **nine clean axes** (#6, #2's two
faces, #4's two faces, #5, exception-safety, leak-freedom), **an inseparable category** (move-stability,
validity — recurring, not a one-off), and **two outside-the-axes** (the MMIO TCB axiom, the intrusive
horn-drop) — recovered one primitive at a time, with one candidate (checker-tractability) **rejected** as
confounded and the basis shown **closed** by a new danger class projecting cleanly onto it.

**…and closed *per domain*.** A thirteenth primitive — a **constant-time secret** — stepped outside
memory safety entirely, re-importing an *information-flow / timing* danger (branching on a secret leaks
it; **not UB**). The cold panel **generalized the whole result**: information-flow is **out of frame** — a
*different soundness domain* with its **own basis** — so "the cost-basis is closed" refines to **closed
*per domain***. Two confirmations: the secret seam **BREAKS [horn 1] in the IFC sense** (the intrusive
list's horn-drop re-run in a new domain — `ct_eq(a,b).expose_bool()` then `if b` leaks a secret
relationship; declassification is *etiquette, not type-structure*; closing it needs whole-program
undecidable IFC, so a concentrated decidable seam *cannot* enforce non-interference — **the trilemma
recurs, sharper**); and the IFC domain has its own axes with **no memory-safety analog** —
**declassification discipline**, plus termination/concurrency channels. (Honest correction the panel
forced: constant-time codegen is *dischargeable* — CompCert-CT/Jasmin/FaCT — not an MMIO-class irreducible
axiom.)

**The deepest refinement: a closing seam exists only when the *floor* matches the *domain*.** A
*fourteenth* primitive — a **capability-gated declassification** (`labeled.rs`) — probed whether the IFC
domain has a *closing* seam (the IFC analog of SPSC). It **also breaks, unanimously**: to *compute* on a
secret, `map(self, f)` must hand the closure the **bare plaintext**, and a captured-`&mut` closure
exfiltrates it (`s.map(move |x| sink.push(x))`) — cap-free, no `declassify` site, **unpatchable without an
effect system the floor disclaims**; and the affine "budget" counts *uses*, not *bits* (a loop mints N
caps; one cap leaks a megabyte). What it *can* enforce — release *location* — **projects onto the
memory-safety affine axis**; the genuinely-IFC coordinate is the *amount* (**quantitative information
flow**, Smith min-entropy), which an affine floor cannot bound. So **both IFC seams break, for one
reason: Rust's affine floor carries none of the IFC precondition.** It is *matched* to memory safety (SPSC
closes free, basis closed) and *mismatched* to information flow (no seam closes). So the deepest form of
the result is layered: **the Seam Trilemma is domain-general; the cost-basis is per-domain; and *which
seams close* depends on the floor-domain match.** Memory safety mapped (on a matched floor), information
flow sketched (every seam breaks on Rust's floor — pointing at the quantitative/effect floor it needs:
Jif, FlowCaml, QIF). The build-and-break method transfers — and *diagnoses the floor mismatch* itself.

**The match is GRADED, not binary — a third domain shows the middle value.** A *fifteenth* primitive —
**type-level lock ordering** (`ordered_locks.rs`) — stepped into a *third* domain, **liveness/progress**
(deadlock is neither UB nor a leak), to test the *positive* half of the floor-domain match: does a closing
seam reappear once the floor (a type-level rank order) carries the new domain's precondition (an acyclic
global order)? The cold panel ruled **SEAM-BREAKS [horn 1], FAILED** (3/5 attackers, one fatal) — and the
break is the richer result, because it grades the match. It fails along *two* independent dimensions, both
"the floor under-carries the precondition": **(1) granularity** — `Floor::root` is re-entrant, so two
sibling lineages let safe code type a low-then-high *inversion* (verified: it compiles and runs); the
discipline is *per-token* while the precondition is *per-thread-global*, and Rust's floor has no ambient
per-thread one-shot resource — so even the static fragment does not close; **(2) coverage** — even patched,
the *dynamic*-acyclicity fragment (runtime-ordered / same-rank locks — bank-transfer `min/max`,
hand-over-hand coupling, IRQ ordering) escapes the *static* type-level-order floor, exactly as IFC's
quantitative coordinate escaped the affine one. Prior art forces the ruling: this is **Boyapati–Lee–Rinard
OOPSLA'02 minus their dynamic tree-order extension**, and **Kobayashi's natural-number levels** — both
abandoned pure static levels for the same wall (soundness itself = Havender 1968). So the floor-domain
match is **GRADED**: **full** (memory safety on the affine floor — basis closed), **partial** (liveness on
the static-order floor — static fragment carried, per-thread + dynamic fragments escape), **none**
(information flow on the affine floor — every seam breaks). A floor can fail to carry a domain along (at
least) two axes — *granularity* and *static-vs-dynamic* — and the third domain fails on both. (Corrections
the panel forced: the right name is **lock-order-inversion freedom**, not "deadlock-freedom" — condvar /
channel / join cycles remain; and the claim that Rust's *affine* floor enables closure is overstated — the
order-carrying floor predates affinity by two decades in two non-affine settings, and affinity buys only
token non-duplication, which the re-entrant `root` shows it does not even buy.)

**…and the grade is not monotonically liftable — a sixteenth primitive proves it.** The obvious repair to
#15 is to *upgrade the floor*: a **runtime lock registry** (`lock_registry.rs`, lockdep-in-miniature) — a
`thread_local!` held-set of **runtime** ranks checked on each acquire. It directly targets #15's two gaps,
and the cold panel ruled **SEAM-BREAKS [horn 1], FAILED** (r22), the *fourth* broken seam — because the
upgrade is **non-monotone**. It *did* fix **coverage** (runtime-chosen-fixed ranks close — bank-transfer,
hand-over-hand) and *did* pay the predicted price (**#5 bare-metal** — needs OS thread-local, *cannot be
`no_std`*, hence feature-gated; **#6** a per-acquire runtime check; a **prevention→detection modality** —
the inversion compiles and is trapped only on *executed* paths, soundness now path-coverage-dependent). But
**granularity recurred one rung finer**: `HELD` is keyed on the OS *thread*, while the unit of control in
async is the *task*; a work-stealing migration resumes a rank-holding task on a fresh thread with an empty
`HELD`, laundering an **undetected inversion** (verified: the registry is thread-blind), and a
single-threaded executor *false-panics* on independent tasks (`!Send` closes neither direction). So the
**granularity axis is a containment-scope LADDER — token < thread < task** — and each ambient Rust floor
reaches exactly one rung (affine token #15, thread-local #16); there is no ambient *task-local*. **A
partial match is not lifted to full by upgrading the floor — the upgrade trades one gap for a finer one and
spends cost-basis axes (#5, #6) plus a modality downgrade doing it**, coupling the graded match to the
closed cost-basis. Prior art forecloses the static-rank design entirely: FreeBSD **WITNESS** / lockdep /
Abseil all abandoned static ranks for dynamically-*learned* acquires-before graphs (an even higher floor,
living wholly in the detection modality).

**…and the match-unit is the *precondition*, not the *domain* — a seventeenth primitive, breaking
breadth-wise, proves it.** Instead of climbing the ladder again, probe a *new precondition* in the *same*
liveness domain: **"this finalizer MUST run"** (a `MustRun<F: FnOnce()>` cleanup guard, `core`-only). The
cold panel ruled **SEAM-BREAKS [horn 1], FAILED** across **two** rounds (r23 + r24, breaking attackers
included), the *fifth* broken seam — and the break is compiler-settled: `mem::forget` is *safe*, so `Drop`
never fires and the finalizer runs zero times (the leakpocalypse — RFC 1066 / 1084). Three keeps. (a) The
grade is a clean **none** at the type level; the "partial-via-`#[must_use]`-lint" texture was **rejected**
(`scopeguard` ships that exact warning as a cosmetic advisory over a non-guarantee, not a floor fragment).
(b) So the *same* liveness domain grades deadlock-freedom **partial** (#15/#16) but must-run-cleanup
**none** — the floor-match unit is the **precondition, not the domain**, sharpening the Phase-M
domain-parameterized result. (c) The verdict mode is the **ninth, new one — *wrong-structural-logic /
out-of-frame***: every sound *decidable* fix collapses to a viral `Leak`/linear bound (the alternative,
heap-reachability, is undecidable ≅ halting), so the floor is the wrong *structural logic* (affine vs
linear), not the wrong *domain* — sharpened by prior art to Boats' *scoped-task trilemma*, of which Rust
deliberately rejected the linear horn. It is the **enforcement** face of the ninth axis (leak-freedom),
inverse to `drain` (#10, the *tolerance* face), so **N is unchanged**. A coda the two rounds taught: a seam
*designed to break* cannot be coherently dressed as usable — both FAILED on spec-coherence, not the break
(the disarm/commit-rollback mismatch, the fallible-finalizer scope), which is itself positive evidence for
the verdict.

The taxonomy is now a map of *what the missing conjunct is* **and which axis (or category) closing it spends**:
- **Fully floor-expressible** → seam closes free, every axis ✓ (SPSC).
- **Runtime-checkable** → closes by spending **zero-cost/#6** (slab; `RefCell`).
- **Cross-object identity via a generative brand** → closes by spending **API-locality/#5** (and
  #7 legibility), with a #5↔#6 *trade* (range-branding buys erasure by deepening the brand tax)
  (branded index; `indexing`).
- **In-bounds via a legible predicate** → closes brand-free + legible, spending **solver-freedom**
  (SMT in the TCB) and hitting the R2 trade again (brand-free ⟹ runtime check; zero-cost ⟹
  length-brand) (refined index; Flux/Liquid Haskell).
- **Dynamic shared ownership** → closes brand-free, spending **bare-metal** (a global-allocator
  refcount manager), with a bare-metal↔capacity trade (a pool-backed `Rc` is bare-metal but bounded)
  (shared; std `Rc` / `heapless`).
- **A transient broken invariant across a possible unwind** → closes by spending **unwind-safety** (the
  8th axis), with a *discharge menu*: abort (lose recoverability), placeholder-repair (pay API-surface),
  or poisoning (take-and-replace; `take_mut`↔`replace_with`).
- **A guard whose `Drop` is load-bearing, on a leak-permitting floor** → closes by spending
  **leak-freedom** (the 9th axis): pre-pessimize so a leaked guard loses data but never double-frees
  (drain-and-yield; `Vec::drain`). Distinct from unwind-safety (leak = `Drop` never runs).
- **A value whose *address* is load-bearing** → closes via `Pin`, but the cost is **inseparable**
  (immovability — non-uniform: stack-pin → API-locality, heap-pin → bare-metal). Move-stability is a
  real, distinct danger with **no clean coordinate** (self-ref cell; `Pin`). N stays 9 — a new category.
- **A value whose *bit pattern* must be valid** → closes via a set-membership check, projecting onto
  **zero-cost #6** for a single read, but **inseparable** for the zero-copy view (per-read #6 vs sealed
  ctor API-locality). Validity adds **no new axis** — the closed-basis proof (validity seam; `num_enum`).
- **An external physical axiom** → sound modulo a TCB `assume`, outside the axes (MMIO).
- **Whole-heap aliasing+liveness that escapes to the caller** → no sound+usable+concentrated
  decision exists; a trilemma horn genuinely drops (intrusive list).
- **A finalizer that *must run* on a leak-permitting floor** → no sound decision exists either, but for a
  *structural-logic* reason, not a domain one: a *safe* `mem::forget` skips the `Drop`, and no decidable fix
  buys it back (affine "at most once" cannot compel linear "exactly once"). The *inverse* of `drain` (which
  closed by tolerating a skipped `Drop`); the **enforcement** face of leak-freedom, N unchanged
  (must-run-cleanup guard; `scopeguard` / leakpocalypse).

**The honest meta-pattern, and the closed basis.** Every recovered axis turns out to be an
*already-named PL concern* — zero-cost abstraction, generativity, refinement-legibility, the
`core`/`alloc` split, exception-safety, leak-freedom. The phase did not find new physics; it found that
the trilemma's three horns silently *bundle a set of independently-known PL design axes*, unified by one
rubric: **what does the safe witness cost?** The cold panel showed **five** adjudication modes — it
**confirmed-but-deflated** three new dimensions to "real but pre-existing" (bare-metal, unwind-safety,
leak-freedom), **sharpened without extending** once (solver-freedom = horn 2's "keep" face), **rejected
outright** once (checker-tractability, confounded), ruled two **real but structurally inseparable**
(move-stability, validity — a recurring category), and — the capstone — certified a new danger class
**projects entirely onto the existing basis** (validity adds no tenth axis). That last mode is what
*closes* the basis: the cost-space is a small fixed set — nine clean axes + an inseparable category + two
outside-the-axes — that dangers project onto, with finitely many answers to "what does the witness cost?"
The full face lattice (axes, dissociation proofs, the five-mode meta-pattern, the closed-basis section)
is `REALIZABILITY.md`.

## The second movement: composite preconditions, the fourth-domain probes, and the builder's calibration

The first movement (seams 1–17) answered *"what does a safe witness cost?"* — the closed nine-axis basis, the
domain-parameterized trilemma, the graded floor-match. The next twenty seams (18–37) held those answers fixed and
pushed on three new fronts: **how a floor matches a *composite* precondition**, **whether a *fourth* soundness domain
exists**, and — the finding that only a stateful-builder-against-a-cold-gauntlet method could produce — **how the
builder itself is miscalibrated, and how that miscalibration is measured and corrected.**

**Liveness resolves into three coordinates, each with a lock face and a session face, and the packaging dial that
sets which horn a danger threatens is itself fenced.** The session family (#18 `session` through #32 `guarded`)
charted the liveness domain the way the first twelve seams charted memory safety. It has **three coordinates** —
completion, leak-freedom, and progress/deadlock-freedom — and each surfaced *twice*, once on a lock and once on a
session, breaking or costing the same way both times: progress broke on `ordered_locks` #15/#16 (the lock face) and
again on `priority` #26 (the session face), each time because *type-level order carries the local bound while
whole-program acyclicity escapes the per-edge floor*. Progress then **decomposed** (#24 `delegation`, #25
`multiparty`, #27 `projected`): a trusted constructor can carry the **topology** obligation (no cycle) but the
**execution** obligation (run-all, in-order, no-drop) never leaves the consumer — "schedulability by construction,
*given all roles execute*." Completion, long treated as an irreducible residue (#17/#18/#23), turned out
**concentrable** after all: #29 `must_complete` reversed its own prediction and #30 `must_scoped` built the fix — a
scoped-CPS combinator (`thread::scope`'s trick) turns a forgettable `Drop` bomb into an unforgeable *return-type*
obligation, defanging `mem::forget` — at the price of surrendering the free handle (and only **per-endpoint**: global
completion still escapes via peer divergence). Recursion (#31/#32) split cleanly: guardedness/productivity
**concentrates** (a sealed `Guarded` marker rejecting the non-contractive `µX.X` in Θ(type-size), the same machinery
as order), leaving **termination alone** as the one genuinely undecidable NONE. Across all of it, the earlier
*packaging-dial* finding (#19 `restore` owned-costs vs #20 `reseat` borrowed-forces) held as a **fence, not a free
choice**: which horn a danger threatens is a design decision only *within* the structural envelope set by whether
soundness can be decoupled from the danger.

**Four probes for a fourth soundness domain; four deflations by four different mechanisms; the three-domain map is
closed.** After memory safety, information flow, and liveness, the natural question was whether a *fourth* domain
exists. Four seams went looking, each in a different candidate domain, and **none opened one**: `crdt` #21
(distributed convergence) **folded** onto the application-correctness basis — replica divergence is the generic
unchecked-typeclass-law phenomenon (Rust checks the *signatures* of `Ord`/`Monoid`/semilattice, none of their *laws*),
undecidable by Rice, no new coordinate; `wcet` #22 (real-time/WCET) **broke** — an op-count is an unsound proxy for
cycles, WCET's own founding premise, "pick a lane"; `divisor` #33 (panic-freedom/totality) **folded** — a panic is
*defined behaviour*, a clean abort, never UB, so totality is a correctness property (SPARK's own division/overflow
split); and `sync_cell` #35 (data-race/concurrency) **reclassified into memory safety** — a data race *is* UB. Four
mechanisms — fold, break, fold, reclassify — and the same verdict each time. The map that survives is **three
soundness domains** (memory safety / information flow / liveness) plus an **application-correctness basis** that
absorbs the "semantic law" dangers (typeclass laws, totality, convergence) — the things a type floor checks the
*shape* of but never the *meaning* of.

**How a floor relates to its domain has exactly two shapes, not three — and the concentrating shape has a static and a
dynamic end.** The concurrency probe forced the sharpest structural correction of the second movement. The earlier
picture was an implicit *trichotomy*: information flow floor-closes-nothing, memory-aliasing concentrates, and
concurrency looked *floor-closed-special* (the `Send`/`Sync` auto-traits seemed to close data-race freedom outright).
`sync_cell` #35 collapsed it to a **dichotomy**: only information flow is special (the floor runs *no* non-interference
pass, so every IFC seam breaks by omission — `secret` #13, `labeled` #14); **memory-aliasing and data-race freedom are
the same shape** — the floor *propagates* the invariant structurally (borrowck for aliasing, the auto-traits for
`Send`/`Sync`) but does **not close** it; closure comes from **concentrating seams**. So the memory-safety domain
carries *two* concentrate-behind-a-seam coordinates, and the concentrating bucket itself has texture: a **static end**
(SPSC #1, the slab #4 — discharge is compiler-verified, total, composable) and a **dynamic end** (`SpinLock` #37,
`RefCell` — discharge is a runtime witness that leaves a compiler-invisible obligation, e.g. lock-ordering). This
sharpened, in turn, what an `unsafe impl` *is*: not "just trust," but a proof obligation that is either **discharged**
(met — `SpinLock`'s lock invariant plus the guard's `T: Sync` bound) or **false** (deleted — `SyncCell`'s
unconditional `Sync`). That discharged-vs-false distinction is exactly RustBelt's semantic type soundness (POPL 2018),
floor-relativized: a re-import inhabits the logical relation *iff* its invariant is provable.

**The adjudication vocabulary closed at nine verdict modes, and the broken-seam count reached ten.** Distinct from the
nine cost *axes*, the cold panel's *rulings* settled into **nine verdict modes** (SEAM-HOLDS [none]; SEAM-COSTS by
horn; SEAM-BREAKS by horn; the sound-but-deliverable-rejected *rejection* mode; the real-but-inseparable mode; the
projects-onto-basis mode; the tradeoff-survival; and so on) — a fixed taxonomy that absorbed every one of the last
twenty seams without a tenth ever appearing. Ten seams **broke** a horn (nine *discovered* under adversarial pressure;
one — `sync_cell` #35 — a *conceded folklore* strawman built as a probe vehicle), and each break, like each cost,
mapped back to a missing precondition conjunct rather than to bad luck.

**The genuinely new second-movement finding is about the *builder*, not the seams: a stateful builder has
domain-local blind spots that a cold gauntlet measures and a salvage loop corrects.** Eight times the panel ruled a
seam *sound* but **rejected its deliverable** — and in every case the refutation came from the builder's **own cited
lineage** (Kobayashi as counterexample where I filed him as support; `checked_div` omitted from my own prior-art;
`SpinMutex` omitted as the counterexample to "no concurrency seam exists"). The eight rejections share one shape: **I
flee the *middle* — a decidable partial cost — toward whichever *extreme* is cleaner**, whether *total concentration*
(#8/#24/#25/#27/#28), *irreducible/global* danger (#29/#30), or *undecidable* residue (#31/#33). That tic is not
static: it traces a **calibration arc**. Early, it decided horns (the eight rejections, #8–#31, were horn-deciding
over-claims). Then it fell *below* the horn-deciding threshold — #32 and #34 both *survived* with the tic present only
as a **marked tradeoff** (an overstated baseline, an over-general "folklore" claim), because the builder had learned to
pre-concede the uncarriable half up front. But the arc is **not monotonic**: on the *fresh* concurrency domain the tic
re-fired at **full strength** — #35 over-claimed "floor-closed," and #36, the salvage built to prove the correction,
**omitted a conjunct and broke the same way** — before receding again on #37 (whose only dings were the mild
overstated-parity/nil-novelty shape). The coda: **on novel ground a builder's tic fires repeatedly until the gauntlet
exhausts the domain's obligation set, then recedes** — concurrency took three seams (break, break, hold) to map an
`unsafe impl`'s full four-obligation set, where a familiar domain took one. This is the load-bearing argument for the
method: *coldness is the instrument that measures a warm builder's calibration*, and the measurement is domain-local.

**The salvage rhythm is the method's self-correcting loop — five settled lineages, one of them two-step.** Every
rejection and break **prescribes its own fix** (the panel names the missing conjunct), the stateful builder supplies
it, and adjudication confirms or re-refutes. Five such lineages settled: four **one-step** pairs (#26→#27 sealed the
public constructor; #29→#30 built the scoped combinator; #31→#32 added the `Guarded` marker; #33→#34 excluded the
overflow point) and one **two-step** chain (#35→#36→#37 — two steps precisely because #36's salvage *also* broke, on
the omitted guard bound, before #37 discharged all four obligations and held). The loop is the constructive analog of
the impossibility result: you cannot build a *seamless* language, but against a cold adversary you *can* converge a
*seam* to soundness, and the number of rounds it takes is a readout of how far the danger sits from the builder's
prior.

## Decontamination (the prompt's C1): did the obstruction move or vanish?

The seamful2 theorem predicted that when a seam fails to close, the obstruction *relocates*
to the re-import boundary rather than disappearing. Across the primitives:

- **SPSC — it vanished.** No surviving obstruction anywhere; the precondition factors
  through the witness, so a sound+usable+concentrated decision exists. (This is the
  `spsc_seam_closes` Lean instance.)
- **MMIO — it moved, and stayed decidable.** The obstruction relocated onto the re-import
  boundary (the foundation-author `Send` opt-in) and stayed inside floor trait resolution —
  SEAM-COSTS, no horn fully drops. Plus a TCB axiom that sits *outside* the trilemma.
- **Slab — it moved off the *static* witness onto a *runtime* one.** The obstruction could not
  be carried statically (a `Copy` key defeats the affine/NLL witness), but it *did* factor
  through a runtime witness (the generation+occupancy check) — so the seam closes, with the
  relocation showing up as the non-erasable runtime cost. (This is the
  `slab_runtime_closes_but_static_does_not` Lean pairing.)
- **Branded index — it moved onto the *type surface*, and stayed sound.** The obstruction (B3,
  cross-object identity) factors through a *generative brand* witness, so the seam closes
  (`branded_seam_closes`, like SPSC) — but the relocation shows up not in code crossings (O(1)) and
  not at run time (the brand erases), but as a **viral `'id` in every consumer type**. The
  obstruction went somewhere genuinely new: the API surface. This is what made "concentration"
  un-bundle into a code face and a type face.
- **Refined index — it moved into the *trusted toolchain*, and stayed sound + brand-free.** The
  obstruction (in-bounds) factors through a *legible predicate* witness, so the seam closes
  (`refined_seam_closes`, like SPSC) — brand-free and legible — but the relocation lands on neither
  code, type, nor (in principle) runtime: it lands on the **SMT solver in the trusted loop** (horn
  2's solver-freedom face). The obstruction moved *out of the program entirely* and into the
  toolchain's TCB. This is what closed the #5⟂#7 cell and un-bundled horn 2.
- **Shared (Rc) — it moved into a *runtime subsystem*, and stayed sound + brand-free.** The
  obstruction (the data-dependent last-drop) factors through a runtime refcount **manager**, so the
  seam closes (`shared_seam_closes`, like SPSC) — but the relocation lands neither in code, type, nor
  the toolchain: it lands on the **global allocator in the deployed binary** (the bare-metal axis).
  The obstruction moved into a runtime subsystem the no-heap target does not have.
- **Intrusive — it moved, and broke soundness.** The obstruction relocated to the boundary
  and what sits there is genuinely unsound (L2 UAF): horn 1 drops. Any repair walks into
  another horn (collapse lifetimes → drop usability; full separation logic → drop
  decidability). (This is the `intrusive_seam_does_not_close` Lean instance.)
- **Take-and-replace — it moved onto the *control-flow / unwind path*, and stayed sound.** The
  obstruction (the transient broken invariant) factors through an *abort-bomb* witness, so the seam
  closes (`take_seam_closes`, like SPSC) — but the relocation lands neither in code, type, runtime
  (happy path), toolchain, nor heap: it lands on the **unwinding behavior** — a mid-take panic becomes
  a process abort. The obstruction moved onto the exception-safety guarantee level (the 8th axis).
- **Drain-and-yield — it moved onto the *leak path*, and stayed sound.** The obstruction (a guard whose
  `Drop` is load-bearing) factors through *pre-pessimization*, so the seam closes (`drain_seam_closes`,
  like SPSC) — but the relocation lands neither in code, type, runtime (happy path), toolchain, heap,
  nor the unwind path: it lands on the **leak path** — `mem::forget` the guard and the un-yielded tail
  is silently lost. The obstruction moved onto leak-freedom (the 9th axis), distinct from unwind-safety.
- **Self-ref cell — it moved onto *immovability*, and stayed sound.** The obstruction (a value's address
  being load-bearing) factors through `Pin` over a `!Unpin` cell, so the seam closes (`selfref_seam_closes`,
  like SPSC) — but the relocation lands on the **immovability** of the value (the revoked move/return/store
  capability), and *which named axis that spends is non-uniform* (stack-pin → API-locality, `Box::pin` →
  bare-metal). So unlike the previous nine, the obstruction moved to a place that is **not a single clean
  axis** — move-stability is real but inseparable (N stays 9).
- **Validity — it moved onto an *already-occupied* place, and stayed sound.** The obstruction (a byte's
  bit-level validity) factors through a set-membership check, so the seam closes (`validity_seam_closes`,
  like SPSC) — and the relocation lands on a coordinate *already on the map*: **zero-cost #6** for a
  single read (the slab's place, a different predicate), or **API-locality** for the zero-copy view. The
  obstruction moved, stayed sound, and added *nothing new* — the **closed-basis** signature.

"Moves, doesn't vanish" is confirmed for the eleven non-trivially-encapsulable primitives — and
*where* it moves (decidable boundary bound / runtime witness / type surface / trusted toolchain /
runtime subsystem / `recursion_limit` fuel knob / unwind path / leak path / immovability / *an
already-occupied axis* / genuinely unsound) is the whole verdict; "vanishes" is the signature of the
fully-encapsulable one. The decontamination method generalized cleanly across all twelve — including the
five axis probes past seven: one moved to a place (the fuel knob) that was *not* a clean axis, two (the
unwind path, the leak path) that **were** clean distinct axes, one (immovability) to a **real danger with
no clean coordinate** (inseparable), and one (validity) to a place that was **already on the map** — the
last one closing the basis.

## What the harness added that the builder missed

The adversarial process earned its keep eighteen times:

1. **MMIO:** the builder framed M1/M4 as possibly "a different shape than the trilemma." The
   harness sharpened it to the exact right category — a TCB `assume` boundary, standard in
   verified-systems practice — and found the *actual* recordable price was a horn-3 `Send`
   opt-in the builder had under-weighted.
2. **Intrusive:** the builder predicted a horn-2/3 *cost* (token threads through code). The
   harness proved something worse — branding does not restore soundness at all, because a
   brand is an access capability, not a storage lifetime (L2 stays open). This corrected a
   real overclaim in the build-side framing ("the sound fix threads a token"): the token
   fixes L1/L4, not L2.
3. **Slab:** the first gauntlet FAILED [horn 1] on a genuine UB the builder's *spec* had —
   the §4 guard was written as generation-equality only, so a fresh `Slab::new()`'s vacant
   slot 0 (generation 0) authenticates a first-ever key `{0,0}` and `assume_init` runs on
   uninitialized memory. Crucially the bug was in the **spec, not the implementation**:
   `seam/src/slab.rs` checks `Entry::Occupied` before the generation compare, and a regression
   test confirms it rejects the exact exploit. This is the phase's build/break separation
   surfacing a real spec↔impl divergence; the revised spec (occupancy AND generation) then
   SURVIVED. The harness also named the canonical fix (slotmap's `version | 1` parity).
4. **Branded index:** the builder framed it as the clean primitive that *aces* all four axes
   (sound, decidable, concentrated, zero-cost) and pays only a fifth. The prior-art panel both
   confirmed the fifth axis with four decades of grounding (`runST` / Kiselyov–Shan / GhostCell)
   *and* corrected a zero-cost overclaim: single-index `try_index` relocates the bounds check per
   element (same cost as `slice[i]`), so the erasure win is *deferred* to the range-branding the
   spec scoped out. It also settled the sixth-axis probe — #5 (virality) and #7 (legibility)
   dissociate, because a refinement type is brand-free yet legible.
5. **Refined index:** the builder claimed the (brand-free, legible) corner with "zero-cost erasure
   with *no* brand." The panel found that claim to be the union of two incompatible configs (the
   **R2 trilemma**: brand-free + sound ⟹ a runtime check; zero-cost + sound ⟹ a length-brand),
   found an undisclosed expressiveness bug (R2 used `==` where safety needs `≤` — now fixed), and
   deflated the novelty of the horn-2 un-bundling (solver-in-TCB is DML/Liquid Types/Flux's founding
   tradeoff, the `unified` "keep" horn, not a new 7th axis). It nonetheless ruled the deliverable
   cleanly: **#5⟂#7 closes — yes**, the slab→illegible reclassification endorsed by two attackers.
6. **Shared (Rc):** the builder pitched bare-metal as a possibly-new seventh dimension. The panel
   confirmed it is a real, independent axis (the slab/`heapless` off-diagonal holds) **but deflated
   the novelty**: it is precisely Rust's `core`/`alloc` boundary (RFC 2480, stable 1.36), and
   `heapless::pool` already witnesses the bare-metal↔capacity trade — so "drop the undiscovered-
   dimension framing." That deflation generalized the honest meta-pattern: every recovered axis is a
   pre-existing named PL concern; the result is the unification, not the dimensions.
7. **Type-level index (the eighth probe):** the builder pitched **checker-tractability** as a possible
   eighth axis and built the off-diagonal to isolate it. The panel conceded soundness unanimously but
   **rejected the axis** — the sharpest intervention of the project. Three independent attackers
   converged: the off-diagonal silently flips **expressiveness** (the seam is static-index-only, a
   strict subset of its siblings — and the confound is intrinsic to a compile-time proof); the
   `recursion_limit` cost is an **encoding artifact** (binary type-level naturals are O(log N) and
   never hit it); and the axis is **mislabeled** (`Lt` is linear, *cheaper* than refined's QF-LIA — the
   real, pre-existing concern is terminating-resolution, Paterson / `UndecidableInstances`). The
   builder took it on the chin: N stays 7. This is the keep that proves the rest — a discovery
   procedure with no rejection mode discovers nothing.
8. **Take-and-replace (the second eighth-axis probe):** the builder pitched **unwind-safety** as the
   eighth axis, having designed the seam to avoid the type-level index's confound (it aces all seven
   prior axes, pays only U2). The panel **confirmed it (N ≥ 8)** and explicitly ruled the off-diagonal
   *clean* (contrasting it with the rejected typed_nat). Two sharp corrections, both recorded: (a) the
   builder **mislabeled the axis as "the abort cost"** — it is the *exception-safety-guarantee-level*,
   and abort is one **discharge** of it (`replace_with` repairs the gap recoverably, paying API-surface;
   the axis carries a discharge menu); (b) prior-art surfaced that **RFC #1508 rejected this exact
   `take_mut` from std** on the very abort cost scored — the "ace seven, pay one" framing was the
   *historically losing* argument. Net: a clean eighth axis, deflated to pre-existing — the discovery
   half of the instrument, complementing the rejection half from probe 7.
9. **Drain-and-yield (the ninth axis):** the builder pitched **leak-freedom**, having reverse-engineered
   the primitive from the off-diagonal requirement — a free-standing guard (no closure) so it would ace
   API-locality, the confound the obvious scoped-thread witness would have hit. The panel **confirmed it
   (N ≥ 9)**, ruled it **distinct from unwind-safety** (off-diagonal populated both ways: take is
   unwind-sensitive/leak-safe, drain is leak-sensitive/unwind-safe — refusing to merge the two
   destructor-execution siblings), and confirmed no confound (it aces API-locality *because* the guard
   owns its data). One sharp recorded nuance: leak-freedom is the **thinnest, most-deflatable** axis (its
   cost is invisible in non-leak code), yet still a genuinely populated off-diagonal — "deflatable" and
   "not an axis" are different verdicts (probe 7 was the latter, this the former).
10. **Self-ref cell (the inseparable verdict):** the builder pitched **move-stability** as a tenth axis,
    honestly flagging the confound risk. The panel returned the project's most novel ruling — **real but
    structurally inseparable** — and corrected three framings: (a) the cost is **immovability** (revoked
    move/return/store), heavier than the builder's claimed "API-locality alone"; (b) "no free-standing
    witness *can* exist" overstates it — RFC 2349's *rejected* `?Move` immovable types were a designed
    free-standing witness, so Rust's denial is *contingent*; (c) the crisp `core`-only reading is a
    **non-composable toy** (real intrusive nodes / futures force `unsafe` pin-projection or `Box::pin`).
    This is the keep that *extended the method itself*: a fourth verdict mode (beyond confirm / sharpen /
    reject) — a real danger that cannot be made a clean coordinate. N stays 9.
11. **Validity (the closed-basis capstone):** the builder pitched a new danger class (bit-level validity)
    *not* to hunt a tenth axis but to test closure, predicting it projects onto zero-cost #6. The panel
    earned its keep to the last: two of three gauntlets found real defects — a **range** check unsound for
    non-contiguous discriminants (the headline bug, four attackers converging), then a residual stale
    `b<=2` the builder left when fixing the first (a self-inflicted miss). The corrected seam survived,
    and the verdict refined the builder's prediction: validity is **(3) inseparable** (the zero-copy view
    splits #6 vs API-locality), a *second* inseparable danger — but the **meta-claim is unanimous: no
    tenth axis, the cost-basis is closed.** Non-novel (`num_enum`/`bytemuck`/`zerocopy`). The keep that
    *concluded the arc*: a brand-new danger projecting cleanly onto the basis is what closure looks like.
12. **Constant-time secret (the generalization):** the builder stepped outside memory safety to test
    whether closure is absolute, predicting information-flow is *out of frame*. The panel **vindicated the
    domain hypothesis and enriched it** — but **broke the seam** (SEAM-BREAKS, IFC sense): two attackers
    independently found that `ct_eq(a,b).expose_bool()` then `if b` leaks a secret relationship using only
    vended ops — *declassification is etiquette, not type-structure*, the intrusive-list horn-drop re-run
    in a new domain. And two corrections: the §4 universal "no source-level leak" is false; constant-time
    codegen is **dischargeable** (CompCert-CT/Jasmin/FaCT), *not* MMIO-shaped. The keep that *generalized
    the result*: the IFC domain has its own axis (**declassification discipline**, no memory-safety
    analog), the trilemma recurs sharper, and closure is **per domain**.
13. **Capability-gated declassification (the floor-domain lesson):** the builder probed whether the IFC
    domain has a *closing* seam, paying declassification discipline as a clean axis. The panel **broke it
    5/5** — and the breaks *deepened* the result: `map`/`zip` hands the closure the bare plaintext (an
    uncapped exit, **unpatchable without an effect system**), and the affine cap bounds *uses*, not *bits*
    (prior-art: it enforces Sabelfeld–Sands' *where/when*, not the *amount* — **quantitative info-flow**).
    What it enforces *projects onto the affine memory-safety axis*; the real IFC coordinate an affine floor
    **cannot** bound. So both IFC seams break, for one reason — **Rust's floor carries none of the IFC
    precondition.** The keep that *finished the generalization*: a closing seam exists only when the
    **floor matches the domain**; the affine floor closes memory-safety seams and *no* IFC seam.
14. **Type-level lock ordering (the floor-domain match is GRADED):** the builder probed the *positive*
    half — does a closing seam reappear in a *third* domain (**liveness**) once the floor (a type-level
    rank order) carries its precondition? The panel **broke it [horn 1], FAILED** (3/5), and the break
    grades the match into **full / partial / none**. Two failure dimensions, both "the floor under-carries
    the precondition": **granularity** — re-entrant `Floor::root` mints a second lineage, so safe code
    types a low-then-high inversion (verified compiles+runs); the discipline is per-token, the precondition
    per-thread-global, and Rust has no ambient per-thread one-shot resource (even the *static* fragment
    fails) — and **coverage** — the *dynamic*-acyclicity fragment (runtime/same-rank locks) escapes the
    *static* floor, as IFC's quantitative coordinate escaped the affine one. Prior art forces the
    partial-match ruling (Boyapati–Lee–Rinard OOPSLA'02 *minus* their dynamic extension; Kobayashi's
    natural-number levels; Havender 1968). The keep: name it **lock-order-inversion freedom**, not
    "deadlock-freedom"; the affine-floor framing is overstated (the order predates affinity 20y).
15. **Runtime lock registry (the floor-upgrade is non-monotone; granularity is a ladder):** the obvious
    repair to #14 is to *upgrade the floor* — a `thread_local!` registry of **runtime** ranks (lockdep-in-
    miniature) — to lift the partial match. The panel **broke it [horn 1], FAILED** (r22), the *fourth*
    broken seam, because the upgrade is **non-monotone**. It fixed **coverage** (runtime-chosen-fixed ranks
    close — bank-transfer, hand-over-hand) and paid the predicted price (**#5 bare-metal** — needs OS
    thread-local, *cannot be `no_std`*; **#6** a per-acquire check; a **prevention→detection modality** —
    soundness path-coverage-dependent). But **granularity recurred one rung finer**: `HELD` is keyed on the
    OS *thread*, while async *tasks* are the control unit — work-stealing migration launders an undetected
    inversion (verified: thread-blind), and a single-threaded executor false-panics (`!Send` closes
    neither). So the **granularity axis is a containment-scope LADDER — token < thread < task** — no ambient
    Rust floor tops out (there is no task-local). The keep: **a partial match is not lifted to full by
    upgrading the floor** — the upgrade trades one gap for a finer one and spends cost-basis axes (#5, #6)
    plus a modality downgrade, coupling the graded match to the closed cost-basis. Prior art forecloses
    static ranks entirely (FreeBSD WITNESS / lockdep / Abseil → learned acquires-before graphs).
16. **Must-run cleanup guard (breadth-wise — the match-unit is the *precondition*; a NEW mode,
    wrong-structural-logic):** rather than climb the ladder again, probe a new *precondition* — "this
    finalizer MUST run" — against the affine floor. The panel **broke it [horn 1], FAILED** across **two**
    rounds (r23 + r24, breaking attackers included), the *fifth* broken seam: `mem::forget` is *safe*, so
    `Drop` never fires and the finalizer runs zero times (leakpocalypse, RFC 1066/1084). The grade is a
    clean **none** at the type level — the "partial-via-`#[must_use]`-lint" texture was **rejected**
    (`scopeguard` ships that exact warning as a cosmetic advisory, not a floor fragment). The keeps: (a) the
    floor-match unit is the **precondition**, not the *domain* — the *same* liveness domain grades
    deadlock-freedom *partial* (#14/#15) but must-run-cleanup *none*; (b) it is the **enforcement** face of
    the ninth axis (leak-freedom), inverse to `drain` (#10, the *tolerance* face), so **N is unchanged**;
    (c) a **new method mode — *wrong-structural-logic* / out-of-frame**: every sound *decidable* fix
    collapses to a viral `Leak`/linear bound (the alternative, heap-reachability, is undecidable ≅ halting),
    so the floor is the wrong *structural logic* (affine "at most once" vs linear "exactly once"), not the
    wrong *domain* — sharpened to Boats' *scoped-task trilemma* (Rust rejected the linear horn). Both rounds
    FAILED on *spec-coherence*, not the break — a seam designed to break cannot be coherently dressed as
    usable, which is itself evidence for the verdict.
17. **Session-typed channel (the composite — same gap, different horn; the first out-of-domain HOLD):** a
    `Chan<P>` affine typestate handle, built as the crucible for #15 (typestate carries static **order**) and
    #17 (the floor cannot compel **completion**). The panel ruled **SEAM-COSTS [horn 3]** (TRADEOFF, **survived
    1/1**, r25) — the **first** out-of-(memory-safety) probe to *hold with a cost* rather than break. The floor
    matches this *composite* precondition **component-wise**, and the components are *orthogonal* (the
    soundness attacker confirmed `mem::forget` cannot shorten `'a` to start a second session, so completion
    cannot infect safety): protocol **order** *closes* via affine typestate (sound, decidable, concentrated;
    **no `root`-hole**, unlike #15), while **completion** is an undischargeable horn-3 *concentration* cost
    (the #17 linearity gap, recurring at every `?`/panic/early-return). The illumination: the *same*
    `mem::forget` gap that **broke** #17's horn 1 only **costs** here, because completion was #17's *whole*
    promise but is *one component beside a sound safety core* here — **which horn a danger threatens is set by
    how the rule packages it.** Three deflations recorded: it is a single-handle typestate **walk**, not a
    binary session (`core`/no-`unsafe` forecloses dual endpoints, `E0499`; the abandonment is *self*-
    abandonment); "usable" holds only for *infallible straight-line* protocols; and the split is documented
    **folklore** (Jespersen–Munksgaard–Larsen 2015; RFC 1066; ECOOP 2022 "Stay Safe Under Panic") — the
    contribution is *locating* it on the lattice, not discovering it.
18. **Value-owning scope-restore guard (the controlled-packaging probe — the dial is real but BOUNDED):** a
    `Restore<T, F: FnOnce(T)>` guard (the real `scopeguard::guard(value, f)` + `into_inner` — the value-owning
    shape #17's `MustRun` *declined*), built to turn the session finding ("which horn a danger threatens is set
    by how the rule packages it") from an **observation** across two primitives into a *deliberate* experiment:
    it carries #17's **identical** un-compellable-`Drop` danger (`mem::forget` skips the restore) wrapped in
    #18's structure (a sound affine-ownership core beside which the restore is a side-obligation). The panel
    ruled **SEAM-COSTS [horn 3]** (TRADEOFF, **survived 1/1**, r26) — the dial is **real**: the *same*
    `mem::forget` gap that **broke** #17's horn 1 only **costs** horn 3 here (an undischargeable "don't forget
    the guard" obligation; the restore strands a logical invariant, never UB, because memory safety never
    rested on the closure). Three attackers conceded the ownership core sound (no safe double-free/alias/
    double-consume; trivially decidable). But two ruled BREAKS, folded into recorded costs, **bounding the
    thesis** (all accepted): (a) the move is **confounded, not single-variable** — #19 also adds the
    value-ownership core #17 lacked, so the value-ownership model co-varies (a better-controlled probe than the
    incidental #17/#18 pair, not a clean manipulation); (b) the thesis is **bounded, not universal** — the horn
    is *forceable*: the same skippable-`Drop` danger in a **borrowed-slot / `take_mut`** packaging forces
    **horn-1 UB** (the #9 `take` danger), and `Pin`'s drop guarantee + the `thread::scope` restructure exhibit
    *un-repackageable* horns, so *which* horn depends on **where safety rests** — partly danger-intrinsic; (c)
    **not novel, no new mode** — `JoinGuard` broke horn 1 and `scopeguard` costs horn 3 already in 2015 (RFC
    1066, the leakpocalypse); the contribution is the *method* and *locating the dial*. The refined finding:
    **the packaging dial moves the horn only within a structural envelope set by whether soundness can be
    *decoupled* from the un-compellable closure** — owned value ⇒ a side-obligation that *costs*;
    borrowed-across-unwind / `Pin` / `thread::scope` ⇒ a danger that *forces* horn 1. Phase Q said the horn
    moves; Phase R says it moves *within a fence*, and names the fence.
19. **Take-and-reseat guard over a borrowed slot (forcing the horn — the clean single-variable experiment):** a
    `Reseat<'a, T>` guard (`take_mut` in guard form), the **controlled counterpart** to #19. It keeps #19's
    packaging **byte-identical** (a guard with `Deref`/`DerefMut`/`into_inner`/Drop-restore) and changes
    **only** the decoupling property — owned value → borrowed slot — repairing Phase R's confound (#17→#19
    co-varied packaging *and* ownership; #19→#20 co-varies *neither*). The panel ruled **SEAM-BREAKS [horn 1],
    FAILED** (r27), the **sixth** broken seam. The break: a 4-line **safe** witness — `let mut s =
    String::from("hello"); let mut g = Reseat::new(&mut s); g.push_str(" world"); g.into_inner()` — returns the
    owned `String` while the slot keeps the `ptr::read` duplicate (`E0509` forces the `mem::forget(self)` that
    suppresses the reseat), so both own overlapping heap and **double-free, no `unsafe` in user code**. Two
    findings: (a) **the horn flip is clean** — all five attackers agreed #19 (owned + `into_inner`) *cost*
    horn 3 and this (borrowed + `into_inner`) *breaks* horn 1 with byte-identical packaging, so the flip is
    attributable to the decoupling property **alone**; (b) **the fence is FORCED, not movable** — the spec's
    *movable* hedge (that the `into_inner`-less subset holds) was **rejected**: that subset is **degenerate** (it
    deletes the by-value move-out that *defines* `take_mut`, strictly dominated by plain `&mut T`), so every
    *usable* packaging breaks, and the fence sits at the **owned-vs-borrowed** line. Deflation (pre-registered,
    a flag not a kill): no novel mechanism — `take_mut` `ptr::read`/`write` + `scopeguard`'s `into_inner` defuse
    + RFC 1066's forget-skips-`Drop`, reassembled; the contribution is **analytical**. The trilogy closes: **the
    horn a danger threatens is movable iff the value-ownership model lets soundness *decouple* from the
    un-compellable closure** — owned can (⇒ cost), a borrowed slot cannot for any usable packaging (⇒ forced
    break). Phase Q: the horn moves. Phase R: within a fence (confounded). Phase S: the fence is real, FORCED,
    at owned-vs-borrowed.
20. **State-based CRDT merge (the fourth-domain probe that DEFLATED):** a `Replica<T: JoinSemilattice>` whose
    `merge` applies the semilattice join — the **first probe aimed at a fourth soundness domain**,
    distributed/replicated **convergence**, after memory safety (match), information flow (mismatch, the
    `secret`/`labeled` siblings), and liveness (partial). Like the IFC seams it has **no `unsafe`**; the danger
    is **replica divergence** (two replicas that saw the same updates disagree). The panel ruled **SEAM-BREAKS
    [floor-mismatch]** *floor-relatively* but scored a **TRADEOFF survival** (r28) — the break was
    pre-registered, and the dominant finding is the **deflation**. The supply-side break is real: `T:
    JoinSemilattice` is a **syntactic proxy, not a witness** — `merge` runs whatever `join` the impl supplies,
    the bound checks only the *signature*, so a safe non-commutative/non-idempotent `impl` type-checks and
    diverges replicas (closing it is **undecidable by Rice**). But the candidate fourth domain **collapsed**,
    three corrections accepted: (a) the **blessed-CRDT TCB rescue is NOT MMIO-shaped** — MMIO has finitely many
    foundation-owned registers, but the lattice space is **open-ended**, so the law-trust **recurs per
    user-defined CRDT** instead of amortizing into one axiom; (b) **'dischargeable' holds only at the model
    level** — Liquid-Haskell/Coq proofs are about a mathematical model, and no Rust pass checks
    model-to-shipped-code, so the trust is **irreducible at the compilation boundary** (subtler than `secret`'s
    constant-time, which CompCert-CT/Jasmin *do* preserve); (c) **the decisive deflation** — the gap is the
    *generic* unchecked-typeclass-law phenomenon (Rust checks the **signatures** of `Ord`/`Eq`/`Hash`/`Monoid`/
    semilattice, none of their **laws**), so convergence is **not a fourth soundness domain** — it **projects
    onto `restore`'s application-correctness basis** (#19). The finding: **no clean fourth domain here.** The
    soundness-domain map stays at **three** (memory safety / information flow / liveness) plus the
    **application-correctness** basis, into which convergence folds. Floor-relatively the supply-side break
    stands (the affine floor does not carry the semilattice laws — the IFC-shaped mismatch), but the
    *structural* result is a **deflation**, not new ground. N stays **9** modes; the FAILED broken-seam tally
    stays **six** (this one *survived*).
21. **Cost-typed computation (the second fourth-domain probe — FAILED):** a `Timed<C, T>` graded cost monad
    (reusing `typed_nat`'s sealed `Nat`/`Lt`) over **real-time / WCET** — the *second* probe aimed at a fourth
    soundness domain, after convergence deflated. A cleaner candidate (a cycle budget is a *quantitative
    resource bound*, not an algebraic law); no `unsafe`, the danger is a **deadline miss**. The panel ruled
    **SEAM-BREAKS [horn 1], FAILED** (r29) — the **seventh broken seam**, and unlike `crdt` it did **not**
    survive: my §4 re-import rule **over-claimed** (it promised `within` returns a value only on a met
    deadline, but the type tracks the **op-count**, not the time). All five attackers landed: (a) **soundness
    (fatal)** — `step` charges `+1` *decoupled from its closure*, so a million-iteration loop is cost `1` and
    `within::<1>` admits it; `C ≤ D` bounds the op-count, not the cycles; (b) **ergonomics — Horn 3 is FALSE**
    — non-unifying `if`-branches force manual step-padding, so the cost index is *heavy* bookkeeping; (c)
    **implementation** — the unary Peano cost is **O(C)**, uncompilable at real 10³–10⁷-cycle budgets, and the
    only compilable path (bundling work into one `step`) *is* the soundness break; (d) **expressiveness** — no
    type-level addition, so no compositional sequencing; (e) **prior-art** — this is **Danielsson's 2008 cost
    monad**, and *op-count ≠ cycles* is **WCET's founding premise, not a finding**; **pick a lane** (abstract
    cost xor physical deadline — the latter needs a defining machine the floor lacks). The finding: **WCET does
    not open a fourth domain either.** It **deflates** multiply — the cost monad onto Danielsson / `typed_nat`
    (#8, which does not even *scale*), the physical-deadline claim onto an `mmio`-shaped axiom (#2) the seam
    **never installs** (worse than MMIO — a fiction, and absent), the data-dependent-loop gap onto liveness's
    **partial-match**. The op-count is an **unsound** proxy — *sharper* than `secret`'s sound-but-incomplete
    one. **Meta:** two fourth-domain candidates probed (`crdt` deflated + *survived*; `wcet` deflated +
    *FAILED*), **neither opens a fourth domain** — the three confirmed domains (memory safety / information
    flow / liveness) look **special**. N stays **9** modes; the FAILED broken-seam tally rises to **seven**.
22. **Real binary session (the two-endpoint liveness DEEPENING — HELD):** `session::<P>()` vends two *dual*
    affine endpoints (`Chan<P>` / `Chan<P::Dual>`), **both live**, sharing the wire through
    `Rc<RefCell<Transport>>` (the `alloc` + interior mutability the duality costs). **Not** a fourth-domain bid
    — after both fourth-domain candidates deflated, this stays in the **third confirmed domain (liveness)** and
    *deepens* it, removing #18's most-cited limitation (one `&mut Transport` could not hold two
    concurrently-live endpoints at all — `E0499`). The panel ruled **SEAM-COSTS [horn 3], TRADEOFF**
    (r30) — it **HOLDS**, every attacker granting **no-UB** (the worst case is a starved, memory-safe handle).
    **#18's component-wise grading SURVIVES:** order **closes** (affine typestate), *per-session* relative
    progress **closes** (`HasDual` duality), decidability holds (`HasDual` projection Θ(depth), no SMT), and
    **completion costs horn 3** (the #17/#18 linearity gap — a peer's `mem::forget` starves its dual — now
    **peer-observable**). But the "real binary session" framing was substantially **deflated**, five corrections
    accepted: (a) **the sharpest** — relative progress closes only **per session**, because two interleaved
    `session()` calls **type-check a mutual-wait deadlock** (Kobayashi/Padovani); **global deadlock-freedom is
    out of reach of binary session types** — it needs multiparty/Kobayashi typing, and the spec scoped out
    exactly the case where duality's progress claim has teeth; (b) the **single-threaded** witness makes "peer
    waits forever" *self-abandonment-flavored* (one thread owns both handles) — only the **static**
    two-live-endpoint limitation is lifted, not `session_types` (2015)'s real two-thread concurrency; (c) the
    fallible `recv`'s `Err` is **non-actionable** — `mem::forget` **leaks the `Rc`** (skips `Drop`, strong count
    not decremented), so a starved dual cannot distinguish "not-yet" from "abandoned" (the **leakpocalypse
    double-bind**: the gap that costs completion *also* denies an actionable abandonment signal — an actionable
    one, like `mpsc`'s disconnect-`Err`, would need `Drop` to fire); (d) only **straight-line `i32` protocols**
    — no choice/`offer`, no **delegation** (channel-over-channel, *the* session-type feature), no recursion, so
    without delegation it is "#18 doubled"; (e) **citation fix** — ECOOP 2022 "Stay Safe Under Panic" is
    **Lagaillardie–Neykova–Yoshida**. The finding: the seam **HELD** and confirms #18's grading transfers, but
    the genuinely new content — once "real binary session" deflates to "#18 with a duplicated handle and a
    `RefCell` between the copies" — is **two sharpenings of the liveness map**: binary duality gives only
    **per-session** progress, and the leakpocalypse double-bind **deepens** the #17/#18 linearity gap. The probe
    earned its keep by **refuting its own headline** while confirming the grading. N stays **9** modes; liveness
    stays the **third** domain (its partial-match now bounded to *per-session*); the FAILED broken-seam tally
    stays **seven** (this one *survived*).

23. **Delegation — channel-over-channel (SOUND, but the DELIVERABLE was REJECTED):** the higher-order session
    step #23's straight-line `i32` witness lacked — `Chan<TxChan<Q, C>>::send_chan(c: Chan<Q>)` hands off a
    whole live session, `RxChan<Q, C>::recv_chan` receives it (built on #23's `Rc<RefCell<Transport>>` dual
    endpoints; `HasDual` recurses through the payload protocol `Q`). The r30 expressiveness attacker named
    delegation as *the* feature distinguishing session types from coupled typestate. The panel split: the
    horn-classifier read the **mechanism** as **SEAM-COSTS [horn 3]** (sound, decidable, no UB — the floor even
    carries *don't-use-after-delegate for free*, by the affine **move** that consumes the delegated handle), but
    the **judge ruled FAILED on the §6 DELIVERABLE** — the method's **rejection mode**, firing the **second
    time** (after `typed_nat` #8, also sound + rejected). My pre-registered claim — "delegation adds **no new
    coordinate**; its hazards project onto leak-freedom (#9/#10), a Phase-K closed-basis confirmation" — was
    **refuted from my own cited lineage**. Delegation's signature hazard — B `mem::forget`s the received
    `Chan<Q>`, **starving C** — is, in the exact words of the `session_types` crate I cited
    (Jespersen–Munksgaard–Larsen, WGP 2015), "**deadlock at the other end**": that is **progress /
    deadlock-freedom**, a coordinate *distinct* from completion (linearity) **and** leak-freedom. An entire
    literature exists *because delegation is the feature that breaks first-order binary sessions' automatic
    deadlock-freedom* (Coppo–Dezani–Yoshida–Padovani, MSCS 2016, "*the communication topology changes because of
    delegation*"; Kobayashi; deadlock-free GV — Lindley–Morris, Padovani, Multiparty GV PACMPL 2022, Rusty
    Variation 2019). **The category error:** I saw the `Rc` hazard's **leak-face** and filed it under #9/#10,
    missing that the signature danger is a **progress** phenomenon that merely *also* leaks under `Rc`; the
    "no new coordinate" conclusion was an **artifact of a single-threaded witness that definitionally cannot
    deadlock**. My one real defense — the symmetric two-session delegation **cycle is type-obstructed** (an
    inexpressible infinite type) — rescues *behavior*, not *classification*. Recorded tradeoffs: no mid-protocol
    abort verb (`Result<_, Self>` bans `?`, so the completion obligation is undischargeable by the *programmer*);
    `HasDual` well-formedness is **Θ(depth²)** not the spec's Θ(depth) (the involution normalizes
    `dual(dual(carrier))`); the finite-type, `!Send` single-thread witness cannot express the recursive
    accept-loop server (delegation's canonical use case). The finding: the probe **earned its keep by being
    instructively *wrong*** — I predicted a closed-basis confirmation, and the method refuted it with my own
    citations, establishing **progress / deadlock-freedom as a third liveness coordinate** (the lock seams
    #15/#16 were its *lock* face; delegation its *session* face), distinct from completion and leak-freedom. The
    seam is **sound**, so the FAILED broken-seam tally **stays seven** (a deliverable rejection, not a soundness
    break); N stays **9** (rejection is one of the nine modes — its 2nd firing); Lean
    `delegation_handoff_core_closes` (the sound order + affine-handoff core *closes*, like `typed_nat`).

## Artifacts

- Specs (attacked as text): `SEAM_SPEC_{spsc,mmio,intrusive,slab,branded,refined,shared,typed_nat,take,drain,selfref,validity,secret,labeled,ordered_locks,lock_registry,linear,session,restore,reseat,crdt,wcet,binary_session,delegation}.md`
- Cold verdicts: `REPORT_SEAM_SPEC_{spsc,mmio,intrusive,slab,branded,refined,shared,typed_nat,take,drain,selfref,validity,secret,labeled,ordered_locks,lock_registry,linear,session,restore,reseat,crdt,wcet,binary_session,delegation}.md` (+ raw transcripts in `runs_seam/`)
- Real Rust (rustc-checked, `#![no_std]`): `../seam/` — `src/lib.rs` (SPSC), `src/mmio.rs`,
  `src/intrusive.rs`, `src/slab.rs`, `src/branded.rs`, `src/refined.rs`, `src/shared.rs` (the lone
  `alloc` user), `src/typed_nat.rs` (eighth-axis probe 1 — sound, no new axis), `src/take.rs`
  (eighth-axis probe 2 — the confirmed unwind-safety axis), `src/drain.rs` (the confirmed leak-freedom
  axis), `src/selfref.rs` (move-stability — real but inseparable), `src/validity.rs` (the closed-basis
  capstone — a new danger projects onto the basis), `src/secret.rs` + `src/labeled.rs` (the two IFC-domain
  seams — *both break*; closure is per-domain, and no IFC seam closes on Rust's affine floor),
  `src/ordered_locks.rs` (the third domain — liveness; *breaks*, grading the floor-domain match
  full/partial/none), `src/lock_registry.rs` (the floor-UPGRADE probe — feature `lockdep`, needs `std`;
  *breaks*, the upgrade is non-monotone — granularity recurs token→thread→task), `src/linear.rs` (the
  breadth-wise *linearity* probe — `core`-only; *breaks*, a clean **none** grade and a new
  *wrong-structural-logic* mode — affine vs linear), `src/session.rs` (the *composite-precondition* probe —
  `core`-only, no `unsafe`; *holds with a cost* — SEAM-COSTS [horn 3]: order-safety closes, completion costs;
  the same `mem::forget` gap that broke #17's horn 1 only costs here), `src/restore.rs` (the
  *controlled-packaging* probe — `core`-only, no `unsafe`; *holds with a cost* — SEAM-COSTS [horn 3]: a
  value-owning scope-restore guard carrying #17's identical danger in #18's structure; the dial is real but
  **bounded** — the move is confounded, the horn is forceable in a borrowed-slot/`take_mut` packaging, and
  it restages the 2015 leakpocalypse), `src/reseat.rs` (the *forcing-the-horn* probe — `core`-only,
  `take_mut` in guard form over a **borrowed slot**; *breaks* — SEAM-BREAKS [horn 1], the sixth broken seam:
  the controlled counterpart to `restore` keeps the packaging byte-identical and flips only owned→borrowed,
  so `into_inner` becomes a safe-code double-free; the fence is **FORCED**, not movable, confirming the
  decoupling envelope under a clean single variable), `src/crdt.rs` (the *fourth-domain* probe — `core`-only,
  no `unsafe`; a `Replica<T: JoinSemilattice>` over distributed/replicated **convergence**; *breaks
  floor-relatively* — SEAM-BREAKS [floor-mismatch], a TRADEOFF survival — but the candidate fourth domain
  **DEFLATED**: the trait bound checks only the signature so a safe lawless `join` diverges replicas, yet the
  gap is the *generic* unchecked-typeclass-law phenomenon, so convergence projects onto `restore`'s
  application-correctness basis, not a new domain), `src/wcet.rs` (the *second fourth-domain* probe —
  `core`-only, no `unsafe`; a `Timed<C, T>` cost monad over **real-time/WCET**; *breaks and FAILED* —
  SEAM-BREAKS [horn 1], the **seventh broken seam**: `step` charges one unit decoupled from its closure's
  work, so `within::<1>` admits a million-cycle computation — the op-count is an **unsound** proxy for real
  time; it is Danielsson's 2008 cost monad and deflates to `typed_nat` #8 + an uninstalled `mmio`-shaped
  axiom + liveness's partial-match — the second fourth-domain candidate that fails to open a fourth domain);
  consumer code is `#![forbid(unsafe_code)]`; laundering paths are `compile_fail`
  doctests (90 `tests/` integration tests + 53 doctests = 143 total in the default `no_std` build; with
  `--features lockdep` the runtime registry adds its tests for **150 total**).
- Seam-width instrument: `SEAM_WIDTH.md`; full face lattice: `REALIZABILITY.md`
- Mechanization: `../proof/SeamGap.lean` — `seam_closes_iff_factors` + the
  `spsc`/`intrusive`/`slab`/`branded`/`refined`/`shared`/`typed_nat`/`take`/`drain`/`selfref`/`validity` instances,
  plus `secret_typed_discipline_factors`, `labeled_typed_discipline_factors`,
  `ordered_locks_typed_discipline_factors`, `lock_registry_typed_discipline_factors`,
  `linear_typed_discipline_factors`, and `reseat_typed_discipline_factors` (the two IFC seams' + the three
  liveness seams' + the borrowed-slot reseat's trivial sub-facts — all six break), and
  `crdt_typed_discipline_factors` (the fourth-domain probe's trivial sub-fact — it breaks *floor-relatively*
  but DEFLATED to a TRADEOFF survival, projecting onto `restore`'s application-correctness basis, so it is
  **not** one of the FAILED breaks), and `wcet_typed_discipline_factors` (the *second* fourth-domain probe —
  it breaks **and FAILED**, the **seventh** broken seam: the op-count is an unsound proxy for real cycles;
  Danielsson's 2008 cost monad, deflating to `typed_nat` #8 + an uninstalled `mmio`-shaped axiom + liveness's
  partial-match — so the `..._factors` family is now eight, of which **seven** are FAILED breaks and `crdt` is
  the lone TRADEOFF-survival deflation), plus
  `session_order_safety_closes` (the eighteenth seam *holds* with a horn-3 cost — its
  order-safety component closes like a closing seam; the completion cost is modeling-dictionary) and
  `restore_ownership_core_closes` (the nineteenth seam *holds* with a horn-3 cost — its affine-ownership core
  closes; the restore cost is modeling-dictionary, the same #17 gap that only *costs* once soundness is
  decoupled from the closure; its forced counterpart `reseat_typed_discipline_factors` *breaks* once the
  value is borrowed) and `binary_session_progress_core_closes` (the twenty-third seam *holds* with a horn-3
  cost — its order + *per-session* relative-progress core closes via affine typestate + the `HasDual`
  duality; the completion cost is modeling-dictionary, now peer-observable across two real endpoints, and the
  per-session-only deadlock-freedom and the non-actionable `Err` from the leaked `Rc` live in the prose) and
  `delegation_handoff_core_closes` (the twenty-fourth seam — channel-over-channel — is **sound** and its order +
  *don't-use-after-delegate* core closes via affine typestate + the affine move, exactly like `typed_nat`; but
  its §6 **deliverable was REJECTED** (the rejection mode's 2nd firing): delegation opens **progress /
  deadlock-freedom** as a third liveness coordinate distinct from completion and leak-freedom — the prose
  records the refutation, the kernel sees only that the handoff core suffices)
  (propext-only).

## The one-sentence upgrade to the trilemma

The Seam Trilemma says *a* seamless language is impossible; the constructive phase says
**which** seams you can build losslessly anyway — exactly the ones whose dangerous precondition
is fully encapsulable (SPSC-shaped) — while the rest force a predictable, measurable price set by
*what the missing conjunct is*: a runtime witness that spends zero-cost (slab), a viral generative
brand that spends API-locality (branded index), a legible predicate that spends solver-freedom
(refined index), a refcount manager that spends bare-metal (shared), an external TCB axiom (MMIO), or
a genuine horn drop (intrusive), an immovability that has no clean coordinate at all (self-ref cell), or
a bit-validity that projects onto a coordinate *already on the map* (validity) — and the trilemma itself
turns out to be a **3-of-N projection of a *closed* object (N ≥ 9)**: its three horns a shadow of **nine
clean axes** (zero-cost #6, API-locality, AI-legibility #7, bare-metal #5, unwind-safety, leak-freedom,
plus the original sound/decidable/concentration), an **inseparable category** (move-stability, validity),
and **two outside-the-axes** (the MMIO TCB axiom, the intrusive horn-drop) — each surfacing only when a
primitive becomes the witness that two previously-identical configurations differ (and one candidate,
checker-tractability, was **rejected** as confounded; the basis shown **closed** when a brand-new danger
class projected cleanly onto it — the method discriminates in nine modes). Honest coda: those axes are
each a *pre-existing* named PL boundary; the result is their unification under one "what does the witness
cost?" rubric — a question with **finitely many answers** — not their discovery. **And two more
generalizations**: that closed object is the **memory-safety** domain's; a constant-time secret showed
information-flow is a *different* domain with its *own* basis, so the Seam Trilemma is
**domain-parameterized** — the same shape, a different basis, in every soundness domain; and a
capability-gated declassification showed that *which seams close* depends on the **floor-domain match** —
Rust's affine floor closes memory-safety seams but *no* information-flow seam (it bounds resource *uses*,
not information *bits*; the IFC coordinate needs a quantitative/effect floor — Jif, FlowCaml, QIF); and a
third domain — type-level lock ordering (**liveness**) — showed that match is **graded, not binary**: it
breaks two ways (a per-token-vs-per-thread *granularity* gap, and a *static-vs-dynamic* coverage gap, the
dynamic-acyclicity coordinate escaping the static floor as IFC's quantitative one escaped the affine
floor), so liveness on a static-order floor is a **partial** match; and a sixteenth — a *runtime* lock
registry on an **upgraded** floor — showed that grade is **not monotonically liftable**: upgrading the
floor fixed coverage but **granularity recurred one rung finer** (thread-keyed vs task-keyed control), so
the granularity axis is a **containment-scope ladder** (token < thread < task) no ambient floor tops out,
and the lift was *bought* with cost-basis axes (#5, #6) plus a prevention→detection modality; and a
seventeenth — a *must-run cleanup guard*, probing **breadth-wise** — showed the floor-match unit is the
**precondition, not the domain** (the *same* liveness domain grades deadlock-freedom *partial* but
must-run-cleanup *none*) and that a floor can be the wrong *structural logic* entirely (affine "at most
once" vs linear "exactly once": a *safe* `mem::forget` skips the finalizer, no decidable fix buys it back —
the ninth, *wrong-structural-logic* mode); and an eighteenth — a *session-typed channel* — was the
**first** out-of-(memory-safety) probe to *hold with a cost* (SEAM-COSTS [horn 3]), showing the floor
matches a *composite* precondition **component-wise** (protocol order *closes* via affine typestate;
completion is a horn-3 *concentration* cost — the #17 linearity gap), and that the *same* `mem::forget` gap
that **broke** #17's horn 1 only **costs** here, because completion is one component beside a sound safety
core: *which horn a danger threatens is set by how the rule packages it*. So the final shape of the result:
**the Seam Trilemma is a domain-general tension; each soundness domain has its own cost-basis; a floor
closes a domain's seams only to the degree it carries that *precondition* — a graded match, full (memory
safety) / partial-to-none (liveness, by precondition) / none (information flow) — that grade cannot be
freely lifted (upgrading the floor trades one gap for a finer one and spends the closed cost-basis), the
floor can fail a precondition not only by domain but by *structural logic* (affine vs linear), and a single
*composite* precondition can be matched *component-wise* — one horn closing while another costs, with which
horn a danger threatens set by how the re-import rule packages it — yet that packaging dial is itself
*bounded*: it moves the horn only within a structural envelope set by whether soundness can be *decoupled*
from the danger (owned ⇒ the danger costs; borrowed-across-unwind / address-pinned / borrow-scoped ⇒ it
forces a fixed horn), so "which horn" is a design choice **within** a danger-intrinsic fence, not freely.**

**The completed arc (37 seams), in one more sentence.** Everything above was settled by seam ~18; the remaining twenty
close the account without adding a horn, an axis, or a domain — **the map is finite and it is now full.** Liveness
turned out to have exactly **three coordinates** (completion, leak-freedom, progress), each appearing on both a lock
face and a session face, with completion *concentrable* after all (a scoped-CPS combinator, per-endpoint) and only
*termination* left as an undecidable NONE; **four probes for a fourth soundness domain all deflated** — convergence
and totality *fold* onto the application-correctness basis, real-time *breaks* on the cost/cycles gap, and data-race
freedom *reclassifies* into memory safety — so the domain count is **three, closed**; the floor-domain relation
sharpened from a trichotomy to a **dichotomy** (information flow is the lone *floor-closes-nothing* domain, while
memory-aliasing and data-race freedom are the *same* floor-propagates-but-concentrating shape, with a **static** end
(SPSC, the slab) and a **dynamic** end (`SpinLock`, `RefCell`), and an `unsafe impl` revealed as a proof obligation
that is either *discharged* or *false* — RustBelt semantic typing, floor-relativized); and the adjudication vocabulary
closed at **nine verdict modes** over a **ten**-seam broken tally. But the second movement's own discovery is
reflexive and could only come from *this* method — a warm, stateful builder red-teamed by a cold, stateless gauntlet:
**the builder has a measurable, domain-local miscalibration** — a tic to *flee the decidable middle toward the cleaner
extreme* (total / global / undecidable), which the gauntlet caught **eight** times as sound-but-rejected deliverables,
each refuted from the builder's *own* citations, and which traces a **non-monotonic calibration arc** (horn-deciding
early, a marked-tradeoff-below-threshold as the builder learned to pre-concede the uncarriable half, then re-firing at
*full* strength on the fresh concurrency domain before receding) — so that **coldness is not merely a demolition tool
but a calibration instrument**, and the **five settled salvage lineages** (four one-step, one two-step because the
salvage itself first broke) are the constructive dual of the impossibility result: you cannot build a *seamless*
language, but against a cold adversary a single *seam* can be **converged** to soundness, and the round-count of that
convergence *measures how far the danger sat from what the builder already knew.***
