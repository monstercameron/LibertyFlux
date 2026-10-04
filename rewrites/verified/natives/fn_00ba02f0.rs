// original: 0x00ba02f0 LOAD_CHAR_DECISION_MAKER
/// Script native `LOAD_CHAR_DECISION_MAKER` (hash 0x7F7B4FC5).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00ba02f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
