// original: 0x00bc5040 ADD_UPSIDEDOWN_CAR_CHECK
/// Script native `ADD_UPSIDEDOWN_CAR_CHECK` (hash 0x557C076C).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bc5040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
