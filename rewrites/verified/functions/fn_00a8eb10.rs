// original: 0x00A8EB10 pool_word_table_via_helper (proposed)

/// Look up a signed word from the global word table via the pool helper.
///
/// The helper runs on the pool page with `a`; its result `+0x10`, plus `b`,
/// indexes the global 16-bit table whose entry is returned sign-extended.
///
/// Original: thiscall, two stack words, returns u32 in EAX. One outgoing
/// call (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00A8EB10(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const INDEX_OFF: u32 = 0x10;
        const WORD_TABLE: u32 = 0x16556d8;
        const HELPER: u32 = 1;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, page, a);
        let s = ((r + INDEX_OFF) as *const u32)
            .read_unaligned()
            .wrapping_add(b);
        ((lf_checker_rt::relocated(WORD_TABLE).wrapping_add(s.wrapping_mul(2))
            as *const i16)
            .read_unaligned()) as i32 as u32
    }
});
