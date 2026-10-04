// original: 0x00bc5310 CLEAR_ROOM_FOR_CAR
/// Script native `CLEAR_ROOM_FOR_CAR` (hash 0x5FD24FEA).
///
/// Forwards one script argument (a car handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc5310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
