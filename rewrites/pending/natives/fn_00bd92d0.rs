// original: 0x00bd92d0 SET_LCPD_CRIMINAL_SCORE
/// Script native `SET_LCPD_CRIMINAL_SCORE` (hash 0x7457458C).
///
/// Forwards one script argument (the score) to the engine. No return slot
/// is written.
export!(cdecl, rw_00bd92d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
