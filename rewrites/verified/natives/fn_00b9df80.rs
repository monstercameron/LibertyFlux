// original: 0x00b9df80 APPLY_FORCE_TO_PED
/// Script native handler `APPLY_FORCE_TO_PED` (hash 0x7305301D).
///
/// Calls the engine worker with a fixed routine address and the call context and returns its answer.
const ROUTINE_00B9DF80: u32 = 0x00BA3C70;
export!(cdecl, rw_00b9df80(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        // Fixed engine routine address, relocated like any absolute reference.
        let answer: u32 = callee_cdecl!(1, u32, relocated(ROUTINE_00B9DF80), ctx);
        answer
    }
});
