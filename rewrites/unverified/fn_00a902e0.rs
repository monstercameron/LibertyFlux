// original: 0x00a902e0 stream_sync_range_then_extra (proposed)

/// Run a callee over a range of slots, then once more past the end.
///
/// `this+0xe4` is the slot array, `this+0xe8` the signed slot count and the
/// argument points at a context whose `+0x38` is a second count. Each slot
/// whose index differs from the second count (and while the word at
/// `this+0xea` is nonzero) is visited (callee 1, thiscall/1 with the
/// context); slots are 0xa0 bytes starting 8 into the array. When the
/// `+0xea` word exceeds the second count, the slot at index
/// `second*0xa0 + 8` is visited the same way, and when that call's low byte
/// is nonzero a finaliser runs (callee 2, thiscall/1 with the context) and
/// the context's `+0x34` object (unless null) is cleared in bit 27 of its
/// `+0x24` word and stamped 9 at `+0x41`.
///
/// Returns the `+0xea` word when no extra slot runs, the visit answer when
/// the finaliser is skipped, else the `+0x34` object (possibly null).
/// Thiscall: object in ecx, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a902e0(this: u32, ctx: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0xe4;
        const COUNT: u32 = 0xe8;
        const LIMIT: u32 = 0xea;
        const CTX_COUNT: u32 = 0x38;
        const CTX_OBJ: u32 = 0x34;
        const SLOT_STRIDE: u32 = 0xa0;
        const SLOT_BASE: u32 = 8;
        const CLEAR_MASK: u32 = 0xf7ffffff;
        const STAMP_OFF: u32 = 0x41;
        const STAMP: u8 = 9;
        const VISIT_CALLEE: u32 = 1;
        const FINISH_CALLEE: u32 = 2;
        let n1 = ((this + COUNT) as *const u16).read_unaligned() as u32;
        let c2 = ((ctx + CTX_COUNT) as *const u32).read_unaligned();
        // The original keeps its loop bound in eax: it still holds the slot
        // count when the first pass skips its visit, and is reloaded from
        // the spilled count after every visit. (Entry eax is dead.)
        let mut bound = n1;
        if (n1 as i32) > 0 {
            let mut idx = 0u32;
            let mut off = 0u32;
            loop {
                let live = ((ctx + CTX_COUNT) as *const u32).read_unaligned();
                let gate = ((this + LIMIT) as *const u16).read_unaligned();
                if live != idx && gate != 0 {
                    let slot = ((this + SLOTS) as *const u32).read_unaligned()
                        + SLOT_BASE
                        + off;
                    lf_checker_rt::callee_thiscall!(VISIT_CALLEE, u32, slot, ctx);
                    bound = n1;
                }
                idx = idx.wrapping_add(1);
                off = off.wrapping_add(SLOT_STRIDE);
                if !((idx as i32) < (bound as i32)) {
                    break;
                }
            }
        }
        let limit = ((this + LIMIT) as *const u16).read_unaligned() as u32;
        if (limit as i32) <= (c2 as i32) {
            return limit;
        }
        let extra = ((this + SLOTS) as *const u32).read_unaligned()
            + SLOT_BASE
            + c2.wrapping_mul(5).wrapping_mul(32);
        let ans = lf_checker_rt::callee_thiscall!(VISIT_CALLEE, u32, extra, ctx);
        if (ans & 0xff) == 0 {
            return ans;
        }
        lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, extra, ctx);
        let obj = ((ctx + CTX_OBJ) as *const u32).read_unaligned();
        if obj == 0 {
            return 0;
        }
        let flags = (obj + 0x24) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & CLEAR_MASK);
        ((obj + STAMP_OFF) as *mut u8).write(STAMP);
        ((ctx + CTX_OBJ) as *const u32).read_unaligned()
    }
});
