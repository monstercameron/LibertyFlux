// original: 0x00b86d50 GET_VIEWPORT_POS_AND_SIZE
/// Script native `GET_VIEWPORT_POS_AND_SIZE` (hash 0x4DDC6FB4).
///
/// Forwards five script arguments (out-pointers for the viewport position and size) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b86d50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
