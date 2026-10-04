// original: 0x00bd8b70 NETWORK_RESULT_MATCHES_SEARCH_CRITERIA
/// Script native `NETWORK_RESULT_MATCHES_SEARCH_CRITERIA` (hash 0x767F1E44).
///
/// Forwards one script argument (a network search-result index) to the
/// engine and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00bd8b70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
