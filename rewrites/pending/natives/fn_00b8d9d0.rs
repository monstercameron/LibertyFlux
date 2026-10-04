// original: 0x00b8d9d0 SET_TEXT_LINE_HEIGHT_MULT
/// Script native `SET_TEXT_LINE_HEIGHT_MULT` (hash 0x5BF53817).
///
/// Forwards one script argument (a float multiplier, copied as raw bits so the forward is bit-exact) to the engine. No return slot is written.
export!(cdecl, rw_00b8d9d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
