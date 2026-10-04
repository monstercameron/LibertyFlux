// original: 0x00b9aec0 SWITCH_ROADS_BACK_TO_ORIGINAL
use lf_k2_rt::{callee_addr, export};
// Rewrite of native handler SWITCH_ROADS_BACK_TO_ORIGINAL.
//
// Passes six road-region bounds to the roads engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00b9aec0(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let a2 = *argv.add(2);
        let a3 = *argv.add(3);
        let a4 = *argv.add(4);
        let a5 = *argv.add(5);
        let engine: extern "cdecl" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(a0, a1, a2, a3, a4, a5)
    }
});
