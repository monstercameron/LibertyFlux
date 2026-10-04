// original: 0x00bc5de0 GET_CURRENT_TAXI_CAR_MODEL
/// Script native `GET_CURRENT_TAXI_CAR_MODEL` (hash 0x1D6D767E).
///
/// Forwards one script argument (a taxi handle) to the engine. No return slot is written.
export!(cdecl, rw_00bc5de0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
