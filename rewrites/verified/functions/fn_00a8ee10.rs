// original: 0x00A8EE10 pool_two_field_fetch (proposed)

/// Fetch two fields through chained pool helpers into two outputs.
///
/// The first helper runs on the pool page with `a`; its result `+0x10`,
/// plus `b`, indexes the global word table. That signed word becomes the
/// second helper's argument (same page), and the words at its result
/// `+0x40` and `+0x44` are stored to `*out0` and `*out1`.
///
/// Original: thiscall, four stack words, no return value. Two outgoing
/// calls (thiscall, one stack word each).
lf_checker_rt::export!(thiscall, rw_00A8EE10(this: u32, a: u32, b: u32, out0: u32, out1: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const INDEX_OFF: u32 = 0x10;
        const WORD_TABLE: u32 = 0x16556d8;
        const FIELD0_OFF: u32 = 0x40;
        const FIELD1_OFF: u32 = 0x44;
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
        let f0 = ((r2 + FIELD0_OFF) as *const u32).read_unaligned();
        (out0 as *mut u32).write_unaligned(f0);
        let f1 = ((r2 + FIELD1_OFF) as *const u32).read_unaligned();
        (out1 as *mut u32).write_unaligned(f1);
        0
    }
});
