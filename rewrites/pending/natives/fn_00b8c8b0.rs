// original: 0x00b8c8b0 GET_FRONTEND_DESIGN_VALUE
/// Script native `GET_FRONTEND_DESIGN_VALUE` (hash 0x747E681E).
///
/// Forwards three script arguments to the engine. No return slot is written
/// by the handler itself.
export!(cdecl, rw_00b8c8b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
