// original: 0x00624e20 net_list_notify (proposed)

/// Notify two gamer lists of a network event, unlinking retired nodes.
///
/// `this` is the session object holding two list heads: words at `+HEAD1`
/// (head), `+HEAD1+4` (tail), `+HEAD1+8` (count) for the first list and the
/// same shape at `+HEAD2` for the second, plus a reentrancy byte at `+BUSY`
/// and a container pointer at `+0`. `arg` is an opaque event word passed to
/// interested nodes. No return value.
///
/// First list: set `BUSY`, then walk the chain from `HEAD1` via each node's
/// `NEXT` link. A node whose signed `STATE` is below 1 gets its slot-2 hook
/// called; one at 1 or above 2 gets its slot-3 hook called with `arg`; one
/// exactly at 2 is unlinked from the head and, when its `FLAGS` low bit is
/// set, gets its slot-0 hook called with 0 followed by the container's
/// slot-3 hook with the node. The walk uses the next link saved before any
/// call, so unlinking never disturbs it.
///
/// Second list: while the head at `HEAD2` is non-null (re-read after every
/// pass, because the unlink call empties it), run the same three hooks by
/// state, unlink at state 2, then re-read the head; a node still present
/// with state 0 gets one more slot-2 call. The pass repeats only while the
/// head's state is below 1 or exactly 2, otherwise the function is done.
/// Clear `BUSY` on exit.
///
/// Hook answers are ignored; only their order and arguments matter. The
/// unlink helper removes the node from its doubly linked list (head, tail,
/// count, both links). Original: 0x00624e20 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00624e20(this: u32, arg: u32) -> u32 {
    unsafe {
        const HEAD2: u32 = 0x32d4;
        const HEAD1: u32 = 0x32e0;
        const BUSY: u32 = 0x32ec;
        const STATE: u32 = 0x0c;
        const NEXT: u32 = 0x64;
        const FLAGS: u32 = 0x6c;
        const SLOT_CLEAR: u32 = 0x00;
        const SLOT_TOUCH: u32 = 0x08;
        const SLOT_EVENT: u32 = 0x0c;
        const UNLINK_FIRST: u32 = 4;
        const UNLINK_SECOND: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn hook0(node: u32, v: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(node) + SLOT_CLEAR) as usize);
                slot(node, v);
            }
        }
        #[inline(always)]
        unsafe fn hook1(node: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(node) + SLOT_TOUCH) as usize);
                slot(node);
            }
        }
        #[inline(always)]
        unsafe fn hook2(node: u32, v: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(node) + SLOT_EVENT) as usize);
                slot(node, v);
            }
        }

        wr8(this.wrapping_add(BUSY), 1);
        // First list: saved-next walk; the link slot mirrors the original's
        // local, including its (never odd in practice) tag check.
        let head1 = this.wrapping_add(HEAD1);
        let mut link = head1;
        let mut node = rd32(head1);
        if node != 0 {
            let mut next = rd32(node.wrapping_add(NEXT));
            loop {
                let state = rd32(node.wrapping_add(STATE)) as i32;
                if state < 1 {
                    hook1(node);
                }
                let state = rd32(node.wrapping_add(STATE)) as i32;
                if state >= 1 && state != 2 {
                    hook2(node, arg);
                }
                if rd32(node.wrapping_add(STATE)) == 2 {
                    lf_checker_rt::callee_thiscall!(UNLINK_FIRST, u32, head1, node);
                    if rd8(node.wrapping_add(FLAGS)) & 1 != 0 {
                        hook0(node, 0);
                        hook2(rd32(this), node);
                    }
                }
                node = next;
                if next != 0 {
                    next = rd32(next.wrapping_add(NEXT));
                } else {
                    let cont = link;
                    if cont & 0xffff_fffe != 0 && cont & 1 != 0 {
                        link = cont & 0xffff_fffe;
                        next = rd32(link);
                    }
                }
                if node == 0 {
                    break;
                }
            }
        }
        // Second list: re-read the head after every pass.
        loop {
            let head_now =
                if rd32(this.wrapping_add(HEAD2 + 8)) == 0 { 0 } else { rd32(this.wrapping_add(HEAD2)) };
            if head_now == 0 {
                break;
            }
            let cur = head_now;
            if (rd32(cur.wrapping_add(STATE)) as i32) < 1 {
                hook1(cur);
            }
            let state = rd32(cur.wrapping_add(STATE)) as i32;
            if state >= 1 && state != 2 {
                hook2(cur, arg);
            }
            if rd32(cur.wrapping_add(STATE)) == 2 && rd8(this.wrapping_add(BUSY)) != 0 {
                lf_checker_rt::callee_thiscall!(UNLINK_SECOND, u32, this.wrapping_add(HEAD2), cur);
                if rd8(cur.wrapping_add(FLAGS)) & 1 != 0 {
                    hook0(cur, 0);
                    hook2(rd32(this), cur);
                }
            }
            let again =
                if rd32(this.wrapping_add(HEAD2 + 8)) == 0 { 0 } else { rd32(this.wrapping_add(HEAD2)) };
            if again == 0 {
                break;
            }
            if rd32(again.wrapping_add(STATE)) == 0 {
                hook1(again);
            }
            let restate = rd32(again.wrapping_add(STATE)) as i32;
            if restate >= 1 && restate != 2 {
                break;
            }
        }
        wr8(this.wrapping_add(BUSY), 0);
        0
    }
});
