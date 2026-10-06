// original: 0x00DB5770 cursor_tree_insert

/// Insert a node into the tree by key, rebalancing afterwards.
///
/// `tree` points to the tree head (root pointer at `+0x0`, entry count at
/// `+0x4`); `out` receives a 13-byte result; `key_ptr` points at the search
/// key; `node` is the node to link in. The node layout is the same as in
/// `cursor_tree_rotate_a`: right link plus colour bit at `NODE_RIGHT`,
/// left link at `NODE_LEFT`, parent at `NODE_PARENT`, key at `NODE_KEY`.
///
/// The walk descends left while the node's key is above the search key and
/// right while below, both comparisons UNSIGNED (`jbe`/`jae` in the
/// original), linking the node as the child of the last node visited and
/// recording its parent. An equal key inserts nothing. An empty tree takes
/// the node as its root. On insertion the node's colour bit is set, the
/// rebalancing helper is called (scripted on both sides), the key is
/// stored into the node and the entry count grows by one.
///
/// `out` holds (key, node-or-null, tree) plus a one-byte inserted flag at
/// `+0xc`. The return register carries `out`.
///
/// Original: 0x00DB5770 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_00db5770(tree: u32, out: u32, key_ptr: u32, node: u32) -> u32 {
    unsafe {
        const NODE_RIGHT: u32 = 0x1c0;
        const NODE_LEFT: u32 = 0x1c4;
        const NODE_PARENT: u32 = 0x1c8;
        const NODE_KEY: u32 = 0x1cc;
        const LINK_MASK: u32 = 0xffff_fffe;
        const REBALANCE: u32 = 1;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let want = rd(key_ptr);
        let root = rd(tree);
        let mut placed = 0u32;
        if root == 0 {
            wr(tree, node);
            placed = node;
        } else {
            let mut cur = root;
            loop {
                let have = rd(cur + NODE_KEY);
                if have > want {
                    let left = rd(cur + NODE_LEFT);
                    if left != 0 {
                        cur = left;
                        continue;
                    }
                    wr(cur + NODE_LEFT, node);
                    placed = node;
                    wr(node + NODE_PARENT, cur);
                    break;
                } else if have < want {
                    let rlink = rd(cur + NODE_RIGHT);
                    let right = rlink & LINK_MASK;
                    if right != 0 {
                        cur = right;
                        continue;
                    }
                    wr(cur + NODE_RIGHT, (rlink & 1) | node);
                    placed = node;
                    wr(node + NODE_PARENT, cur);
                    break;
                } else {
                    break;
                }
            }
        }
        if placed == 0 {
            wr(out, 0);
            wr(out + 4, 0);
            wr(out + 8, tree);
            ((out + 0xc) as *mut u8).write(0);
        } else {
            wr(node + NODE_RIGHT, rd(node + NODE_RIGHT) | 1);
            let _: u32 = lf_checker_rt::callee_thiscall!(REBALANCE, u32, tree, node);
            wr(node + NODE_KEY, want);
            wr(tree + 4, rd(tree + 4).wrapping_add(1));
            wr(out + 4, placed);
            wr(out, rd(placed + NODE_KEY));
            wr(out + 8, tree);
            ((out + 0xc) as *mut u8).write(1);
        }
        out
    }
});
