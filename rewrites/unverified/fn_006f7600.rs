// original: 0x006f7600 rbtree_erase_rebalance_alt (proposed)

/// Climb from NODE toward the root of a red-black tree, repainting colours
/// and rotating, until the tree is balanced above NODE.
///
/// `this` points to the tree anchor whose first word is the root pointer, and
/// `node` is the node the climb starts from. Every node carries three links:
/// `RIGHT` holds the right-child pointer with the colour in bit 0 (1 = red),
/// `LEFT` the left child, `PARENT` the parent. The two rotations are the
/// intercepted callees (id 1 rotates left, id 2 rotates right); this routine
/// only decides which to run and maintains the colour bits, including copying
/// the low bit between a node and its grandparent link on the finishing paths
/// (an xor of the two low bits applied back, so only bit 0 can change).
///
/// Each iteration tests the current node: red exits at once, otherwise the
/// side it hangs on selects one of two mirrored fixups. The fixup either
/// climbs to the parent (repainted paths) or finishes at the root after two
/// rotations. Edge cases: NODE equal to the root repaints the root and
/// returns it; a null grandparent or sibling link ends the climb at the
/// parent; every exit repaints the returned node black.
///
/// Original: 0x006f7600 (thiscall, one stack word: the starting node).
lf_checker_rt::export!(thiscall, rw_006f7600(this: u32, node: u32) -> u32 {
    unsafe {
        const RIGHT: u32 = 0x4c;
        const LEFT: u32 = 0x50;
        const PARENT: u32 = 0x54;
        const ROT_LEFT: u32 = 1;
        const ROT_RIGHT: u32 = 2;

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

        /// What the shared tail does after a fixup: compare NEXT with the
        /// root and either climb to it or finish there.
        enum Tail {
            Climb(u32),
            Finish,
        }

        /// Fixup for a current node that is the LEFT child of its parent.
        unsafe fn branch_left(this: u32, cur: u32, parent: u32) -> Tail {
            unsafe {
                let mut link_parent = parent;
                let mut grand = rd32(link_parent + RIGHT) & (!1);
                let mut grand_slot = link_parent + RIGHT;
                if grand == 0 {
                    return Tail::Climb(link_parent);
                }
                if is_red(grand) {
                    blacken(grand);
                    redden(rd32(cur + PARENT));
                    lf_checker_rt::callee_thiscall!(ROT_LEFT, u32, this, rd32(cur + PARENT));
                    link_parent = rd32(cur + PARENT);
                    grand = rd32(link_parent + RIGHT) & (!1);
                    grand_slot = link_parent + RIGHT;
                }
                if grand == 0 {
                    return Tail::Climb(link_parent);
                }
                let sib = rd32(grand + LEFT);
                if sib == 0 || !is_red(sib) {
                    let held = rd32(grand + RIGHT);
                    if (held & (!1)) == 0 || !is_red(held & (!1)) {
                        wr32(grand + RIGHT, held | 1);
                        return Tail::Climb(rd32(cur + PARENT));
                    }
                }
                let held = rd32(grand + RIGHT);
                if (held & (!1)) == 0 || !is_red(held & (!1)) {
                    blacken(sib);
                    redden(grand);
                    lf_checker_rt::callee_thiscall!(ROT_RIGHT, u32, this, grand);
                    grand_slot = rd32(cur + PARENT) + RIGHT;
                    grand = rd32(grand_slot) & (!1);
                }
                let low = rd8(grand_slot) as u32;
                let gword = rd32(grand + RIGHT);
                wr32(grand + RIGHT, gword ^ ((low ^ gword) & 1));
                blacken(rd32(cur + PARENT));
                blacken(rd32(grand + RIGHT) & (!1));
                lf_checker_rt::callee_thiscall!(ROT_LEFT, u32, this, rd32(cur + PARENT));
                Tail::Finish
            }
        }

        /// Fixup for a current node that is NOT the left child of its parent.
        unsafe fn branch_right(this: u32, cur: u32, parent: u32) -> Tail {
            unsafe {
                let mut link_parent = parent;
                let mut sib = rd32(link_parent + LEFT);
                if sib == 0 {
                    return Tail::Climb(link_parent);
                }
                if is_red(sib) {
                    blacken(sib);
                    redden(rd32(cur + PARENT));
                    lf_checker_rt::callee_thiscall!(ROT_RIGHT, u32, this, rd32(cur + PARENT));
                    link_parent = rd32(cur + PARENT);
                    sib = rd32(link_parent + LEFT);
                }
                if sib == 0 {
                    return Tail::Climb(link_parent);
                }
                let held = rd32(sib + RIGHT);
                if (held & (!1)) == 0 || !is_red(held & (!1)) {
                    let inner = rd32(sib + LEFT);
                    if inner == 0 || !is_red(inner) {
                        wr32(sib + RIGHT, held | 1);
                        return Tail::Climb(rd32(cur + PARENT));
                    }
                }
                let inner = rd32(sib + LEFT);
                if inner == 0 || !is_red(inner) {
                    blacken(held & (!1));
                    redden(sib);
                    lf_checker_rt::callee_thiscall!(ROT_LEFT, u32, this, sib);
                    link_parent = rd32(cur + PARENT);
                    sib = rd32(link_parent + LEFT);
                }
                let low = rd8(link_parent + RIGHT) as u32;
                let sword = rd32(sib + RIGHT);
                wr32(sib + RIGHT, sword ^ ((low ^ sword) & 1));
                blacken(rd32(cur + PARENT));
                blacken(rd32(sib + LEFT));
                lf_checker_rt::callee_thiscall!(ROT_RIGHT, u32, this, rd32(cur + PARENT));
                Tail::Finish
            }
        }

        if node == rd32(this) {
            blacken(node);
            return node;
        }
        let mut cur = node;
        loop {
            if is_red(cur) {
                blacken(cur);
                return cur;
            }
            let parent = rd32(cur + PARENT);
            let tail = if cur == rd32(parent + LEFT) {
                branch_left(this, cur, parent)
            } else {
                branch_right(this, cur, parent)
            };
            match tail {
                Tail::Finish => {
                    let root = rd32(this);
                    blacken(root);
                    return root;
                }
                Tail::Climb(next) => {
                    if next == rd32(this) {
                        blacken(next);
                        return next;
                    }
                    cur = next;
                }
            }
        }
    }
});
