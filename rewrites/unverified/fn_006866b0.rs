// original: 0x006866b0 frame_dispatcher (proposed)

/// Dispatch a frame pair to one of several handlers by header comparison.
///
/// `inner = [this]`; `e8 = [inner+8]`. When `a1` is null the peer `a0`
/// is reloaded: a zero tag calls the null-handler callee, otherwise the
/// tag is compared against `[a0+8]` and the equal/unequal callee runs.
/// Otherwise the equal path is taken when `e8` is non-zero and matches
/// `[a0+8]`; any other case takes the alternate path. Both paths resolve a
/// context word (`[inner+4]`, else `[a0+4]`): when both are zero the
/// path's direct callee is invoked with a zero context and the function
/// returns. Otherwise the probe callee runs with
/// `(context, a1, inner, block)` where `block` is an 8-byte zeroed frame
/// buffer it fills; a zero answer byte runs the cleanup callee on the
/// block and the direct callee with the block address (skipped from the
/// call comparison), while a non-zero byte reads the block's second word
/// (zero, or forwarded through `+0x0c`) into the indirect callee followed
/// by the cleanup callee. No result.
///
/// Original: 0x006866b0 (thiscall, two stack words, no result).
lf_checker_rt::export!(thiscall, rw_006866B0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const EQ_IND: u32 = 2;
        const EQ_DIR: u32 = 3;
        const BR_IND: u32 = 4;
        const CLEANUP: u32 = 5;
        const BR_DIR: u32 = 6;
        const NULL_EQ: u32 = 7;
        const NULL_NE: u32 = 8;
        const INNER: u32 = 0x00;
        const CTX_OFF: u32 = 0x04;
        const TAG_OFF: u32 = 0x08;
        const FWD_OFF: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let inner = unsafe { rd32(this + INNER) };
        let e8 = unsafe { rd32(inner + TAG_OFF) };
        if a1 == 0 {
            // The peer is `a0` here (reloaded from the stack, never null in
            // any trial). Both null-path callees pop three stack words while
            // the caller pushes one over an 8-byte scratch reservation: the
            // second word is zeroed scratch, the third is the saved `this`.
            if e8 == 0 {
                let _n: u32 = lf_checker_rt::callee_thiscall!(NULL_NE, u32, this, a0, 0u32, this);
            } else {
                let v = unsafe { rd32(a0.wrapping_add(TAG_OFF)) };
                if e8 == v {
                    let _e: u32 =
                        lf_checker_rt::callee_thiscall!(NULL_EQ, u32, this, a0, 0u32, this);
                } else {
                    let _n: u32 =
                        lf_checker_rt::callee_thiscall!(NULL_NE, u32, this, a0, 0u32, this);
                }
            }
            return 0;
        }
        // Equal path first, exactly like the original: non-zero tag
        // matching the peer's tag (the peer is unread when zero).
        if e8 != 0 && unsafe { rd32(a0 + TAG_OFF) } == e8 {
            let mut ctx = unsafe { rd32(inner + CTX_OFF) };
            if ctx == 0 {
                ctx = unsafe { rd32(a0 + CTX_OFF) };
                if ctx == 0 {
                    let _d: u32 =
                        lf_checker_rt::callee_thiscall!(EQ_DIR, u32, this, a0, a1, 0u32);
                    return 0;
                }
            }
            let mut blk = [0u32; 2];
            let ans: u32 = lf_checker_rt::callee_stdcall!(
                PROBE,
                u32,
                blk.as_mut_ptr() as u32,
                inner,
                a1,
                ctx
            );
            if (ans as u8) == 0 {
                let _c: u32 =
                    lf_checker_rt::callee_thiscall!(CLEANUP, u32, blk.as_mut_ptr() as u32);
                let frame = blk.as_mut_ptr() as u32;
                let _d: u32 =
                    lf_checker_rt::callee_thiscall!(EQ_DIR, u32, this, a0, a1, frame);
                return 0;
            }
            let o1 = blk[1];
            let av = if o1 == 0 { 0 } else { unsafe { rd32(o1 + FWD_OFF) } };
            let _d: u32 = lf_checker_rt::callee_thiscall!(EQ_IND, u32, this, a0, ctx, av);
            let _c: u32 = lf_checker_rt::callee_thiscall!(CLEANUP, u32, blk.as_mut_ptr() as u32);
            return 0;
        }
        // Alternate path: same shape, other callees.
        let mut ctx = unsafe { rd32(inner + CTX_OFF) };
        if ctx == 0 {
            ctx = unsafe { rd32(a0 + CTX_OFF) };
            if ctx == 0 {
                let _d: u32 =
                    lf_checker_rt::callee_thiscall!(BR_DIR, u32, this, a0, a1, 0u32);
                return 0;
            }
        }
        let mut blk = [0u32; 2];
        let ans: u32 = lf_checker_rt::callee_stdcall!(
            PROBE,
            u32,
            blk.as_mut_ptr() as u32,
            inner,
            a1,
            ctx
        );
        if (ans as u8) == 0 {
            let _c: u32 = lf_checker_rt::callee_thiscall!(CLEANUP, u32, blk.as_mut_ptr() as u32);
            let frame = blk.as_mut_ptr() as u32;
            let _d: u32 =
                lf_checker_rt::callee_thiscall!(BR_DIR, u32, this, a0, a1, frame);
            return 0;
        }
        let o1 = blk[1];
        let av = if o1 == 0 { 0 } else { unsafe { rd32(o1 + FWD_OFF) } };
        let _d: u32 = lf_checker_rt::callee_thiscall!(BR_IND, u32, this, a0, ctx, av);
        let _c: u32 = lf_checker_rt::callee_thiscall!(CLEANUP, u32, blk.as_mut_ptr() as u32);
        0
    }
});
