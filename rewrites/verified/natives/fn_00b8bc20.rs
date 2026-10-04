// original: 0x00b8bc20 ADD_TO_PREVIOUS_BRIEF
/// Script native `ADD_TO_PREVIOUS_BRIEF` (hash 0x446E6515).
///
/// Forwards one script argument (a text-label reference) to the engine. No return slot is written.
export!(cdecl, rw_00b8bc20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
