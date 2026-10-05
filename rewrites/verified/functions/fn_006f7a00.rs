// original: 0x006f7a00 rbtree_insert_rebalance_alt (proposed)

/// Rebalance a red-black tree after an insertion at NODE, climbing from NODE
/// toward the root while the parent is red.
///
/// `this` points to the tree anchor whose first word is the root pointer, and
/// `node` is the freshly inserted node. Every node carries three links:
/// `RIGHT` holds the right-child pointer with the colour in bit 0 (1 = red),
/// `LEFT` the left child, `PARENT` the parent. The right rotation is the
/// intercepted callee (id 1); the left rotation is inline below (`rotate_left`
/// lifts `[P+RIGHT]&~1` above P, keeps both colours, and repairs the
/// grandparent link or the root).
///
/// Each iteration reads the parent and grandparent: a black parent ends the
/// climb; a red uncle (or aunt on the mirrored side) means repaint parent,
/// uncle and grandparent and climb to the grandparent; otherwise repaint the
/// parent black and the grandparent red and rotate, with an extra inner
/// rotation first when the current node hangs on the inside. Edge cases: NODE
/// equal to the root, or a black parent on entry, only repaints the root;
/// every exit returns the (possibly rotated) root, repainted black.
///
/// Original: 0x006f7a00 (thiscall, one stack word: the inserted node).
lf_checker_rt::export!(thiscall, rw_006f7a00(this: u32, node: u32) -> u32 {
    unsafe {
        const RIGHT: u32 = 0x4c;
        const LEFT: u32 = 0x50;
        const PARENT: u32 = 0x54;
        const ROT_RIGHT: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn is_red(n: u32) -> bool {
            unsafe { (rd8(n + RIGHT) & 1) != 0 }
        }
        #[inline(always)]
        unsafe fn blacken(n: u32) {
            unsafe { wr32(n + RIGHT, rd32(n + RIGHT) & !1) }
        }
        #[inline(always)]
        unsafe fn redden(n: u32) {
            unsafe { wr32(n + RIGHT, rd32(n + RIGHT) | 1) }
        }
        #[inline(always)]
        unsafe fn exit_with_root(this: u32) -> u32 {
            unsafe {
                let root = rd32(this);
                blacken(root);
                root
            }
        }
        /// Left rotation at P, lifting `[P+RIGHT]&~1` into P's place.
        unsafe fn rotate_left(this: u32, p: u32) {
            unsafe {
                let up = rd32(p + RIGHT) & (!1);
                wr32(p + RIGHT, (rd32(p + RIGHT) & 1) | rd32(up + LEFT));
                let down = rd32(up + LEFT);
                if down != 0 {
                    wr32(down + PARENT, p);
                }
                wr32(up + PARENT, rd32(p + PARENT));
                if p == rd32(this) {
                    wr32(this, up);
                } else {
                    let grand = rd32(p + PARENT);
                    if p == rd32(grand + LEFT) {
                        wr32(grand + LEFT, up);
                    } else {
                        wr32(grand + RIGHT, (rd32(grand + RIGHT) & 1) | up);
                    }
                }
                wr32(up + LEFT, p);
                wr32(p + PARENT, up);
            }
        }

        if node == rd32(this) {
            return exit_with_root(this);
        }
        let mut cur = node;
        loop {
            let parent = rd32(cur + PARENT);
            if !is_red(parent) {
                return exit_with_root(this);
            }
            let grand = rd32(parent + PARENT);
            if parent == rd32(grand + LEFT) {
                let uncle = rd32(grand + RIGHT) & (!1);
                if uncle != 0 && is_red(uncle) {
                    blacken(parent);
                    blacken(uncle);
                    redden(grand);
                    cur = grand;
                } else {
                    if cur == (rd32(parent + RIGHT) & (!1)) {
                        rotate_left(this, parent);
                        cur = parent;
                    }
                    blacken(rd32(cur + PARENT));
                    redden(rd32(rd32(cur + PARENT) + PARENT));
                    lf_checker_rt::callee_thiscall!(ROT_RIGHT, u32, this,
                        rd32(rd32(cur + PARENT) + PARENT));
                }
            } else {
                let aunt = rd32(grand + LEFT);
                if aunt != 0 && is_red(aunt) {
                    blacken(parent);
                    blacken(aunt);
                    redden(grand);
                    cur = grand;
                } else {
                    if cur == rd32(parent + LEFT) {
                        cur = parent;
                        lf_checker_rt::callee_thiscall!(ROT_RIGHT, u32, this, cur);
                    }
                    blacken(rd32(cur + PARENT));
                    redden(rd32(rd32(cur + PARENT) + PARENT));
                    rotate_left(this, rd32(rd32(cur + PARENT) + PARENT));
                }
            }
            if cur == rd32(this) {
                return exit_with_root(this);
            }
        }
    }
});
