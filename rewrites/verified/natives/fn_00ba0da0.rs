// original: 0x00ba0da0 SEARCH_CRITERIA_CONSIDER_PEDS_WITH_FLAG_FALSE
/// Script native `SEARCH_CRITERIA_CONSIDER_PEDS_WITH_FLAG_FALSE` (hash 0x2A860E89).
///
/// Forwards one script argument (a search-criteria handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba0da0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
