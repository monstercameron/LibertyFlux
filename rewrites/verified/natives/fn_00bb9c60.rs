// original: 0x00bb9c60 TASK_GUARD_ANGLED_DEFENSIVE_AREA
/// Script native handler `TASK_GUARD_ANGLED_DEFENSIVE_AREA` (hash 0x030E0224).
///
/// Calls the engine worker with a fixed routine address and the call context and returns its answer.
const ROUTINE_00BB9C60: u32 = 0x00BBF1A0;
export!(cdecl, rw_00bb9c60(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // Fixed engine routine address, relocated like any absolute reference.
        let answer: u32 = callee_cdecl!(1, u32, relocated(ROUTINE_00BB9C60), ctx);
        answer
    }
});
