// original: 0x00bc5e30 GET_DOOR_ANGLE_RATIO
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_DOOR_ANGLE_RATIO`: forwards script args [arg0 (dword), arg1 (dword), arg2 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00bc5e30(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2));
        0
    }
});
