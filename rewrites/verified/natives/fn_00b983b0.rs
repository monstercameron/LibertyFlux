// original: 0x00b983b0 GET_PAD_STATE
/// Script native `GET_PAD_STATE` (hash 0x5D4C1D59).
///
/// Forwards three script arguments (pad index, state index, out-pointer) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b983b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
