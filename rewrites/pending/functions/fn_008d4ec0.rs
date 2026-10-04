// original: 0x008d4ec0 blend_rational
/// Rational blend of three floats with unit constant K (1.0): `a / b` when
/// `b > a`; 1.0 when `K - c > a`; otherwise `K - (a - (K - c)) / c`.
/// NaN inputs take the not-greater path at each comparison, as with `comiss`.
export!(cdecl, rw_008d4ec0(a: f32, b: f32, c: f32) -> f32 {
    unsafe {
        const K_ADDR: u32 = 0x00FE_88E8;
        let k = *global::<f32>(K_ADDR);
        if b > a {
            fdiv(a, b)
        } else if fsub(k, c) > a {
            1.0
        } else {
            fsub(k, fdiv(fsub(a, fsub(k, c)), c))
        }
    }
});
