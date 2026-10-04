// original: 0x00BB6430 INCREMENT_FLOAT_STAT_NO_MESSAGE
// Rewrite of native handler INCREMENT_FLOAT_STAT_NO_MESSAGE.
//
// Passes stat id plus increment to the stat engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bb6430(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let engine: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(a0, a1)
    }
});
