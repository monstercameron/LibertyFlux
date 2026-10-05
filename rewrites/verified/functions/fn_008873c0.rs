// original: 0x008873C0 stream_slot_resolve (proposed)

/// Resolve a stream handle to its slot and refresh the slot's stamp.
///
/// Splits the handle into bank (`>> 8 & 0xff`, times 0x6f40) and index
/// (`& 0xff`); when the presence flag for the pair in the bank table is
/// clear the function faults reading a null slot (both sides fault alike).
/// Otherwise it looks up the slot's class (callee 1), resolves the object
/// (callee 2), marks the class word, and stamps the slot with the object's
/// sequence divided by 1000 (multiply-and-shift) while setting its live
/// bit. The answer is the divided sequence.
///
/// Original: 0x008873C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_008873C0(this: u32, handle: u32) -> u32 {
    unsafe {
        const BANK_TABLE_GLOBAL: u32 = 0x0115_d988;
        const SEQ_BASE_GLOBAL: u32 = 0x0115_a520;
        const CLASS_TABLE_FILE_VA: u32 = 0x0115_d8a0;
        const BANK_STRIDE: u32 = 0x6f40;
        const SLOT_STRIDE: u32 = 0x70;
        const SLOT_BIAS: u32 = 0xc0;
        const CLASS_BYTE: u32 = 0x6e;
        const STAMP_BYTE: u32 = 0x6d;
        const LIVE_BYTE: u32 = 0x18;
        const LIVE_BIT: u8 = 0x40;
        const LOOKUP: u32 = 1;
        const RESOLVE: u32 = 2;
        let bank = (handle >> 8) & 0xff;
        let index = handle & 0xff;
        let bank_off = bank.wrapping_mul(BANK_STRIDE);
        let table =
            (lf_checker_rt::relocated(BANK_TABLE_GLOBAL) as *const u32)
                .read_unaligned();
        let slot = if ((table.wrapping_add(bank_off).wrapping_add(index))
            as *const u8)
            .read()
            & 1
            == 0
        {
            0
        } else {
            table
                .wrapping_add(SLOT_BIAS)
                .wrapping_add(index.wrapping_mul(SLOT_STRIDE))
                .wrapping_add(bank_off)
        };
        let class = ((slot + CLASS_BYTE) as *const u8).read() as u32;
        let classes = lf_checker_rt::relocated(CLASS_TABLE_FILE_VA);
        let c = lf_checker_rt::callee_thiscall!(LOOKUP, u32, classes, class, slot);
        let cw = c
            .wrapping_add(0x2a6)
            .wrapping_add(class.wrapping_mul(0x37a))
            .wrapping_mul(32)
            .wrapping_add(table);
        let obj = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, slot);
        let w = ((cw + 0x10) as *const u32).read_unaligned();
        ((cw + 0x10) as *mut u32).write_unaligned(w & 0xffff_fffd | 1);
        let seq_base = (lf_checker_rt::relocated(SEQ_BASE_GLOBAL) as *const u32)
            .read_unaligned();
        let d = obj.wrapping_sub(
            (seq_base as *const u32).read_unaligned());
        // Divide by 1000 with the original's signed multiply-and-shift.
        let hi = ((0x67b2_3a55i64 * d as i32 as i64) >> 32) as i32;
        let q = hi >> 9;
        let r = q.wrapping_add(((q as u32) >> 31) as i32) as u32;
        ((slot + STAMP_BYTE) as *mut u8).write(r as u8);
        let lb = ((slot + LIVE_BYTE) as *const u8).read();
        ((slot + LIVE_BYTE) as *mut u8).write(lb | LIVE_BIT);
        r
    }
});
