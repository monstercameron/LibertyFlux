// original: 0x00635480 bst_remove (proposed)

/// Remove `node` from the string-keyed binary search tree headed by `this`.
///
/// Header holds the root at `+ROOT` and the count at `+COUNT`; nodes link
/// through `+PARENT`/`+LEFT`/`+RIGHT` (keys are not read). A node with no
/// children is unlinked directly; a node with one child is replaced by
/// that child; a node with two children is replaced by its successor (the
/// successor callee), which takes over both of the node's links while the
/// successor's own child (if any) takes the successor's old place. The
/// replacement's parent link, the node's old parent slot and the root (when
/// it pointed at the node) are updated, the node's own links are cleared
/// and the count is decremented.
///
/// Returns the node's old parent (the value the original reloads into eax
/// on every path).
///
/// Original: 0x00635480 (thiscall, one stack word, one call).
lf_checker_rt::export!(thiscall, rw_00635480(this: u32, node: u32) -> u32 {
    unsafe {
        const ROOT: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const PARENT: u32 = 0x34;
        const LEFT: u32 = 0x38;
        const RIGHT: u32 = 0x3c;
        const SUCC: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let left = rd32(node.wrapping_add(LEFT));
        let right = rd32(node.wrapping_add(RIGHT));
        let repl: u32;
        if left == 0 && right == 0 {
            repl = 0;
        } else if left == 0 {
            repl = right;
            wr32(repl.wrapping_add(PARENT), rd32(node.wrapping_add(PARENT)));
        } else if right == 0 {
            repl = left;
            wr32(repl.wrapping_add(PARENT), rd32(node.wrapping_add(PARENT)));
        } else {
            let s: u32 = lf_checker_rt::callee_stdcall!(SUCC, u32, node);
            let mut sc = rd32(s.wrapping_add(LEFT));
            if sc == 0 {
                sc = rd32(s.wrapping_add(RIGHT));
            }
            if sc != 0 {
                wr32(sc.wrapping_add(PARENT), rd32(s.wrapping_add(PARENT)));
            }
            let sp = rd32(s.wrapping_add(PARENT));
            if rd32(sp.wrapping_add(LEFT)) == s {
                wr32(sp.wrapping_add(LEFT), sc);
            } else if rd32(sp.wrapping_add(RIGHT)) == s {
                wr32(sp.wrapping_add(RIGHT), sc);
            }
            wr32(s.wrapping_add(LEFT), left);
            wr32(left.wrapping_add(PARENT), s);
            wr32(s.wrapping_add(RIGHT), right);
            wr32(right.wrapping_add(PARENT), s);
            repl = s;
            wr32(repl.wrapping_add(PARENT), rd32(node.wrapping_add(PARENT)));
        }
        let par = rd32(node.wrapping_add(PARENT));
        if par != 0 {
            if rd32(par.wrapping_add(LEFT)) == node {
                wr32(par.wrapping_add(LEFT), repl);
            } else if rd32(par.wrapping_add(RIGHT)) == node {
                wr32(par.wrapping_add(RIGHT), repl);
            }
        }
        if rd32(this.wrapping_add(ROOT)) == node {
            wr32(this.wrapping_add(ROOT), repl);
        }
        wr32(node.wrapping_add(PARENT), 0);
        wr32(node.wrapping_add(LEFT), 0);
        wr32(node.wrapping_add(RIGHT), 0);
        wr32(this.wrapping_add(COUNT), rd32(this.wrapping_add(COUNT)).wrapping_sub(1));
        par
    }
});
