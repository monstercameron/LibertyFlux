// original: 0x00BBA7A0 TASK_SIT_DOWN
// Rewrite of native handler TASK_SIT_DOWN.
//
// Passes ped handle plus three seat words to the sit-down engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bba7a0(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let a2 = *argv.add(2);
        let a3 = *argv.add(3);
        let engine: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(a0, a1, a2, a3)
    }
});
