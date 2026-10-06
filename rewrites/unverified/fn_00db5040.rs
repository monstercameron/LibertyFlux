// original: 0x00DB5040 cursor_tree_advance

/// Advance a tree cursor to its next node and copy the cursor out.
///
/// `this` points to a three-word cursor: the current key at `+0x0`, the
/// current node at `CUR_NODE` and a root-pointer-or-flag word at
/// `CUR_ROOT`. `out` receives the three cursor words.
///
/// When the current node is set, the next node comes from the successor
/// helper (scripted on both sides) and is stored back. Otherwise, when the
/// root word is odd and non-null, its low bit is cleared, the word is
/// stored back and the current node becomes the leftmost node of the tree
/// it points at (following `NODE_LEFT` to the end); an even or null root
/// word leaves the cursor alone. The current key is then refreshed from
/// the current node's `NODE_KEY` when a node is set. The return register
/// carries `out`.
///
/// Original: 0x00DB5040 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00db5040(this: u32, out: u32) -> u32 {
    unsafe {
        const CUR_NODE: u32 = 4;
        const CUR_ROOT: u32 = 8;
        const NODE_LEFT: u32 = 0x1c4;
        const NODE_KEY: u32 = 0x1cc;
        const SUCCESSOR: u32 = 1;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let cur = rd(this + CUR_NODE);
        if cur != 0 {
            let nxt: u32 = lf_checker_rt::callee_cdecl!(SUCCESSOR, u32, cur);
            wr(this + CUR_NODE, nxt);
        } else {
            let root_word = rd(this + CUR_ROOT);
            if (root_word & 0xffff_fffe) != 0 && (root_word & 1) != 0 {
                let base = root_word & 0xffff_fffe;
                wr(this + CUR_ROOT, base);
                let mut c = rd(base);
                let mut e = rd(c + NODE_LEFT);
                if e != 0 {
                    loop {
                        c = e;
                        e = rd(e + NODE_LEFT);
                        if e == 0 {
                            break;
                        }
                    }
                }
                wr(this + CUR_NODE, c);
            }
        }
        let node = rd(this + CUR_NODE);
        if node != 0 {
            wr(this, rd(node + NODE_KEY));
        }
        wr(out, rd(this));
        wr(out + 4, rd(this + CUR_NODE));
        wr(out + 8, rd(this + CUR_ROOT));
        out
    }
});
