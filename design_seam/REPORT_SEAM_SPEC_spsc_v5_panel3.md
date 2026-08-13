# Seam phase report — SEAM_SPEC_spsc_v5.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 1) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Split's input type is unconstrained; safe code pops never-written bytes.
- **rationale:** attacker-soundness: §3's 'emits exactly this, and nothing more' omits RingBuffer, whose §1 representation is a plain record. head=1/tail=0 is a valid non-empty state under P3, so correct atomics hand safe code a Box<u8> from uninitialized slots. Every §4 stopper constrains outputs only. v3 bounded the input's type parameter but never its value. Fix (private fields) is decidable but unwritten.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let buf: RingBuffer<Box<u8>, 2> = RingBuffer { slots: Default::default(), head: AtomicUsize::new(1), tail: AtomicUsize::new(0) };
  let (_p, mut c) = split(buf);   // N >= 2 discharged; Box<u8>: Send; variance/dropck vacuous
  drop(c.pop());                  // free() on never-written bytes
  ```
