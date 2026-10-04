// original: 0x00bc5cf0 GET_CHAR_IN_CAR_PASSENGER_SEAT
/// Script native `GET_CHAR_IN_CAR_PASSENGER_SEAT` (hash 0x5E756B51).
///
/// Forwards three script arguments (a vehicle handle, a seat index and an out-pointer) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bc5cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
