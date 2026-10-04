// original: 0x00b87810 SET_CAR_FOV_START_SPEED_BOAT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_CAR_FOV_START_SPEED_BOAT`: forwards script args [arg0 (float bits)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00b87810(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0));
        0
    }
});
