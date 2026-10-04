// original: 0x00bd23c0 REMOVE_SCRIPT_FIRE
/// Script native `REMOVE_SCRIPT_FIRE` (hash 0x0E633C13).
///
/// Forwards one script argument (a fire handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd23c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args,)
    }
});
