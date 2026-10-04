// original: 0x00ba1a10 SET_CHAR_ROTATION
/// Script native `SET_CHAR_ROTATION` (hash 0x70E13826).
///
/// Forwards four script arguments to the engine: a character handle and
/// three float bit-patterns (rotation). The floats are only copied onto the
/// stack, so they are forwarded as raw bits. No return slot is written.
export!(cdecl, rw_00ba1a10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
