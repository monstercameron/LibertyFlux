// original: 0x008ed720 biased_scale_blend
/// Blend two integers with a float bias under a scale.
///
/// Picks a scale by `flag`, then: a zero `a1` decays `a2` from K, a
/// zero `a2` decays `a1` from K, otherwise the scaled `a3` bias is
/// added to the scaled base. Returns the result in ST0.
export!(cdecl, rw_008ed720(a1: i32, a2: i32, a3: f32, flag: u32) -> f64 {
    unsafe {
        let s: f32 = if flag & 0xFF != 0 {
            *global::<f32>(0xFE8AB8)
        } else {
            *global::<f32>(0xE833D4)
        };
        let kk: f32 = *global::<f32>(0xFE8830);
        let v = if a1 == 0 {
            (kk - a2 as f32 * kk) * s
        } else if a2 == 0 {
            (kk - a1 as f32 * kk) * s
        } else {
            s * kk + a3 * kk
        };
        v as f64
    }
});
