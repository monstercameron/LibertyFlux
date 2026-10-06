// original: 0x00DB5AC0 cursor_tree_rotate_b

/// Right rotation: the node's left child takes the node's place.
///
/// `tree` points to the tree head (root pointer at `+0x0`); `node` is the
/// rotation pivot, which must have a left child. The node layout is the
/// same as in `cursor_tree_rotate_a`: right link plus colour bit at
/// `NODE_RIGHT`, left link at `NODE_LEFT`, parent at `NODE_PARENT`.
///
/// The left child `l` moves into the node's slot. Note the asymmetry with
/// the left rotation: the node's new left link is the masked link only
/// (the node's old colour bit is dropped), while `l`'s new right link
/// keeps `l`'s own colour bit. The tree root, or the parent's link to the
/// node (compared against the parent's masked right link), is repointed
/// at `l`. No keys are read and nothing observable is returned.
///
/// Original: 0x00DB5AC0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5ac0(tree: u32, node: u32) -> u32 {
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

        let l = rd(node + NODE_LEFT);
        let l_right = rd(l + NODE_RIGHT) & LINK_MASK;
        wr(node + NODE_LEFT, l_right);
        if l_right != 0 {
            wr(l_right + NODE_PARENT, node);
        }
        let parent = rd(node + NODE_PARENT);
        wr(l + NODE_PARENT, parent);
        if node == rd(tree) {
            wr(tree, l);
        } else {
            let par = rd(node + NODE_PARENT);
            if node == (rd(par + NODE_RIGHT) & LINK_MASK) {
                wr(par + NODE_RIGHT, (rd(par + NODE_RIGHT) & 1) | l);
            } else {
                wr(par + NODE_LEFT, l);
            }
        }
        wr(l + NODE_RIGHT, (rd(l + NODE_RIGHT) & 1) | node);
        wr(node + NODE_PARENT, l);
        0
    }
});
