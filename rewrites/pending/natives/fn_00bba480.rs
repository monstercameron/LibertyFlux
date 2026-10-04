// original: 0x00bba480 TASK_PUT_CHAR_DIRECTLY_INTO_COVER
/// Script native `TASK_PUT_CHAR_DIRECTLY_INTO_COVER` (hash 0x1FDD4860).
///
/// Forwards five script arguments to the engine: a character handle, three float bit-patterns (a position) and an integer. The original shuffles the words through vector registers while building the call frame, but the net effect is a plain in-order forward; floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bba480(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4),)
    }
});
