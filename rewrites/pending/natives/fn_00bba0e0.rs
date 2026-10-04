// original: 0x00BBA0E0 TASK_PERFORM_SEQUENCE_LOCALLY
// Rewrite of native handler TASK_PERFORM_SEQUENCE_LOCALLY.
//
// Passes ped handle plus sequence id to the task-sequence engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bba0e0(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let engine: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(a0, a1)
    }
});
