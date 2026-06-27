//! # Third seam — an intrusive doubly-linked list (Phase C contrast, where the seam does NOT close)
//!
//! Same floor, same rules. The point: the affine-handle re-import that closed the SPSC seam
//! is **structurally inapplicable** here, because intrusive nodes *alias each other* (`a.next
//! → b`, `b.prev → a`) while the caller still owns them — and affinity *is* the absence of
//! aliasing. The sound mechanism is `GhostCell` **branding**: a `GhostToken<'brand>` that must
//! be presented at **every** access. So the obligation does not stay concentrated in a
//! foundation `split`; it threads through ordinary consumer code at every traversal/mutation
//! step. That is a measurable horn-3 drop (seam width O(accesses), not O(lists)) and a horn-2
//! cost (the consumer carries a token its logic must route correctly).
//!
//! This module is the minimal faithful core of `ghost-cell` (Yanovski et al., POPL'21),
//! reduced to exactly what the contrast needs. The `unsafe` is, again, concentrated in the
//! foundation (two blocks); but unlike SPSC the *token* leaks into the caller.
//!
//! L4 (no aliasing `&mut`) is enforced by the *floor*, via token uniqueness — two
//! simultaneous mutable accesses to the aliased node graph do not compile:
//! ```compile_fail
//! # use seam_spsc::intrusive::{GhostToken, Node};
//! GhostToken::new(|mut token| {
//!     let arena = vec![Node::new(1), Node::new(2)];
//!     let (a, b) = (&arena[0], &arena[1]);
//!     let ma = a.next.borrow_mut(&mut token);
//!     let mb = b.next.borrow_mut(&mut token); // ERROR: token already uniquely borrowed
//!     let _ = (ma, mb);
//! });
//! ```

use core::cell::UnsafeCell;
use core::marker::PhantomData;

/// A brand: an *invariant* lifetime, unique per `GhostToken::new` scope. Possessing the token
/// is the right to access every cell of the same brand; `&mut` of it is exclusive access.
pub struct GhostToken<'brand> {
    _invariant: PhantomData<fn(&'brand ()) -> &'brand ()>,
}

impl<'brand> GhostToken<'brand> {
    /// Mint a fresh brand for the duration of `f`. The `for<'new>` HRTB makes each brand
    /// distinct, so a cell of one brand can never be opened with another's token.
    pub fn new<R>(f: impl for<'new> FnOnce(GhostToken<'new>) -> R) -> R {
        f(GhostToken { _invariant: PhantomData })
    }
}

/// A cell branded with `'brand`. Aliasable by shared reference (nodes may point at each
/// other), but its contents are reachable only by presenting the matching token.
pub struct GhostCell<'brand, T: ?Sized> {
    _brand: PhantomData<fn(&'brand ()) -> &'brand ()>,
    value: UnsafeCell<T>,
}

// SAFETY: a GhostCell is Sync when its contents could be shared across threads under a token
// protocol; for this single-threaded demonstration the bounds mirror std::cell semantics.
unsafe impl<'brand, T: ?Sized + Send> Send for GhostCell<'brand, T> {}
unsafe impl<'brand, T: ?Sized + Send + Sync> Sync for GhostCell<'brand, T> {}

impl<'brand, T> GhostCell<'brand, T> {
    pub fn new(value: T) -> Self {
        GhostCell { _brand: PhantomData, value: UnsafeCell::new(value) }
    }
}

impl<'brand, T: ?Sized> GhostCell<'brand, T> {
    /// Read access — requires a *shared* borrow of the token (so many cells may be read at
    /// once, but none mutated meanwhile).
    pub fn borrow<'a>(&'a self, _token: &'a GhostToken<'brand>) -> &'a T {
        // SAFETY: a shared token borrow grants shared access to every cell of this brand;
        // the borrow checker's aliasing rules on `_token` enforce read-XOR-write globally.
        unsafe { &*self.value.get() }
    }

    /// Write access — requires a *unique* borrow of the token. Because the token is unique,
    /// no second `&mut` to any cell of this brand can coexist: this is what discharges L4
    /// (no aliasing `&mut`) for the whole aliased node graph at once.
    pub fn borrow_mut<'a>(&'a self, _token: &'a mut GhostToken<'brand>) -> &'a mut T {
        // SAFETY: the unique token borrow is the single key to the whole brand; holding it
        // `&mut` proves no other access to any same-brand cell is live.
        unsafe { &mut *self.value.get() }
    }
}

/// An intrusive node: the `next`/`prev` links live *inside* the node (branded cells), and
/// nodes are borrowed from an arena the caller owns — the list allocates nothing.
pub struct Node<'brand, 'arena> {
    pub val: u32,
    pub next: GhostCell<'brand, Option<&'arena Node<'brand, 'arena>>>,
    pub prev: GhostCell<'brand, Option<&'arena Node<'brand, 'arena>>>,
}

impl<'brand, 'arena> Node<'brand, 'arena> {
    pub fn new(val: u32) -> Self {
        Node { val, next: GhostCell::new(None), prev: GhostCell::new(None) }
    }

    /// Splice `b` in after `self`. Each link write presents the token — a seam crossing in
    /// what is, for an intrusive list, ordinary consumer code.
    pub fn link_after(&'arena self, b: &'arena Node<'brand, 'arena>,
                      token: &mut GhostToken<'brand>) {
        *self.next.borrow_mut(token) = Some(b);
        *b.prev.borrow_mut(token) = Some(self);
    }
}
