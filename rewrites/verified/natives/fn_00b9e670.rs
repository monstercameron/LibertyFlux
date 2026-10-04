// original: 0x00b9e670 COPY_SHARED_CHAR_DECISION_MAKER
/// Script native `COPY_SHARED_CHAR_DECISION_MAKER` (hash 0x189E32C9).
///
/// Forwards two script arguments (two character handles) to the engine. No return slot is written.
export!(cdecl, rw_00b9e670(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
