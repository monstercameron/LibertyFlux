// original: 0x00ba2920 SUPPRESS_PED_MODEL
use lf_k2_rt::{callee_cdecl, export};
/// Suppresses a ped model.
///
/// Forwards the single argument word to the engine function.
export!(cdecl, rw_00BA2920(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        callee_cdecl!(1, u32, *a)
    }
});
