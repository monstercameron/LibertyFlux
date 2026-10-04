// original: 0x00bb88a0 ADD_FOLLOW_NAVMESH_TO_PHONE_TASK
use lf_k2_rt::{callee_addr, export};
// Rewrite of native handler ADD_FOLLOW_NAVMESH_TO_PHONE_TASK.
//
// Passes phone task target (handle + xyz) to the phone-task engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bb88a0(ctx: u32) -> u32 {
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
