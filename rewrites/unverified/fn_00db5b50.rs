// original: 0x00DB5B50 cursor_tree_successor

/// Step to the in-order successor of a tree node.
///
/// `node` points at the node. Nodes carry the right-child link ORed with
/// the colour bit at `NODE_RIGHT`, the left-child link at `NODE_LEFT` and
/// the parent link at `NODE_PARENT`; keys are never read here.
///
/// When the node has a right child (the link with the low bit masked off
/// is non-null) the answer is the leftmost node of that subtree. Otherwise
/// the walk climbs through parents while the current node is its parent's
/// right child (parent link masked the same way), and answers the first
/// parent reached from the left, or null when the root is passed. Pointer
/// comparisons are plain equality; there is no key comparison.
///
/// The original takes its argument on the stack and cleans nothing up
/// (plain `ret`); callers adjust the stack, so this is cdecl.
///
/// Original: 0x00DB5B50 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00db5b50(node: u32) -> u32 {
    unsafe {
        const NODE_RIGHT: u32 = 0x1c0;
        const NODE_LEFT: u32 = 0x1c4;
        const NODE_PARENT: u32 = 0x1c8;
        const LINK_MASK: u32 = 0xffff_fffe;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut cur = node;
        let mut next = rd(cur + NODE_RIGHT) & LINK_MASK;
        if next != 0 {
            // Leftmost of the right subtree.
            let mut down = rd(next + NODE_LEFT);
            while down != 0 {
                next = down;
                down = rd(down + NODE_LEFT);
            }
            return next;
        }
        // Climb while we arrive from the right.
        let mut up = rd(cur + NODE_PARENT);
        if up == 0 {
            return 0;
        }
        loop {
            let parent_right = rd(up + NODE_RIGHT) & LINK_MASK;
            if cur != parent_right {
                return up;
            }
            cur = up;
            up = rd(up + NODE_PARENT);
            if up == 0 {
                return 0;
            }
        }
    }
});
