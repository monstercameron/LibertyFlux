// original: 0x00bc81a0 SUPPRESS_CAR_MODEL
/// Script native `SUPPRESS_CAR_MODEL` (hash 0x768F640F).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bc81a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
