# Seam phase report — SEAM_SPEC_spsc_v4.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 47) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Invariance fixed; dropck ownership left open — §3's own recommended marker launders use-after-free
- **rationale:** attacker-prior-art: `PhantomData<*mut T>` gives invariance but not ownership to dropck; with the `#[may_dangle]` an owning container needs, `T: 'a` is no longer forced at handle drop, so P6's glue drops a payload whose borrow died. v3's variance ruling forecloses the 'not enumerated in §3' defense; drop-liveness is visible in the client's borrow check, so §5 cannot shield it. The `ConstEvaluatable` horn-2/3 cost is real but outranked.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  Foundation author writes an admissible declaration — invariance per §3, plus the `#[may_dangle]` every owning container in std carries (`Vec`, `Box`, `Arc`, `Rc`, `HashMap`, `BTreeMap`, `LinkedList`), because without it `Producer<T>`/`Consumer<T>` can never be dropped after their payload's regions end: `struct Inner<T> { slots: *mut MaybeUninit<T>, head: AtomicUsize, tail: AtomicUsize, _v: PhantomData<*mut T> }  // §3-sanctioned: invariance ✓, ownership ✗` / `unsafe impl<#[may_dangle] T> Drop for Inner<T> { /* P6: drops published payloads, frees store */ }`. Then, per the judge's verification of the client side: split at `N = 4` over a payload `Noisy<'a>` holding a `&'a String` (`Noisy<'a>: Send` from `&String: Send`; the const bound is satisfied; invariance is fully respected, no coercion occurs at all; affinity and `!Sync` untouched); safe code pushes the borrowing payload, lets the referent's region end, and the handles drop — 'P6's drop glue then runs `Noisy::drop`, reading a `&'a String` whose referent is already gone. Safe consumer code, §3 methods only, use-after-free.'
  ```
