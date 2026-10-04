// original: 0x009cbbb0 FREEZE_RADIO_STATION
use lf_k2_rt::{callee_cdecl, export};
/// Freeze the radio station: forward the script argument to the engine.
/// No return value.
export!(cdecl, rw_009cbbb0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args);
        0
    }
});
