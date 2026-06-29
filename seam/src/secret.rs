//! # Thirteenth seam — a constant-time secret (the *different-domain* probe).
//!
//! *(Project ordinal: the thirteenth distinct primitive, and the first probe deliberately aimed at a
//! **different soundness domain** than the previous twelve. Those were all about **memory safety** — can
//! safe code reach UB? This one is about **information flow / non-interference** — can safe code leak a
//! secret through a **timing side-channel**? The cost-basis was shown *closed for memory safety*; this
//! asks whether that closure is absolute or domain-relative.)*
//!
//! Same floor, same rules — with one twist that is the whole point. This seam re-exports a danger that
//! is **not undefined behaviour**: branching or early-exiting on a secret value leaks it. The canonical
//! example is comparison: a naive `a == b` over byte arrays *short-circuits at the first mismatch*, so
//! its **running time** reveals the length of the matching prefix — a byte-by-byte key-recovery attack.
//! There is no UB here; memory is never corrupted. The "unsafe primitive" is **data-dependent control
//! flow over a secret**, and the seam re-exports it as a checked safe interface that only ever does
//! **constant-time** work.
//!
//! ## The witness — branchless operations (the `subtle`-crate pattern)
//!
//! [`Secret`] holds bytes whose *content* must not influence timing. Its private field and its **absent**
//! `PartialEq`/`Eq`/`Ord`/`Deref`/`Index` impls mean safe code **cannot** branch on it (`if a == b`,
//! `arr[secret]`, `if secret_bool` all fail to compile — see CF1). The only comparison vended is
//! [`Secret::ct_eq`], which XOR-folds *all* bytes with no early exit and returns a [`Choice`] — a
//! constant-time boolean (a `0`/`1` `u8`, never branched on). A `Choice` is consumed by the
//! constant-time [`Secret::conditional_select`] (a branchless masked merge), or finally exposed by the
//! explicitly-named [`Choice::expose_bool`] — the *one* place the secret-derived bit re-enters ordinary
//! control flow, used once at the end (e.g. "accept or reject the MAC").
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_secret): the seam BREAKS in the IFC sense
//!
//! The gauntlet ruled **SEAM-BREAKS [horn 1]** — the information-flow analog of the intrusive list's
//! memory-safety horn-drop. The attack used only the vended ops, no `unsafe`:
//! `secretA.ct_eq(&secretB).expose_bool()` then `if b { … }` **type-checks** and leaks a secret
//! *relationship* (key reuse / equality) through ordinary source-level control flow. The type forecloses
//! branching *on `Secret`*, but [`Choice::expose_bool`] returns an **untainted `bool`** the consumer may
//! then branch on — so "expose once, at the end, on an intended observable" is **etiquette, not
//! type-structure**. Closing it requires propagating a secret label through arbitrary intervening code —
//! a **whole-program, undecidable** non-interference pass (a full IFC type system: Jif / FlowCaml). So a
//! *concentrated, decidable* seam **cannot** soundly enforce non-interference: Horn 2 ("decidable &
//! proof-free") passes **only because the strong property is never actually checked** — the typed
//! discipline (privacy + absent traits) is a *syntactic proxy* for a property it does not discharge.
//! **This is the Seam Trilemma recurring in the information-flow domain, with a sharper bite.** (The
//! `ct_eq`/`select` interface is sound for its narrow `subtle`-style compare-and-select fragment; it is
//! the *general* non-interference claim, and the §4 "no source-level leak can be written" universal, that
//! break.)
//!
//! ## Two builder over-claims the panel corrected
//!
//! 1. **There is no `unsafe`, and the AM models no time** — true, and load-bearing: the project's usual
//!    "can safe code reach UB?" is *trivially* no (the UB slot is empty), yet the seam still *breaks* in
//!    the only sense that matters here. That is *why* information-flow is a **different domain**.
//! 2. **Constant-time-ness is NOT an MMIO-shaped (irreducible) TCB axiom — it is *dischargeable*.** An
//!    earlier draft equated the trust that the optimizer keeps `ct_eq` branchless with MMIO's physical
//!    axiom; the prior-art attacker correctly falsified that. MMIO's trust is irreducible *in principle*;
//!    constant-time codegen has been **formally preserved** by CompCert-CT, Jasmin, and FaCT. So the
//!    codegen assumption is *contingent* (an artifact of building on stock `rustc`/LLVM), not a permanent
//!    axiom — a different category than MMIO's.
//!
//! ## The §6 domain ruling — vindicated and enriched: the closed basis is PER DOMAIN
//!
//! The panel **vindicated hypothesis (2)**: information-flow is **out of frame** — a *different soundness
//! domain* (non-interference, not UB) with its **own basis**. So the memory-safety result "the cost-basis
//! is closed" refines to **closed *per domain***. And the IFC domain's basis is genuinely *different*:
//! two attackers independently surfaced **declassification discipline** (which / where / how-often a
//! `Choice` may exit — exactly the hole that broke this seam) as a real IFC-domain axis with **no
//! memory-safety analog**, alongside termination channels and concurrency channels. The build-and-break
//! *method* transfers — its very first IFC seam reproduced the trilemma and surfaced a new axis — but the
//! *coordinates* are the new domain's, not the nine. **The Seam Trilemma and the closed cost-basis are
//! domain-parameterized; memory safety is one domain, information flow is another.**
//!
//! ### CF1 — safe code cannot branch on a secret (no `PartialEq`, no `bool`, no `Index`):
//! ```compile_fail
//! # use seam_spsc::secret::Secret;
//! let a = Secret::new([1u8, 2, 3, 4]);
//! let b = Secret::new([1u8, 2, 3, 5]);
//! if a == b {            // ERROR: `Secret` does not implement `PartialEq` — no data-dependent branch
//!     let _ = ();
//! }
//! ```

