// original: 0x00bb1bd0 CHANGE_PLAYER_PHONE_MODEL_OFFSETS
use lf_k2_rt::{callee_cdecl, export};
/// Changes the player phone model offsets (one int, six floats).
///
/// Forwards all seven argument words to the engine function. Float words
/// pass through bit-exact; the rewrite names them as floats.
export!(cdecl, rw_00BB1BD0(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let f = |i: usize| f32::from_bits(*a.add(i)).to_bits();
        callee_cdecl!(1, u32, *a, f(1), f(2), f(3), f(4), f(5), f(6))
    }
});
