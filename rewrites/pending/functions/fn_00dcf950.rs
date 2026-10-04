// original: 0x00dcf950 rb_tree_erase
use lf_checker_rt::{callee_thiscall, export};

#[inline(always)]
unsafe fn ld(a: u32) -> u32 {
    *(a as *const u32)
}

#[inline(always)]
unsafe fn st(a: u32, v: u32) {
    *(a as *mut u32) = v;
}

// ---------------------------------------------------------------------------
// fn1 0x00DCF950: red-black erase of a node, with result iterator.
//
// thiscall(head, out, node). Fills the 12-byte `out` struct with the erased
// value, the node and the head; notifies callee id 1 (passing a scratch
// word, whose address is skipped in the call comparison); unlinks the node
// or transplants its successor into its place; rebalances through callee
// id 2 (the fixup, fn2); clears the erased links, decrements head[+4] and
// returns `out`. Node layout is the same as in fn2.
// ---------------------------------------------------------------------------
export!(thiscall, rw_b187_f1(head: u32, out: u32, nd: u32) -> u32 {
    unsafe {
        st(out, 0);
        st(out.wrapping_add(4), 0);
        st(out.wrapping_add(8), head);
        if nd != 0 {
            st(out, ld(nd.wrapping_add(0x14)));
            st(out.wrapping_add(4), nd);
        } else {
            st(out.wrapping_add(4), 0);
        }
        // Original passes a pointer to its own frame scratch; the address
        // is skipped in the comparison, so any scratch word will do.
        let mut scratch: u32 = 0;
        let scratch_addr = core::ptr::addr_of_mut!(scratch) as u32;
        let _ = callee_thiscall!(1, u32, out, scratch_addr);
        // Find the node to splice out: E.
        let mut e = nd;
        if ld(nd.wrapping_add(0xC)) != 0 && ld(nd.wrapping_add(8)) & 0xFFFF_FFFE != 0 {
            if ld(head.wrapping_add(4)) & 1 != 0 {
                // Descend the +0xC chain from the masked left child.
                e = ld(nd.wrapping_add(8)) & 0xFFFF_FFFE;
                let mut c = ld(e.wrapping_add(0xC));
                if c != 0 {
                    loop {
                        e = c;
                        c = ld(e.wrapping_add(0xC));
                        if c == 0 {
                            break;
                        }
                    }
                }
            } else {
                // Descend the masked +8 chain from the right child.
                e = ld(nd.wrapping_add(0xC));
                loop {
                    let c = ld(e.wrapping_add(8));
                    if c & 0xFFFF_FFFE == 0 {
                        break;
                    }
                    e = c;
                    e &= 0xFFFF_FFFE;
                }
            }
        }
        // Splice E out, promoting its single child D (if any).
        let mut d = ld(e.wrapping_add(0xC));
        if d == 0 {
            d = ld(e.wrapping_add(8)) & 0xFFFF_FFFE;
            if d != 0 {
                st(d.wrapping_add(0x10), ld(e.wrapping_add(0x10)));
            }
        } else {
            st(d.wrapping_add(0x10), ld(e.wrapping_add(0x10)));
        }
        let ep = ld(e.wrapping_add(0x10));
        if ep == 0 {
            st(head, d);
        } else if ld(ep.wrapping_add(0xC)) == e {
            st(ep.wrapping_add(0xC), d);
        } else {
            st(ep.wrapping_add(8), (ld(ep.wrapping_add(8)) & 1) | d);
        }
        // Unless E was already the erased node, transplant E into its place.
        if nd != e {
            if nd == ld(head) {
                st(head, e);
            } else {
                let np = ld(nd.wrapping_add(0x10));
                if ld(np.wrapping_add(0xC)) == nd {
                    st(np.wrapping_add(0xC), e);
                } else {
                    st(np.wrapping_add(8), (ld(np.wrapping_add(8)) & 1) | e);
                }
            }
            st(e.wrapping_add(0x10), ld(nd.wrapping_add(0x10)));
            st(e.wrapping_add(0xC), ld(nd.wrapping_add(0xC)));
            // Two-stage colour merge, replicated literally: first take the
            // erased word with E's colour bit, then re-take it with the
            // erased low byte's bit.
            let mut m = ld(nd.wrapping_add(8));
            m ^= ld(e.wrapping_add(8));
            m &= 1;
            m ^= ld(nd.wrapping_add(8));
            st(e.wrapping_add(8), m);
            let mut m2 = ld(nd.wrapping_add(8)) & 0xFF;
            m2 ^= m;
            m2 &= 1;
            m2 ^= m;
            st(e.wrapping_add(8), m2);
            let nl = ld(nd.wrapping_add(0xC));
            if nl != 0 {
                st(nl.wrapping_add(0x10), e);
            }
            let nr = ld(nd.wrapping_add(8));
            if nr & 0xFFFF_FFFE != 0 {
                st((nr & 0xFFFF_FFFE).wrapping_add(0x10), e);
            }
        }
        // Rebalance when a black node kept a child.
        if d != 0 && ld(e.wrapping_add(8)) & 1 == 0 {
            let _ = callee_thiscall!(2, u32, head, d);
        }
        st(nd.wrapping_add(8), 0);
        st(nd.wrapping_add(0xC), 0);
        st(nd.wrapping_add(0x10), 0);
        st(head.wrapping_add(4), ld(head.wrapping_add(4)).wrapping_sub(1));
        out
    }
});
