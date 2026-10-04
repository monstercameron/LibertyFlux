// original: 0x00ba12a0 SET_CHAR_COORDINATES_NO_OFFSET
// Rewrite of the SET_CHAR_COORDINATES_NO_OFFSET native handler.

/// Script native `SET_CHAR_COORDINATES_NO_OFFSET(char, x, y, z)`.
///
/// Forwards the character handle and the three coordinate floats to the
/// engine teleport routine. No return slot is written; the engine's answer is
/// left in the return register.
export!(cdecl, rw_ba12a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
