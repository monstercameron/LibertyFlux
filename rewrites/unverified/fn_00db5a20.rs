// original: 0x00DB5A20 cursor_tree_rotate_a

/// Left rotation: the node's right child takes the node's place.
///
/// `tree` points to the tree head (root pointer at `+0x0`); `node` is the
/// rotation pivot, which must have a right child. Nodes carry the
/// right-child link ORed with the colour bit at `NODE_RIGHT`, the
/// left-child link at `NODE_LEFT` and the parent link at `NODE_PARENT`.
///
/// The right child `r` moves into the node's slot: the node's right link
/// becomes `r`'s left child with the node's own colour bit kept, `r`'s
/// parent becomes the node's parent, and the node becomes `r`'s left
/// child. The tree root, or the parent's link to the node (left link, or
/// right link with the parent's colour bit kept), is repointed at `r`.
/// The routine reads no keys and returns nothing observable.
///
/// Original: 0x00DB5A20 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5a20(tree: u32, node: u32) -> u32 {
    unsafe {
        const NODE_RIGHT: u32 = 0x1c0;
        const NODE_LEFT: u32 = 0x1c4;
        const NODE_PARENT: u32 = 0x1c8;
        const LINK_MASK: u32 = 0xffff_fffe;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let rlink = rd(node + NODE_RIGHT);
        let r = rlink & LINK_MASK;
        let r_left = rd(r + NODE_LEFT);
        wr(node + NODE_RIGHT, (rlink & 1) | r_left);
        if r_left != 0 {
            wr(r_left + NODE_PARENT, node);
        }
        let parent = rd(node + NODE_PARENT);
        wr(r + NODE_PARENT, parent);
        if node == rd(tree) {
            wr(tree, r);
        } else {
            let par = rd(node + NODE_PARENT);
            if node == rd(par + NODE_LEFT) {
                wr(par + NODE_LEFT, r);
            } else {
                wr(par + NODE_RIGHT, (rd(par + NODE_RIGHT) & 1) | r);
            }
        }
        wr(r + NODE_LEFT, node);
        wr(node + NODE_PARENT, r);
        0
    }
});
