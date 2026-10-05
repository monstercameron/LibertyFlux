// original: 0x00A8EE90 pool_datum_field_via_helper (proposed)

/// Fetch the field at result `+0x40` through the pool helper call.
///
/// Same pool-table lookup as the sibling helper callers; the helper runs on
/// the page with `n` and the word at result `+0x40` is returned.
///
/// Original: thiscall, one stack word, returns u32 in EAX. One outgoing
/// call (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00A8EE90(this: u32, n: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const FIELD_OFF: u32 = 0x40;
        const HELPER: u32 = 1;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, page, n);
        ((r + FIELD_OFF) as *const u32).read_unaligned()
    }
});
