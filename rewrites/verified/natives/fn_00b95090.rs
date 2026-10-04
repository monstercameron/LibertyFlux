// original: 0x00b95090 SET_UP_TRIP_SKIP_AFTER_MISSION
use lf_k2_rt::{callee_cdecl, export};
/// Set up trip skip after a mission: pass the four script words (all
/// floats, bitwise) to the engine. No return value.
export!(cdecl, rw_00b95090(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
        0
    }
});
