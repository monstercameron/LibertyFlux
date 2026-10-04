// original: 0x009cbfb0 ON_FIRE_SCREAM
use lf_k2_rt::{callee_cdecl, export};
/// Makes a character scream while on fire.
///
/// Forwards the single argument word to the engine function.
export!(cdecl, rw_009CBFB0(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        callee_cdecl!(1, u32, *a)
    }
});
