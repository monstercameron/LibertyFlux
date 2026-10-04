// original: 0x00ba16a0 SET_CHAR_MAX_TIME_IN_WATER
/// Script native `SET_CHAR_MAX_TIME_IN_WATER` (hash 0x45F32596).
///
/// Forwards two script arguments to the engine: an integer handle and a
/// float duration as a raw bit-pattern. No return slot is written.
export!(cdecl, rw_00ba16a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
