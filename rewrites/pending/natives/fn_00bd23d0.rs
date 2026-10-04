// original: 0x00bd23d0 SET_MAX_FIRE_GENERATIONS
/// Script native `SET_MAX_FIRE_GENERATIONS` (hash 0x03BA036B).
///
/// Forwards one script argument (a fire count limit) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bd23d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
