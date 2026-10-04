// original: 0x00b8cec0 PRINT
/// Script native `PRINT` (hash 0x0A491CFF).
///
/// Forwards three script arguments (a text label and two display parameters)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b8cec0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
