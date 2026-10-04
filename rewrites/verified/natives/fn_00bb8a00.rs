// original: 0x00bb8a00 CLEAR_CHAR_TASKS
/// Script native `CLEAR_CHAR_TASKS` (hash 0x4AB470F3).
///
/// Thin wrapper: reads 1 arg(s) [0], calls 0x00BBB6E0, does not write the return slot. Engine side touches: ped pool (0x018B6F1C).
///
/// Script arguments: 1 word(s).
///
/// Forwards arg0 (integer/handle) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00bb8a00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        callee_cdecl!(1, u32, arg0, )
    }
});
