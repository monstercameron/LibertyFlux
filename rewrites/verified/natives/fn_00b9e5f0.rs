// original: 0x00b9e5f0 COPY_CHAR_DECISION_MAKER
/// Script native `COPY_CHAR_DECISION_MAKER` (hash 0x1BB41B75).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b9e5f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
