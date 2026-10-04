// original: 0x00ba0d80 RESET_VISIBLE_PED_DAMAGE
/// Script native `RESET_VISIBLE_PED_DAMAGE` (hash 0x2A7247EF).
///
/// Forwards one script argument (a ped handle) to the engine.
///
/// No return slot is written.
///
export!(cdecl, rw_00ba0d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
