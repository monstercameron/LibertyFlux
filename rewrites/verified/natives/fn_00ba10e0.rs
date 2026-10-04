// original: 0x00ba10e0 SET_CHAR_CAN_BE_KNOCKED_OFF_BIKE
/// Script native `SET_CHAR_CAN_BE_KNOCKED_OFF_BIKE` (hash 0x30C54CD2).
///
/// Forwards 2 script argument(s) to the engine: 2 integer(s).
/// No return slot is written.
export!(cdecl, rw_00ba10e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
