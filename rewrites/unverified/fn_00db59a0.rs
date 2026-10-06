// original: 0x00DB59A0 cursor_tree_lower_bound

/// Find the first tree node whose key is not below the search key.
///
/// `tree` points to the tree head (root pointer at `+0x0`); `key_ptr`
/// points at the one-word search key; `out` receives a 12-byte result and
/// `count_out`, when non-null, the number of nodes visited.
///
/// Nodes carry the key at `NODE_KEY`, the left-child link at `NODE_LEFT`
/// and the right-child link ORed with the colour bit at `NODE_RIGHT`; the
/// parent link is never read here. The walk goes left (recording the node
/// as the current candidate) while the node's key is above or equal to the
/// search key, else right with the low bit masked off. Both key
/// comparisons are UNSIGNED (`jae` in the original). An empty tree visits
/// nothing and reports no candidate.
///
/// On success `out` holds (candidate key, candidate node, tree); otherwise
/// (0, 0, tree). The return register carries `out` on every path.
///
/// Original: 0x00DB59A0 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_00db59a0(tree: u32, out: u32, key_ptr: u32, count_out: u32) -> u32 {
    unsafe {
        const NODE_RIGHT: u32 = 0x1c0;
        const NODE_LEFT: u32 = 0x1c4;
        const NODE_KEY: u32 = 0x1cc;
        const LINK_MASK: u32 = 0xffff_fffe;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut node = rd(tree);
        let mut candidate = 0u32;
        let mut visited = 0u32;
        if node != 0 {
            loop {
                let want = rd(key_ptr);
                let have = rd(node + NODE_KEY);
                if have >= want {
                    candidate = node;
                    node = rd(node + NODE_LEFT);
                } else {
                    node = rd(node + NODE_RIGHT) & LINK_MASK;
                }
                visited = visited.wrapping_add(1);
                if node == 0 {
                    break;
                }
            }
        }
        if count_out != 0 {
            wr(count_out, visited);
        }
        wr(out, 0);
        wr(out + 4, 0);
        wr(out + 8, tree);
        if candidate != 0 {
            wr(out + 4, candidate);
            wr(out, rd(candidate + NODE_KEY));
        }
        out
    }
});
