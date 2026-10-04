// original: 0x00ba20d0 SET_GROUP_FORMATION_SPACING
/// Script native `SET_GROUP_FORMATION_SPACING` (hash 0x69315157).
///
/// Script arguments: 2 word(s).
///
/// Forwards arg0 (integer/handle), arg1 (float bit-pattern) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00ba20d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        callee_cdecl!(1, u32, arg0, arg1, )
    }
});
