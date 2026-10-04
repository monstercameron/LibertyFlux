// original: 0x00bb9720 TASK_EXTEND_ROUTE
/// Script native `TASK_EXTEND_ROUTE` (hash 0x75353EA4).
///
/// Forwards three script arguments to the engine as raw 32-bit words (moved
/// through vector registers, copied bit-for-bit). No return slot is written.
export!(cdecl, rw_00bb9720(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
