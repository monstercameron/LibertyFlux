// original: 0x009090f0 radar_point_project
/// Project a radar point to grid cells through the engine scaler.
///
/// Scales the point by the configured range values, runs the Y term through
/// the engine's double-precision scaler, rounds with the 2^23 add/sub magic
/// keyed off X's sign, and hands both truncated cells to the engine plot
/// step. Returns the plot step's answer.
export!(cdecl, rw_009090F0(pt: u32) -> u32 {
    unsafe {
        let p = pt as *const u32;
        let px = f32::from_bits(*p);
        let py = f32::from_bits(*p.add(1));
        let half = (*global::<i32>(0x10344E0) as f32) * 0.5f32;
        let dc = *global::<i32>(0x10344DC) as f32;
        let x = (1.0f32 / dc) * (half + px);
        let y = (1.0f32 / dc) * (py + half);
        let d = (((*global::<i32>(0x10344E4)).wrapping_sub(1)) as f32 - y) as f64;
        let scaler: extern "cdecl" fn(f64) -> f32 =
            core::mem::transmute(callee_addr(1) as usize);
        let arg_a = cvttss2si(scaler(d));
        let sign = x.to_bits() & 0x80000000;
        let huge = if f32::from_bits(x.to_bits() & 0x7FFFFFFF) < 8388608.0f32 {
            0x4B000000u32
        } else {
            0
        };
        let magic = f32::from_bits(sign | huge);
        let t = (x + magic) - magic;
        // Predicate byte is 6 (NLE: not-less-or-equal, unordered counts as
        // true), i.e. Rust `!(d <= s)`. Together with the magic rounding this
        // computes floor(x).
        let adj = if !((t - x) <= f32::from_bits(sign)) {
            1.0f32
        } else {
            0.0f32
        };
        let arg_b = cvttss2si(t - adj);
        callee_cdecl!(2, u32, arg_b as u32, arg_a as u32)
    }
});

/// Truncate a float to int with x86 cvttss2si semantics.
///
/// NaN, infinities and out-of-range values yield 0x80000000, which Rust's
/// `as` does not reproduce (it saturates), so the indefinite case is
/// open-coded.
#[inline(always)]
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0f32 || x < -2147483648.0f32 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}
