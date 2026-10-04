// original: 0x00b8bc30 ADD_TO_PREVIOUS_BRIEF_WITH_UNDERSCORE
/// Script native `ADD_TO_PREVIOUS_BRIEF_WITH_UNDERSCORE` (hash 0x3D0A71A2).
///
/// Forwards one script argument (a text label) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8bc30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
