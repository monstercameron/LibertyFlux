// original: 0x00bb9f00 TASK_LEAVE_CAR_DONT_CLOSE_DOOR
use lf_k2_rt::{callee_cdecl, export};
// TASK_LEAVE_CAR_DONT_CLOSE_DOOR: forward (ped, vehicle) to the engine.
export!(cdecl, rw_00BB9F00(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *a, *a.add(1))
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}
