// original: 0x00bd9010 RESERVE_NETWORK_MISSION_OBJECTS
use lf_k2_rt::{callee_cdecl, export};
// RESERVE_NETWORK_MISSION_OBJECTS: forward the count to the engine call.
export!(cdecl, rw_00BD9010(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *a)
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}
