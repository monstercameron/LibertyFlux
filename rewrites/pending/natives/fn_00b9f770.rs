// original: 0x00b9f770 IS_CHAR_FACING_CHAR
/// Script native `IS_CHAR_FACING_CHAR`.
///
/// Forwards three script arguments (two character handles and a float
/// bit-pattern for the angle tolerance) to the engine. The float is copied
/// as raw bits, so the forward is bit-exact. Stores the low byte of the
/// engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9f770(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
