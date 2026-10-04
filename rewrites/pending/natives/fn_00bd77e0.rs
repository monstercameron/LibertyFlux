// original: 0x00bd77e0 CLEAR_SCRIPT_ARRAY_FROM_SCRATCHPAD
/// Script native `CLEAR_SCRIPT_ARRAY_FROM_SCRATCHPAD` (hash 0x6E120246).
///
/// Forwards arg0 to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd77e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
