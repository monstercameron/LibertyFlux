//! Intrusive doubly-linked chain nodes (lifted from the 0x00A7Cxxx family).
//!
//! Original layout touched by the cluster (byte offsets): `0x78` slot word,
//! `0x114` container cookie, `0x118` forward link, `0x11C` back link,
//! `0x120` second forward link, `0x124` anchor link, `0x130` claim word,
//! `0x13C` flag byte. Every other byte is payload the cluster never reads.

/// Index of a node in [`ChainArena::nodes`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NodeId(pub u32);

/// A chain node with native link types. `None` is the null link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainNode {
    /// Word at +0x78, cleared on the sweep anchor's sibling.
    pub slot78: u32,
    /// Opaque cookie at +0x114: address of the container object, which lives
    /// outside the cluster. Becomes a typed handle when the container lifts.
    pub container: u32,
    /// Forward link (+0x118).
    pub fwd: Option<NodeId>,
    /// Back link (+0x11C).
    pub back: Option<NodeId>,
    /// Second forward link (+0x120), walked by the sweep and find routines.
    pub link: Option<NodeId>,
    /// Anchor link (+0x124).
    pub anchor: Option<NodeId>,
    /// Claim word (+0x130): zero means unclaimed.
    pub claim: u32,
    /// Flag byte (+0x13C). Bits 2,3 gate the walks; bit 1 gates anchoring.
    pub flags: u8,
}

impl ChainNode {
    /// Blank node: all links null, matching a zeroed record.
    #[must_use]
    pub fn blank() -> Self {
        Self {
            slot78: 0,
            container: 0,
            fwd: None,
            back: None,
            link: None,
            anchor: None,
            claim: 0,
            flags: 0,
        }
    }
}

/// All nodes the lifted chain functions can reach.
#[derive(Clone, Debug, Default)]
pub struct ChainArena {
    /// Nodes by index; `NodeId(i)` addresses `nodes[i]`.
    pub nodes: Vec<ChainNode>,
}

impl ChainArena {
    /// Fetch a node; panics on a dangling index exactly where the original
    /// would fault on a bad pointer.
    #[must_use]
    pub fn get(&self, id: NodeId) -> &ChainNode {
        &self.nodes[id.0 as usize]
    }
    /// Fetch a node mutably; same fault rule as [`Self::get`].
    pub fn get_mut(&mut self, id: NodeId) -> &mut ChainNode {
        &mut self.nodes[id.0 as usize]
    }
}

/// Is `target` reachable from `start` via forward links?
/// Every visited node must carry flag bits 2 and 3; the first node that
/// fails ends the walk. (Original: `chain_contains_target`.)
pub fn contains(arena: &ChainArena, start: NodeId, target: NodeId) -> bool {
    let mut node = start;
    loop {
        let flags = arena.get(node).flags;
        if flags & 0x04 == 0 || flags & 0x08 == 0 {
            return false;
        }
        match arena.get(node).fwd {
            Some(next) if next == target => return true,
            Some(next) => node = next,
            // The original compares the raw next word with the target
            // address before the null check; a null target is unrepresentable
            // here (targets are always real ids), so null ends the walk.
            None => return false,
        }
    }
}

/// Do `start` and every forward successor carry flag bits 2 and 3?
/// (Original: `chain_all_flagged`.)
pub fn all_flagged(arena: &ChainArena, start: NodeId) -> bool {
    let mut node = start;
    loop {
        let flags = arena.get(node).flags;
        if flags & 0x04 == 0 || flags & 0x08 == 0 {
            return false;
        }
        match arena.get(node).fwd {
            Some(next) => node = next,
            None => return true,
        }
    }
}

/// Set flag bits 2 and 3 on `start` and every forward successor.
/// (Original: `chain_set_flags`.)
pub fn set_flags(arena: &mut ChainArena, start: NodeId) {
    let mut node = start;
    loop {
        arena.get_mut(node).flags |= 0x0C;
        match arena.get(node).fwd {
            Some(next) => node = next,
            None => return,
        }
    }
}

/// Find-or-fallback lookup: run the direct search over `(a1, a2)` and return
/// its result when non-null, else retry through the +0x114 container with
/// `(a1, 0, this)`. (Originals: `find_or_fallback_a` and
/// `find_or_fallback_b`; the two sites have identical bodies, so they share
/// this one lift.)
pub fn find_or_fallback(
    arena: &mut ChainArena,
    this: NodeId,
    a1: u32,
    a2: u32,
    search: &mut dyn FnMut(&mut ChainArena, NodeId, u32, u32) -> Option<NodeId>,
    fallback: &mut dyn FnMut(&mut ChainArena, u32, u32, u32, NodeId) -> Option<NodeId>,
) -> Option<NodeId> {
    if let Some(found) = search(arena, this, a1, a2) {
        return Some(found);
    }
    let container = arena.get(this).container;
    fallback(arena, container, a1, 0, this)
}

