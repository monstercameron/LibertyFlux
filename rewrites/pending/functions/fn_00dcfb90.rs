// original: 0x00dcfb90 rb_tree_erase_fixup
use lf_checker_rt::{callee_thiscall, export};

#[inline(always)]
unsafe fn ld(a: u32) -> u32 {
    *(a as *const u32)
}

#[inline(always)]
unsafe fn st(a: u32, v: u32) {
    *(a as *mut u32) = v;
}

/// Swap the colour bit (bit 0) between two child-pointer words.
///
/// The original does `t = (byte[a] ^ [b]) & 1; [b] ^= t`: when the two
/// words hold different colours it flips bit 0 of `[b]`, which together
/// with the caller clearing bit 0 of `[a]` afterwards exchanges the two
/// colours. Replicated exactly, including the byte-width read of `[a]`.
#[inline(always)]
unsafe fn swap_colour_bit(a_word: u32, b_word: u32) {
    let t = ((ld(a_word) & 0xFF) ^ ld(b_word)) & 1;
    st(b_word, ld(b_word) ^ t);
}


// ---------------------------------------------------------------------------
// fn2 0x00DCFB90: red-black erase fixup over a game tree.
//
// Head: [+0] root pointer. Node: [+8] left-child pointer with the colour in
// bit 0 (1 = red), [+0xC] right-child pointer, [+0x10] parent pointer.
// Walks from the erased node towards the root while the current node is
// black, recolouring and rotating (rotations are intercepted callees:
// id 1 and id 2); finishes by blackening the node it stops at and
// returning it. thiscall(head, node).
// ---------------------------------------------------------------------------
export!(thiscall, rw_b187_f2(head: u32, x0: u32) -> u32 {
    unsafe {
        if x0 == ld(head) {
            st(x0.wrapping_add(8), ld(x0.wrapping_add(8)) & 0xFFFF_FFFE);
            return x0;
        }
        let mut x = x0;
        loop {
            if ld(x.wrapping_add(8)) & 1 != 0 {
                st(x.wrapping_add(8), ld(x.wrapping_add(8)) & 0xFFFF_FFFE);
                return x;
            }
            let p = ld(x.wrapping_add(0x10));
            let next: u32;
            if x == ld(p.wrapping_add(0xC)) {
                // X is the right child; sibling S is the left child.
                let mut s = ld(p.wrapping_add(8)) & 0xFFFF_FFFE;
                if s == 0 {
                    next = p;
                } else {
                    if ld(s.wrapping_add(8)) & 1 != 0 {
                        // Red sibling: recolour and rotate inline (mirror
                        // rotation: S takes P's place, S.right becomes
                        // P.left), then continue with the new sibling.
                        st(s.wrapping_add(8), ld(s.wrapping_add(8)) & 0xFFFF_FFFE);
                        st(p.wrapping_add(8), ld(p.wrapping_add(8)) | 1);
                        let sr = ld(s.wrapping_add(0xC));
                        st(p.wrapping_add(8), (ld(p.wrapping_add(8)) & 1) | sr);
                        if sr != 0 {
                            st(sr.wrapping_add(0x10), p);
                        }
                        let g = ld(p.wrapping_add(0x10));
                        st(s.wrapping_add(0x10), g);
                        if p == ld(head) {
                            st(head, s);
                        } else if p == ld(g.wrapping_add(0xC)) {
                            st(g.wrapping_add(0xC), s);
                        } else {
                            st(g.wrapping_add(8), (ld(g.wrapping_add(8)) & 1) | s);
                        }
                        st(s.wrapping_add(0xC), p);
                        st(p.wrapping_add(0x10), s);
                        s = ld(p.wrapping_add(8)) & 0xFFFF_FFFE;
                    }
                    if s == 0 {
                        next = p;
                    } else {
                        // S is black here (or was just made black).
                        let sr = ld(s.wrapping_add(0xC));
                        let sr_red = sr != 0 && ld(sr.wrapping_add(8)) & 1 != 0;
                        let sl = ld(s.wrapping_add(8));
                        let sl_node = sl & 0xFFFF_FFFE;
                        let sl_red = sl_node != 0 && ld(sl_node.wrapping_add(8)) & 1 != 0;
                        if !sr_red && (sl_node == 0 || !sl_red) {
                            // Both nephews black/missing: redden S, move up.
                            st(s.wrapping_add(8), sl | 1);
                            next = p;
                        } else {
                            if sl_node == 0 || !sl_red {
                                // Far nephew black: recolour + first rotation.
                                st(sr.wrapping_add(8), ld(sr.wrapping_add(8)) & 0xFFFF_FFFE);
                                st(s.wrapping_add(8), ld(s.wrapping_add(8)) | 1);
                                let _ = callee_thiscall!(1, u32, head, s);
                                s = ld(p.wrapping_add(8)) & 0xFFFF_FFFE;
                            }
                            // Second rotation with the colour exchange.
                            swap_colour_bit(p.wrapping_add(8), s.wrapping_add(8));
                            st(p.wrapping_add(8), ld(p.wrapping_add(8)) & 0xFFFF_FFFE);
                            let nl = ld(s.wrapping_add(8)) & 0xFFFF_FFFE;
                            st(nl.wrapping_add(8), ld(nl.wrapping_add(8)) & 0xFFFF_FFFE);
                            let _ = callee_thiscall!(2, u32, head, p);
                            next = ld(head);
                        }
                    }
                }
            } else {
                // X is the left child; sibling S is the right child.
                let mut s = ld(p.wrapping_add(0xC));
                if s == 0 {
                    next = p;
                } else {
                    if ld(s.wrapping_add(8)) & 1 != 0 {
                        // Red sibling: recolour and rotate P down.
                        st(s.wrapping_add(8), ld(s.wrapping_add(8)) & 0xFFFF_FFFE);
                        st(p.wrapping_add(8), ld(p.wrapping_add(8)) | 1);
                        let _ = callee_thiscall!(1, u32, head, p);
                        s = ld(p.wrapping_add(0xC));
                    }
                    if s == 0 {
                        next = p;
                    } else {
                        let sl = ld(s.wrapping_add(8));
                        let sl_node = sl & 0xFFFF_FFFE;
                        let sl_red = sl_node != 0 && ld(sl_node.wrapping_add(8)) & 1 != 0;
                        let sr = ld(s.wrapping_add(0xC));
                        let sr_red = sr != 0 && ld(sr.wrapping_add(8)) & 1 != 0;
                        if !sl_red && (sr == 0 || !sr_red) {
                            // Both nephews black/missing: redden S, move up.
                            st(s.wrapping_add(8), sl | 1);
                            next = p;
                        } else {
                            if sr == 0 || !sr_red {
                                // Far nephew black: recolour + first rotation.
                                st(sl_node.wrapping_add(8), ld(sl_node.wrapping_add(8)) & 0xFFFF_FFFE);
                                st(s.wrapping_add(8), ld(s.wrapping_add(8)) | 1);
                                let _ = callee_thiscall!(2, u32, head, s);
                                s = ld(p.wrapping_add(0xC));
                            }
                            // Second rotation with the colour exchange.
                            swap_colour_bit(p.wrapping_add(8), s.wrapping_add(8));
                            st(p.wrapping_add(8), ld(p.wrapping_add(8)) & 0xFFFF_FFFE);
                            let nr = ld(s.wrapping_add(0xC));
                            st(nr.wrapping_add(8), ld(nr.wrapping_add(8)) & 0xFFFF_FFFE);
                            let _ = callee_thiscall!(1, u32, head, p);
                            next = ld(head);
                        }
                    }
                }
            }
            if next == ld(head) {
                st(next.wrapping_add(8), ld(next.wrapping_add(8)) & 0xFFFF_FFFE);
                return next;
            }
            x = next;
        }
    }
});
