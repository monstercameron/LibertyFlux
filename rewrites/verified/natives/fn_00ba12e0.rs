// original: 0x00ba12e0 SET_CHAR_DECISION_MAKER
/// Script native `SET_CHAR_DECISION_MAKER` (hash 0x01F8116C).
///
/// Forwards two script arguments (a character handle and a decision
/// maker handle) to the engine. No return slot is written.
export!(cdecl, rw_00ba12e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
