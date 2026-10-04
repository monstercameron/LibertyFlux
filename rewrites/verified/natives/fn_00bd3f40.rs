// original: 0x00bd3f40 EVOLVE_PTFX
use lf_k2_rt::{callee_cdecl, export};
/// Evolves a particle effect: handle, attribute and float value.
///
/// Forwards the three argument words (the last is a float) to the engine
/// particle function.
export!(cdecl, rw_00BD3F40(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let v = f32::from_bits(*a.add(2)).to_bits();
        callee_cdecl!(1, u32, *a, *a.add(1), v)
    }
});
