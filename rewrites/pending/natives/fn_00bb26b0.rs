// original: 0x00bb26b0 IS_WANTED_LEVEL_GREATER
/// Script native `IS_WANTED_LEVEL_GREATER` (hash 0x7DA4736D).
///
/// Forwards a player index and a wanted level to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb26b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
