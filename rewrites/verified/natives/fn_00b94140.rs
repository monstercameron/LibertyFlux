// original: 0x00b94140 CLEAR_BIT
/// Script native `CLEAR_BIT` (hash 0x66D57CC4).
///
/// Clears one bit: forwards two script words (a value and a bit index)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b94140(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
