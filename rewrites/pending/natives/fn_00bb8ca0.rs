// original: 0x00bb8ca0 GET_SEQUENCE_PROGRESS_RECURSIVE
/// Script native `GET_SEQUENCE_PROGRESS_RECURSIVE` (hash 0x60BC4116).
///
/// Forwards three script arguments to the engine. Writes no return slot itself.
export!(cdecl, rw_00bb8ca0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2))
    }
});
