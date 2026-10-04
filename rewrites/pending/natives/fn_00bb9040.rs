// original: 0x00bb9040 TASK_AIM_GUN_AT_CHAR
/// Script native `TASK_AIM_GUN_AT_CHAR` (hash 0x4437501B).
///
/// Forwards three script arguments (two character handles and a flag value) to the engine. No return slot is written.
export!(cdecl, rw_00bb9040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
