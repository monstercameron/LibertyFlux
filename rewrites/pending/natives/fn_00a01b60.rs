// original: 0x00a01b60 SET_MONEY_PICKUP_NETWORK_REGEN_TIME
use lf_k2_rt::{callee_cdecl, export};
/// Set a money pickup's network regen time: forward the script argument
/// to the engine. No return value.
export!(cdecl, rw_00a01b60(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args);
        0
    }
});
