// original: 0x00bc80e0 START_PLAYBACK_RECORDED_CAR_USING_AI
use lf_k2_rt::{callee_cdecl, export};
/// Starts AI playback of a recorded car path.
///
/// Forwards the vehicle handle and path word to the engine function.
export!(cdecl, rw_00BC80E0(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
