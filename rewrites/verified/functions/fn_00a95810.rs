// original: 0x00a95810 stream_close_entry_full (proposed)

/// Close a streaming entry: run its state step, unlink it, update counts.
///
/// The entry is `table[idx*3*8]` where `table` is the pointer at `this+0x00`
/// and `idx` is the first argument. Its state step runs (callee 1,
/// thiscall/1 with 3), then it is unlinked (callee 2, thiscall/0). The live
/// count at `this+0x38` is decremented; the second count at `this+0x3c` is
/// decremented unless bit 11 of the flag word at entry `+0x0e` is set; when
/// bit 4 of the flag byte is set the flag word is ANDed with 0xffef and the
/// third count at `this+0x40` is decremented. When the second argument's low
/// byte is nonzero the entry is also detached (callee 3, thiscall/1 on the
/// object with the index).
///
/// Returns the detach call's answer when it runs; otherwise 0xffef when the
/// flag-mask path ran, else the unlink answer's high half above the flag
/// byte shifted right by four (the original tests the shifted `al`).
/// Thiscall: object in ecx,
/// two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00a95810(this: u32, idx: u32, flag: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const LIVE_COUNT: u32 = 0x38;
        const COUNT2: u32 = 0x3c;
        const COUNT3: u32 = 0x40;
        const FLAGS_OFF: u32 = 0x0e;
        const STATE_ARG: u32 = 3;
        const KEEP_BIT: u16 = 11;
        const MASK_BIT: u8 = 4;
        const FLAG_MASK: u32 = 0xffef;
        const STATE_CALLEE: u32 = 1;
        const UNLINK_CALLEE: u32 = 2;
        const DETACH_CALLEE: u32 = 3;
        let elem = ((this + TABLE) as *const u32).read_unaligned()
            + idx.wrapping_mul(3).wrapping_mul(8);
        lf_checker_rt::callee_thiscall!(STATE_CALLEE, u32, elem, STATE_ARG);
        let unlink_ans = lf_checker_rt::callee_thiscall!(UNLINK_CALLEE, u32, elem);
        let c0 = (this + LIVE_COUNT) as *mut u32;
        c0.write_unaligned(c0.read_unaligned().wrapping_sub(1));
        let flagword = ((elem + FLAGS_OFF) as *const u16).read_unaligned();
        if (flagword >> KEEP_BIT) & 1 == 0 {
            let c1 = (this + COUNT2) as *mut u32;
            c1.write_unaligned(c1.read_unaligned().wrapping_sub(1));
        }
        let flagbyte = (flagword & 0xff) as u8;
        let mut ret = if (flagbyte >> MASK_BIT) & 1 != 0 {
            let flags = (elem + FLAGS_OFF) as *mut u16;
            flags.write_unaligned(flagword & (FLAG_MASK as u16));
            let c2 = (this + COUNT3) as *mut u32;
            c2.write_unaligned(c2.read_unaligned().wrapping_sub(1));
            FLAG_MASK
        } else {
            // The original shifts `al` right by 4 before testing its low bit,
            // so the returned low byte is the flag byte's high nibble.
            (unlink_ans & 0xffff0000) | ((flagbyte >> MASK_BIT) as u32)
        };
        if (flag & 0xff) as u8 != 0 {
            ret = lf_checker_rt::callee_thiscall!(DETACH_CALLEE, u32, this, idx);
        }
        ret
    }
});
