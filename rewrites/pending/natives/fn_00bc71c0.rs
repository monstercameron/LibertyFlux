// original: 0x00bc71c0 REMOVE_CAR_RECORDING
/// Script native `REMOVE_CAR_RECORDING` (hash 0x484964FE).
///
/// Forwards one recording index to the engine. No return slot is written.
export!(cdecl, rw_00bc71c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
