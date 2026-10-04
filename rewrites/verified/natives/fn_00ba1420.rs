// original: 0x00ba1420 SET_CHAR_DESIRED_HEADING
/// Script native `SET_CHAR_DESIRED_HEADING` (hash 0x6EF64079).
///
/// Forwards two script arguments (a character handle and a float heading, copied as raw bits) to the engine. No return slot is written.
export!(cdecl, rw_00ba1420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
