// original: 0x00b87b20 SET_HINT_MOVE_IN_DIST
/// Script native `SET_HINT_MOVE_IN_DIST` (hash 0x661A0CCC).
///
/// Script arguments: 1 word(s).
///
/// Forwards arg0 (float bit-pattern) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00b87b20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        callee_cdecl!(1, u32, arg0, )
    }
});
