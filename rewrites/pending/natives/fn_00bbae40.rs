// original: 0x00bbae40 TASK_WARP_CHAR_INTO_CAR_AS_DRIVER
/// Script native `TASK_WARP_CHAR_INTO_CAR_AS_DRIVER` (hash 0x6F363A21).
///
/// Forwards two script arguments (a character handle and a vehicle handle)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bbae40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