/// A constant-time boolean: `1` for true, `0` for false, carried as a `u8` that is **never branched on**
/// until [`Choice::expose_bool`]. Combine/consume it only through constant-time operations.
#[derive(Clone, Copy)]
pub struct Choice(u8);

impl Choice {
    /// The **one** sanctioned exit from constant-time land back into ordinary control flow: turn the
    /// secret-derived bit into a `bool`. Named loudly because *using* the returned `bool` in an `if` is
    /// where a timing dependence is (deliberately) re-introduced — sound only when done once, at the end,
    /// on a result that is *meant* to be observable (accept/reject), never mid-computation on the secret.
    #[inline]
    pub fn expose_bool(self) -> bool {
        self.0 == 1
    }
}

/// A secret byte string whose *content* must not influence running time. Constructed from bytes;
/// compared and selected only in constant time. It deliberately does **not** implement `PartialEq`,
/// `Eq`, `Ord`, `Deref`, or `Index`, and keeps its bytes private — so safe code has *no way* to write a
/// data-dependent branch over it.
pub struct Secret<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> Secret<N> {
    /// Wrap secret bytes (e.g. a key, a MAC, a password hash).
    pub fn new(bytes: [u8; N]) -> Self {
        Secret { bytes }
    }

    /// Constant-time equality: XOR-fold **every** byte (no early exit) and return a [`Choice`]. Running
    /// time depends only on `N`, never on the contents — the whole point.
    #[inline]
    pub fn ct_eq(&self, other: &Self) -> Choice {
        let mut diff = 0u8;
        let mut i = 0;
        while i < N {
            diff |= self.bytes[i] ^ other.bytes[i];
            i += 1;
        }
        // Branchless: nonzero = MSB of (diff | -diff) = 1 iff diff != 0; Choice = 1 - nonzero.
        let nonzero = ((diff | diff.wrapping_neg()) >> 7) & 1;
        Choice(1 - nonzero)
    }

    /// Constant-time select: returns a copy of `a` if `choice` is true, else `b` — by a branchless masked
    /// merge, so which one is chosen leaves no timing trace.
    #[inline]
    pub fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mask = 0u8.wrapping_sub(choice.0); // 0xFF if choice==1, 0x00 if choice==0
        let mut out = [0u8; N];
        let mut i = 0;
        while i < N {
            out[i] = b.bytes[i] ^ (mask & (a.bytes[i] ^ b.bytes[i]));
            i += 1;
        }
        Secret { bytes: out }
    }
}
