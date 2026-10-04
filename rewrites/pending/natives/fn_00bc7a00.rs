// original: 0x00bc7a00 SET_HELI_BLADES_FULL_SPEED
/// Script native `SET_HELI_BLADES_FULL_SPEED` (hash 0x557C3641).
///
/// Forwards one script argument (a helicopter handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc7a00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
