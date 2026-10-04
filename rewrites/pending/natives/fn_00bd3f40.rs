// original: 0x00BD3F40 EVOLVE_PTFX
/// Evolves a particle effect: handle, attribute and float value.
///
/// Forwards the three argument words (the last is a float) to the engine
/// particle function.
lf_rn26_rt::export!(cdecl, rw_00BD3F40(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let v = f32::from_bits(*a.add(2)).to_bits();
        lf_rn26_rt::callee_cdecl!(1, u32, *a, *a.add(1), v)
    }
});
