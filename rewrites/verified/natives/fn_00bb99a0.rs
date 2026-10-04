// original: 0x00bb99a0 TASK_GET_OFF_BOAT
/// Script native `TASK_GET_OFF_BOAT` (hash 0x6C63251D).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb99a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
