// original: 0x00c9d650 float_window_select

/// Pick `a - c`, `a + c` or `b` by how far `b` drifted from `a`.
///
/// With `d = b - a`: when `c` exceeds `|d|` the drift is inside the window
/// and `b` is returned. Otherwise, when `|d|` exceeds `c * 200` and `c`
/// exceeds 0.02, the drift is trusted and `b` is returned too. Every other
/// case clamps to the window edge: `a + c` when `b >= a`, `a - c` when
/// `b < a`. All comparisons are NaN-aware exactly like the original's
/// `comiss` pairs (`ja` is strict `>`, `jbe` is `!(>)`), and the arithmetic
/// keeps the original's operand order.
///
/// Original: 0x00c9d650 (cdecl, three stack words, returns float in st0).
lf_checker_rt::export!(cdecl, rw_00c9d650(a: u32, b: u32, c: u32) -> f32 {
    unsafe {
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        const SCALE: f32 = 200.0;
        const LIMIT_BITS: u32 = 0x3ca3_d70a;
        const ABS_MASK: u32 = 0x7fff_ffff;
        let fa = f32::from_bits(a);
        let fb = f32::from_bits(b);
        let fc = f32::from_bits(c);
        let d = sub(fb, fa);
        let ad = f32::from_bits(d.to_bits() & ABS_MASK);
        if fc > ad {
            return fb;
        }
        if ad > mul(fc, SCALE) && fc > f32::from_bits(LIMIT_BITS) {
            return fb;
        }
        if !(0.0f32 > d) {
            add(fa, fc)
        } else {
            sub(fa, fc)
        }
    }
});
