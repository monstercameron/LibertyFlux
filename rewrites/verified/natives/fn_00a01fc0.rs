// original: 0x00a01fc0 SET_PICKUP_COLLECTABLE_BY_CAR
use lf_k2_rt::{callee_cdecl, export};
/// Set whether a pickup is collectable by car: pass the pickup handle
/// plus the second script argument coerced to 0/1. The original's pushed
/// dword also carries dead-slot high bytes, masked in the contract (call_skip).
export!(cdecl, rw_00a01fc0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag);
        0
    }
});
