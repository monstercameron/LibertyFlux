// original: 0x00A8EA50 pool_datum_word_via_helper (proposed)

/// Fetch a 16-bit datum word through the pool helper call.
///
/// The signed handle at `this+0x2E` indexes the global pool table; the page
/// at entry `+0x70` becomes the helper's object with `n` as its argument,
/// and the word at result `+0x50` is returned zero-extended.
///
/// Original: thiscall, one stack word, returns u32 in EAX. One outgoing
/// call (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00A8EA50(this: u32, n: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const WORD_OFF: u32 = 0x50;
        const HELPER: u32 = 1;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, page, n);
        ((r + WORD_OFF) as *const u16).read_unaligned() as u32
    }
});
