// original: 0x00ba1a50 SET_CHAR_SHOOT_RATE
/// Script native handler `SET_CHAR_SHOOT_RATE` (hash 0x2AE979DC).
///
/// Forwards script arguments 0..1 to the engine worker and returns its answer.
export!(cdecl, rw_00ba1a50(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
