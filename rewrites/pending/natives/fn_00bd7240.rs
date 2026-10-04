// original: 0x00bd7240 GET_TIME_OF_DAY
/// Script native `GET_TIME_OF_DAY` (hash 0x384B3876).
///
/// Forwards two script arguments (hour/minute out-pointers) to the engine. Writes no return slot itself.
export!(cdecl, rw_00bd7240(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
