// original: 0x00ba28a0 SET_SPECIFIC_PASSENGER_INDEX_TO_USE_IN_GROUPS
use lf_k2_rt::{callee_cdecl, export};
// SET_SPECIFIC_PASSENGER_INDEX_TO_USE_IN_GROUPS: forward (group, index).
export!(cdecl, rw_00BA28A0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *a, *a.add(1))
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}
