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
//! ## The honest twist — there is NO `unsafe`, and the guarantee is a TCB axiom
//!
//! Two things make this seam different from the previous twelve, and both are load-bearing for the §6
//! ruling:
//! 1. **No `unsafe` keyword anywhere.** The foundation is ordinary safe arithmetic. The "danger" lives
//!    entirely in a *non-functional* property (wall-clock time), which the Rust Abstract Machine does not
//!    model at all. So the project's usual question — "can safe code reach AM-level UB?" — is *trivially*
//!    answered (no), and yet the seam can still be *broken* in the only sense that matters here.
//! 2. **Constant-time-ness is not guaranteed by the floor.** `rustc`/LLVM may compile the branchless
//!    `ct_eq` *back into* a branch (e.g. via a peephole or a `cmov`-to-branch lowering on some target).
//!    So the seam's soundness rests on a **trusted assumption the compiler does not discharge** — exactly
//!    the shape of MMIO's TCB axiom, but for a security property instead of a hardware fact. Real
//!    constant-time code needs `core::hint::black_box`, assembly, or tooling (`dudect`, `ctgrind`) to
//!    *audit* it; the type system only forecloses the *obvious* leaks (branching in source).
//!
//! ## What §6 must rule
//!
//! Is information-flow a **tenth axis** of the (closed) memory-safety basis, **out of frame** (a
//! different soundness domain, with its own basis — refining "closed" to "closed *per domain*"), or does
//! its cost **project onto** the nine (e.g. the lost data-dependent control flow is an *expressiveness*
//! cost; the trust-the-codegen part is a *TCB axiom* like MMIO)? The builder's hypothesis: **out of
//! frame** — the basis is closed *within memory safety*, and IFC is an orthogonal domain the same method
//! would re-run to find a *different* basis (whose first members look like "constant-time discipline" and
//! "a no-data-dependent-branch TCB axiom").
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
