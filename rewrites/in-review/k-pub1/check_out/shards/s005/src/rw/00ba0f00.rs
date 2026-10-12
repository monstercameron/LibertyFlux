// original: 0x00ba0f00 SET_CHAR_ALL_ANIMS_SPEED
use lf_k2_rt::{callee_cdecl, export};
// SET_CHAR_ALL_ANIMS_SPEED: forward (ped, speed-bits) to the engine call.
// The speed travels as raw float bits.
export!(cdecl, rw_00BA0F00(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        __lf_callee_cdecl!(1, u32, *a, *a.add(1))
    }
});

unsafe fn args_of(ctx: u32) -> *const u32 {
    * (ctx as *const u32).add(2) as *const u32
}
