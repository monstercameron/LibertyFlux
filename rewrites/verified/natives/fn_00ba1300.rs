// original: 0x00ba1300 SET_CHAR_DECISION_MAKER_TO_DEFAULT
/// Script native `SET_CHAR_DECISION_MAKER_TO_DEFAULT` (hash 0x73CB1489).
///
/// Forwards arg0 to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba1300(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
