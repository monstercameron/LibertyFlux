// original: 0x00bc69a0 IS_CAR_PASSENGER_SEAT_FREE
/// Script native `IS_CAR_PASSENGER_SEAT_FREE` (hash 0x1BDA0DA5).
///
/// Returns true if the specified car seat is empty
///
/// Script arguments: 2 word(s).
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle) to the engine routine.
/// The engine's byte answer is zero-extended into the return slot.
export!(cdecl, rw_00bc69a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const *mut u32);
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, arg0, arg1, );
        *slot = answer & 0xFF;
        slot as u32
    }
});
