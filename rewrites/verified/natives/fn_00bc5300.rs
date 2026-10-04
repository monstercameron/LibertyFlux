// original: 0x00bc5300 CLEAR_CAR_LAST_DAMAGE_ENTITY
/// Script native `CLEAR_CAR_LAST_DAMAGE_ENTITY` (hash 0x4D6665F7).
///
/// Forwards one script argument (a vehicle handle) to the engine.
///
/// No return slot is written.
///
export!(cdecl, rw_00bc5300(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
