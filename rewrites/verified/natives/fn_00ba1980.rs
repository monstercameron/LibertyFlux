// original: 0x00ba1980 SET_CHAR_RANDOM_COMPONENT_VARIATION
/// Script native `SET_CHAR_RANDOM_COMPONENT_VARIATION` (hash 0x47D9437C).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba1980(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
