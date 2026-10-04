// original: 0x00bb2920 SET_HEADING_FOR_ATTACHED_PLAYER
/// Script native `SET_HEADING_FOR_ATTACHED_PLAYER` (hash 0x6B247B9E).
///
/// Forwards one integer and two float script arguments to the engine as raw bits. Writes no return slot.
export!(cdecl, rw_00bb2920(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2))
    }
});
