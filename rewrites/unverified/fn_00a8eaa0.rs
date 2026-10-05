// original: 0x00A8EAA0 pool_datum_bounded_via_helper (proposed)

/// Fetch a datum word through the helper when the index is below its limit.
///
/// Same pool-table lookup as the sibling helper callers; the datum limit at
/// page `+8` is compared against `n` (unsigned): `n` at or above the limit
/// returns 0 with no call, otherwise the helper runs with `n` and the word
/// at result `+4` is returned.
///
/// Original: thiscall, one stack word, returns u32 in EAX. One outgoing
/// call (thiscall, one stack word), skipped on the early exit.
lf_checker_rt::export!(thiscall, rw_00A8EAA0(this: u32, n: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const LIMIT_OFF: u32 = 8;
        const WORD_OFF: u32 = 4;
        const HELPER: u32 = 1;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        let limit = ((page + LIMIT_OFF) as *const u32).read_unaligned();
        if n >= limit {
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, page, n);
        ((r + WORD_OFF) as *const u32).read_unaligned()
    }
});
