//! Phase C contrast: using the intrusive list from *ordinary* consumer code. Note what is
//! different from SPSC/MMIO — the consumer must carry and present a `GhostToken` at **every**
//! link and traversal step. There is still zero `unsafe` in consumer code (the brand keeps it
//! sound), but the obligation no longer sits at one foundation `split`: it threads through the
//! whole computation. We *count* the crossings to make the horn-3 drop concrete.

#![forbid(unsafe_code)] // consumer code is still unsafe-free — the cost is the token, not unsafe

use seam_spsc::intrusive::{GhostToken, Node};

#[test]
fn intrusive_threads_token_through_every_access() {
    // Seam width = number of token presentations (borrow / borrow_mut) the consumer makes.
    let crossings = GhostToken::new(|mut token| {
        // Arena is created inside the brand scope so the nodes carry the brand lifetime.
        let arena = vec![Node::new(10), Node::new(20), Node::new(30)];
        let (a, b, c) = (&arena[0], &arena[1], &arena[2]);

        let mut crossings = 0u32;

        // Build the list a <-> b <-> c. Each `link_after` presents the token twice
        // (next of the left node, prev of the right node).
        a.link_after(b, &mut token);
        crossings += 2;
        b.link_after(c, &mut token);
        crossings += 2;

        // Traverse forward from `a`, summing values. Each hop reads a link cell, which
        // requires presenting the (shared) token — one crossing per node visited.
        let mut cur = Some(a);
        let mut sum = 0u32;
        while let Some(n) = cur {
            sum += n.val;
            cur = *n.next.borrow(&token); // <-- the seam crossing, once per node
            crossings += 1;
        }
        assert_eq!(sum, 60, "traversal visits all three nodes in order");

        crossings
    });

    // SPSC's consumer crossed the seam ONCE (at split), regardless of element count.
    // The intrusive consumer crossed it 7 times for a 3-node list: 4 to link, 3 to walk.
    // Seam width here is O(links + accesses), not O(1) — concentration (horn 3) has dropped.
    assert_eq!(crossings, 7, "every link and traversal step crossed the seam");
    eprintln!("intrusive seam width: {crossings} token presentations for a 3-node list \
               (SPSC = 1, constant)");
}
