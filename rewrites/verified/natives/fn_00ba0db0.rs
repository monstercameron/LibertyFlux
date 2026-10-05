// original: 0x00ba0db0 SEARCH_CRITERIA_CONSIDER_PEDS_WITH_FLAG_TRUE
/// Script native `SEARCH_CRITERIA_CONSIDER_PEDS_WITH_FLAG_TRUE` (hash 0x20EC5B84).
///
/// Forwards one script argument to the engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00ba0db0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
