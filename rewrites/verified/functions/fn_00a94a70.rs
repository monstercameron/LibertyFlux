// original: 0x00a94a70 stream_entry_test_mask (proposed)

/// Whether entry `idx` has none of the flag bits in `mask | 0xc6` set.
///
/// Reads the flag word at `+0x0e` of the 24-byte entry and returns 1 when
/// `(flags & (mask | 0xc6)) == 0`, else 0 (the original forms this with a
/// neg/sbb/inc sequence). Pure leaf.
///
/// Original: thiscall, two stack arguments (index, mask).
lf_checker_rt::export!(thiscall, rw_00a94a70(this: u32, idx: u32, mask: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x00;
        const ENTRY_STRIDE: u32 = 24;
        const ENT_FLAGS: u32 = 0x0e;
        const ALWAYS_TEST: u32 = 0xc6;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let base = rd32(this.wrapping_add(TABLE_BASE));
        let ent = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        if rd16(ent.wrapping_add(ENT_FLAGS)) & (mask | ALWAYS_TEST) == 0 {
            1
        } else {
            0
        }
    }
});
