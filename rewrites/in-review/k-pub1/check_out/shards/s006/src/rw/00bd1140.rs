// original: 0x00bd1140 SELECT_WEAPONS_FOR_VEHICLE
use lf_k2_rt::{callee_cdecl, export};
// SELECT_WEAPONS_FOR_VEHICLE: forward (vehicle, weapons) to the engine.
export!(cdecl, rw_00BD1140(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *a, *a.add(1))
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}
