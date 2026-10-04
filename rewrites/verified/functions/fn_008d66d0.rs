// original: 0x008d66d0 select_by_tag_walk
/// Tag-gated list selection over a linked node chain.
///
/// Each node holds a tag at +4, a priority word at +8 (priority is bits
/// 3:1) and the next link at +12. A walk scans from the head while each new
/// priority does not exceed the running one (an ascending step exits once
/// the running priority has reached 2), matching nodes by tag.
///
/// When flag bit 2 at `obj+0x26c` is set, an optional out-word receives
/// `obj+0xb30`, then the chain is walked for tag 0x2e2 and again for
/// 0x2bf: either match returns 3, otherwise 1. When the flag is clear, the
/// chain is walked once for 0x2de: no match writes 0 to the out-word (when
/// given) and returns 0; a match queries two object helpers and returns 2
/// when the second answer falls in 15..=26, else 0 (rewriting the out-word
/// with 0). Only AL carries the result; upper EAX bits are leftovers.
lf_checker_rt::export!(cdecl, rb120_fn6(obj: u32, out: u32) -> u8 {
    const CAL_LOOKUP: u32 = 1; // object tag lookup (thiscall/2)
    const CAL_RESOLVE: u32 = 2; // object tag resolve (thiscall/2)
    const TAG_FIRST: u32 = 0x2E2;
    const TAG_SECOND: u32 = 0x2BF;
    const TAG_PLAIN: u32 = 0x2DE;

    #[inline(always)]
    fn prio(node: u32) -> u32 {
        unsafe { ((node as *const u32).add(2).read_unaligned() >> 1) & 7 }
    }
    #[inline(always)]
    fn tag_is(node: u32, tag: u32) -> bool {
        unsafe { (node as *const u32).add(1).read_unaligned() == tag }
    }
    #[inline(always)]
    fn next(node: u32) -> u32 {
        unsafe { (node as *const u32).add(3).read_unaligned() }
    }
    fn walk(mut node: u32, tag: u32) -> bool {
        if node == 0 {
            return false;
        }
        let mut run = prio(node);
        loop {
            let p = prio(node);
            let check = run >= p || run < 2;
            if check {
                run = p;
                if tag_is(node, tag) {
                    return true;
                }
            } else {
                return false;
            }
            node = next(node);
            if node == 0 {
                return false;
            }
        }
    }

    unsafe {
        let flag = ((obj + 0x26C) as *const u8).read_unaligned();
        let base = (obj as *const u32).add(0x224 / 4).read_unaligned();
        let slot = base + 0x2E0;
        let head = (slot as *const u32).read_unaligned();
        if flag & 4 != 0 {
            if out != 0 {
                let v = (obj as *const u32).add(0xB30 / 4).read_unaligned();
                (out as *mut u32).write_unaligned(v);
            }
            if walk(head, TAG_FIRST) || walk(head, TAG_SECOND) {
                3
            } else {
                1
            }
        } else if walk(head, TAG_PLAIN) {
            if out != 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, slot, TAG_PLAIN, 5u32);
                (out as *mut u32).write_unaligned(r);
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, slot, TAG_PLAIN, 5u32);
            if r.wrapping_sub(15) <= 11 {
                2
            } else {
                if out != 0 {
                    (out as *mut u32).write_unaligned(0);
                }
                0
            }
        } else {
            if out != 0 {
                (out as *mut u32).write_unaligned(0);
            }
            0
        }
    }
});
