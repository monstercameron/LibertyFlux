// original: 0x00bb97e0 TASK_FOLLOW_FOOTSTEPS
/// Script native `TASK_FOLLOW_FOOTSTEPS` (hash 0x45DF7CCA).
///
/// Makes one char follow another
///
/// Script arguments: 2 word(s).
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00bb97e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        callee_cdecl!(1, u32, arg0, arg1, )
    }
});
