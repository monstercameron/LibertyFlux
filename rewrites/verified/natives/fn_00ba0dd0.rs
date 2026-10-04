// original: 0x00ba0dd0 SEARCH_CRITERIA_REJECT_PEDS_WITH_FLAG_TRUE
/// Script native `SEARCH_CRITERIA_REJECT_PEDS_WITH_FLAG_TRUE` (hash 0x27211B1A).
///
/// Forwards one script argument (a flag index) to the engine.
///
/// No return slot is written.
///
export!(cdecl, rw_00ba0dd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
