// original: 0x00bb6650 SET_FLOAT_STAT
/// Script native `SET_FLOAT_STAT` (hash 0x5213511B).
///
/// Forwards two script arguments to the engine: an integer stat index and
/// a float value as a raw bit-pattern. No return slot is written.
export!(cdecl, rw_00bb6650(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
