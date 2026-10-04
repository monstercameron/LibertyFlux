// original: 0x00b8d350 PRINT_WITH_NUMBER_BIG
/// Script native `PRINT_WITH_NUMBER_BIG` (hash 0x49850843).
///
/// Forwards four script arguments (text label, numbers, timing) to the engine. Writes no return slot.
export!(cdecl, rw_00b8d350(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3))
    }
});
