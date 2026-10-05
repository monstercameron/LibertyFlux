// original: 0x00A8F6A0 pool_flag_bit4_via_helpers (proposed)

/// Test bit 4 of the flag byte behind chained pool helpers.
///
/// Same two-helper chain as its bit-6 sibling: the first helper's result
/// `+0x10`, plus `b`, indexes the global word table, and the signed word
/// feeds the second helper. Returns bit 4 of the byte at its result
/// `+0x4C` (value 1 or 0); only the low byte of the return is set.
///
/// Original: thiscall, two stack words, low byte in AL. Two outgoing
/// calls (thiscall, one stack word each).
lf_checker_rt::export!(thiscall, rw_00A8F6A0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const INDEX_OFF: u32 = 0x10;
        const WORD_TABLE: u32 = 0x16556d8;
        const FLAG_OFF: u32 = 0x4c;
        const BIT: u32 = 4;
        const HELPER1: u32 = 1;
        const HELPER2: u32 = 2;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        let r1: u32 = lf_checker_rt::callee_thiscall!(HELPER1, u32, page, a);
        let s = ((r1 + INDEX_OFF) as *const u32)
            .read_unaligned()
            .wrapping_add(b);
        let w = ((lf_checker_rt::relocated(WORD_TABLE).wrapping_add(s.wrapping_mul(2))
            as *const i16)
            .read_unaligned()) as i32 as u32;
        let r2: u32 = lf_checker_rt::callee_thiscall!(HELPER2, u32, page, w);
        ((((r2 + FLAG_OFF) as *const u8).read() >> BIT) & 1) as u32
    }
});
