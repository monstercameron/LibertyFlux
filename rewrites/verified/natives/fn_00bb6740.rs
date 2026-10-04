// original: 0x00bb6740 SET_TOTAL_NUMBER_OF_MISSIONS
/// Script native `SET_TOTAL_NUMBER_OF_MISSIONS`.
///
/// Forwards one float bit-pattern to the engine. No return slot is written.
export!(cdecl, rw_00bb6740(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
