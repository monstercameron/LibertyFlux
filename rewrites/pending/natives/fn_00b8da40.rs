// original: 0x00b8da40 SET_TEXT_SCALE
/// Script native `SET_TEXT_SCALE` (hash 0x02C069E5).
///
/// Forwards two script arguments (float bit-patterns) to the engine. No
/// return slot is written.
export!(cdecl, rw_00b8da40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