/// Clear flag bit 2 and set bit 1, then run the notify step when the
/// argument's low byte is nonzero. Returns the new flag byte when no call
/// is made, else the notify step's answer truncated to a byte.
/// (Original: `update_flags_notify`.)
pub fn update_flags_notify(
    arena: &mut ChainArena,
    this: NodeId,
    arg: u32,
    notify: &mut dyn FnMut(&mut ChainArena, NodeId) -> u32,
) -> u8 {
    let flags = (arena.get(this).flags & !0x04) | 0x02;
    arena.get_mut(this).flags = flags;
    if arg & 0xFF != 0 {
        return notify(arena, this) as u8;
    }
    flags
}

/// First node in the +0x120 chain from the callee-provided head whose flags
/// read `(flags & 0x0C) == 0x0C` with bit 1 clear.
/// (Original: `find_flagged_node`.)
pub fn find_flagged_node(
    arena: &mut ChainArena,
    this: NodeId,
    head: &mut dyn FnMut(&mut ChainArena, NodeId) -> Option<NodeId>,
) -> Option<NodeId> {
    let mut node = head(arena, this);
    while let Some(id) = node {
        let flags = arena.get(id).flags;
        if flags & 0x04 != 0 && flags & 0x08 != 0 && flags & 0x02 == 0 {
            return Some(id);
        }
        node = arena.get(id).link;
    }
    None
}

/// Follow +0x124 once, then walk +0x11C links to the end; return the last
/// node reached, or null when the anchor is null.
/// (Original: `last_link_in_chain`.)
pub fn last_link(arena: &ChainArena, this: NodeId) -> Option<NodeId> {
    let mut node = arena.get(this).anchor?;
    loop {
        match arena.get(node).back {
            Some(prev) => node = prev,
            None => return Some(node),
        }
    }
}

/// Sweep the +0x120 chain from the callee-provided head: touch every
/// bit-2-flagged unclaimed node, adopt the first one with bit 3 set and
/// bit 1 clear as the anchor, then link the anchor through the link step
/// and clear the anchor block's +0x78 slot. Returns whether anything was
/// touched. (Original: `sweep_touch_and_link`.)
///
/// The original passes `sibling+0x10` and `anchor+0x10` (sub-record views)
/// to the link step and dereferences the sibling without a null check; a
/// null sibling faults there. The lift passes plain ids and panics on a
/// null sibling instead of faulting.
pub fn sweep_touch_and_link(
    arena: &mut ChainArena,
    this: NodeId,
    head: &mut dyn FnMut(&mut ChainArena, NodeId) -> Option<NodeId>,
    touch: &mut dyn FnMut(&mut ChainArena, NodeId) -> u32,
    link: &mut dyn FnMut(&mut ChainArena, NodeId, NodeId) -> u32,
) -> bool {
    let mut node = head(arena, this);
    if node.is_none() {
        return false;
    }
    let mut touched_any = false;
    let mut anchor: Option<NodeId> = None;
    while let Some(id) = node {
        let flags = arena.get(id).flags;
        if flags & 0x04 != 0 {
            if arena.get(id).claim == 0 {
                touch(arena, id);
                touched_any = true;
            }
            if anchor.is_none() && flags & 0x08 != 0 && flags & 0x02 == 0 {
                anchor = Some(id);
            }
        }
        node = arena.get(id).link;
    }
    if let Some(a) = anchor {
        let sibling = arena
            .get(a)
            .fwd
            .expect("sweep anchor without a +0x118 sibling faults in the original");
        link(arena, sibling, a);
        arena.get_mut(sibling).slot78 = 0;
    }
    touched_any
}

/// Splice `new_node` into the doubly linked chain ahead of `this`.
/// (Original: `splice_node`.)
pub fn splice_node(arena: &mut ChainArena, this: NodeId, new_node: NodeId) {
    let prev = arena.get(this).back;
    let next = arena.get(this).fwd;
    if let Some(p) = prev {
        arena.get_mut(p).link = Some(new_node);
    }
    arena.get_mut(this).back = Some(new_node);
    arena.get_mut(new_node).fwd = next;
    arena.get_mut(new_node).link = Some(this);
    arena.get_mut(new_node).back = prev;
}

