// original: 0x00b94290 FORCE_WEATHER
use lf_k2_rt::{callee_cdecl, export};
// FORCE_WEATHER: forward the weather id to the engine call. No result.
export!(cdecl, rw_00B94290(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a)
    }
});
