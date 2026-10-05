// original: 0x00a94020 stream_clear_entry_flags (proposed)

/// Clear `mask` from entry `idx`'s flag word and retire it if now idle.
///
/// A null table base returns 0. When `mask` shares no bit with the flag
/// word at `+0x0e`, the flags are returned unchanged. Otherwise the mask
/// bits are cleared and the word is re-read: leftover bits in `0xc6` return
/// the original flags. When the entry's low two size bits at `+0x08` equal
/// 1, the retire callee and then the swap callee run, and the swap answer
/// is returned. Else a set bit 3 of the new flags returns the original
/// flags with the low byte replaced by the masked size bits (the original
/// rewrites `al` before this test); otherwise the unlink callee runs on
/// `(this, idx)` and its answer is returned.
///
/// Original: thiscall, two stack arguments (index, mask).
/// Three callees (thiscall, 0/1/1 args).
lf_checker_rt::export!(thiscall, rw_00a94020(this: u32, idx: u32, mask: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x00;
        const TABLE_AUX: u32 = 0x08;
        const ENTRY_STRIDE: u32 = 24;
        const ENT_FLAGS: u32 = 0x0e;
        const ENT_SIZE: u32 = 0x08;
        const BUSY_BITS: u32 = 0xc6;
        const RETIRE_KIND: u8 = 1;
        const LINK_BIT: u32 = 3;
        const RETIRE: u32 = 0;
        const SWAP: u32 = 1;
        const UNLINK: u32 = 2;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        let base = rd32(this.wrapping_add(TABLE_BASE));
        if base == 0 {
            return 0;
        }
        let ent = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        let flags = rd16(ent.wrapping_add(ENT_FLAGS));
        if mask & flags == 0 {
            return flags;
        }
        wr16(ent.wrapping_add(ENT_FLAGS), (flags & !mask) as u16);
        let cleared = rd16(ent.wrapping_add(ENT_FLAGS));
        if cleared & BUSY_BITS != 0 {
            return flags;
        }
        let kind = rd8(ent.wrapping_add(ENT_SIZE)) & 3;
        if kind == RETIRE_KIND {
            let _: u32 = lf_checker_rt::callee_thiscall!(RETIRE, u32, ent);
            let aux = rd32(this.wrapping_add(TABLE_AUX));
            return lf_checker_rt::callee_thiscall!(SWAP, u32, ent, aux);
        }
        if (cleared >> LINK_BIT) & 1 != 0 {
            return (flags & 0xffff_ff00) | (kind as u32);
        }
        lf_checker_rt::callee_thiscall!(UNLINK, u32, this, idx)
    }
});