/// Touch every node in the chain hanging off +0x124 via +0x11C links.
/// The original always returns 1; the lift returns `()` and the
/// differential test asserts the constant on the 32-bit side.
/// (Original: `touch_chain`.)
pub fn touch_chain(
    arena: &mut ChainArena,
    this: NodeId,
    touch: &mut dyn FnMut(&mut ChainArena, NodeId) -> u32,
) {
    let mut node = arena.get(this).anchor;
    while let Some(id) = node {
        touch(arena, id);
        node = arena.get(id).back;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tiny xorshift for deterministic property tests (no dependencies).
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn below(&mut self, n: u32) -> u32 {
            (self.next() % n as u64) as u32
        }
    }

    fn chain_arena(n: usize, rng: &mut Rng) -> ChainArena {
        let mut nodes = Vec::with_capacity(n);
        for i in 0..n {
            let mut nd = ChainNode::blank();
            nd.flags = rng.below(256) as u8;
            nd.fwd = if i + 1 < n { Some(NodeId(i as u32 + 1)) } else { None };
            nd.back = if i > 0 { Some(NodeId(i as u32 - 1)) } else { None };
            nd.link = nd.fwd;
            nodes.push(nd);
        }
        ChainArena { nodes }
    }

    #[test]
    fn set_flags_makes_all_flagged_true() {
        let mut rng = Rng(0x1234);
        for _ in 0..50 {
            let n = 1 + rng.below(8) as usize;
            let mut arena = chain_arena(n, &mut rng);
            set_flags(&mut arena, NodeId(0));
            assert!(all_flagged(&arena, NodeId(0)));
            if n > 1 {
                assert!(contains(&arena, NodeId(0), NodeId(n as u32 - 1)));
            }
        }
    }

    #[test]
    fn contains_matches_model() {
        let mut rng = Rng(0x99);
        for _ in 0..50 {
            let n = 1 + rng.below(8) as usize;
            let arena = chain_arena(n, &mut rng);
            for t in 0..n {
                // Model: the walk returns true upon REACHING t, so only the
                // flags strictly before t are examined; t == 0 is found only
                // via a back-edge (never in these arenas).
                let mut expect = t > 0;
                for k in 0..t {
                    if arena.nodes[k].flags & 0x0C != 0x0C {
                        expect = false;
                        break;
                    }
                }
                assert_eq!(contains(&arena, NodeId(0), NodeId(t as u32)), expect);
            }
        }
    }

    #[test]
    fn splice_keeps_chain_connected() {
        let mut arena = chain_arena(4, &mut Rng(7));
        set_flags(&mut arena, NodeId(0));
        // Fresh node 4 spliced ahead of node 1.
        arena.nodes.push(ChainNode::blank());
        arena.get_mut(NodeId(4)).flags = 0x0C;
        splice_node(&mut arena, NodeId(1), NodeId(4));
        assert_eq!(arena.get(NodeId(1)).back, Some(NodeId(4)));
        assert_eq!(arena.get(NodeId(4)).link, Some(NodeId(1)));
        assert_eq!(arena.get(NodeId(0)).link, Some(NodeId(4)));
        // fwd-walk from 0 still reaches every original node.
        for t in 1..4 {
            assert!(contains(&arena, NodeId(0), NodeId(t)));
        }
    }

    #[test]
    fn last_link_walks_back_links() {
        let mut arena = chain_arena(3, &mut Rng(7));
        arena.get_mut(NodeId(0)).anchor = Some(NodeId(2));
        assert_eq!(last_link(&arena, NodeId(0)), Some(NodeId(0)));
        arena.get_mut(NodeId(1)).anchor = None;
        assert_eq!(last_link(&arena, NodeId(1)), None);
    }

    #[test]
    fn sweep_touches_and_anchors() {
        // Nodes: 0 (head) ->link 1 ->link 2; node 1 claimed, node 2 anchorable.
        let mut arena = ChainArena { nodes: vec![ChainNode::blank(); 3] };
        arena.nodes[0].flags = 0x04;
        arena.nodes[0].link = Some(NodeId(1));
        arena.nodes[1].flags = 0x04;
        arena.nodes[1].claim = 9;
        arena.nodes[1].link = Some(NodeId(2));
        arena.nodes[2].flags = 0x0C;
        arena.nodes[2].fwd = Some(NodeId(1));
        arena.nodes[1].slot78 = 0xDEAD;
        let mut touched = Vec::new();
        let mut linked = Vec::new();
        let r = sweep_touch_and_link(
            &mut arena,
            NodeId(0),
            &mut |_, _| Some(NodeId(0)),
            &mut |_, id| {
                touched.push(id);
                0
            },
            &mut |_, s, a| {
                linked.push((s, a));
                0
            },
        );
        assert!(r);
        assert_eq!(touched, vec![NodeId(0), NodeId(2)]);
        assert_eq!(linked, vec![(NodeId(1), NodeId(2))]);
        assert_eq!(arena.nodes[1].slot78, 0);
    }
}
