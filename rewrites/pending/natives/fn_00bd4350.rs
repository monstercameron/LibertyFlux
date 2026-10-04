// original: 0x00bd4350 START_PTFX
/// Script native `START_PTFX` (hash 0x3A774777).
///
/// Forwards eight script arguments to the engine particle-effect starter: a
/// name handle plus seven coordinate words copied as raw bits. The original
/// builds the argument block on its stack with vector moves; the values the
/// engine receives are exactly the eight script words. Stores the full
/// 32-bit answer (an effect handle) into the return slot.
export!(cdecl, rw_00bd4350(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(7)
        );
        *slot = answer;
        answer
    }
});
