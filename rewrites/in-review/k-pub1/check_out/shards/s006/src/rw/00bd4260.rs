// original: 0x00bd4260 SET_MASK
use lf_k2_rt::{callee_cdecl, export};
// SET_MASK: forward four float-bit arguments (a mask vector) to the engine.
export!(cdecl, rw_00BD4260(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), *a.add(3))
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}
