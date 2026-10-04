// original: 0x00b9ec30 GET_CAR_CHAR_IS_USING
/// Script native `GET_CAR_CHAR_IS_USING` (hash 0x1B067237).
///
/// Forwards two script arguments (a character handle and an out-pointer) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9ec30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1),)
    }
});
