// original: 0x00a359e0 vehicle_wrapped_pick (proposed)

/// Wrap `b - a` into `[-π, π]` and evaluate one of three candidates.
///
/// The wrapped delta `d` selects the callee (id 1, cdecl/1) argument: `b`
/// when `c` is ordered-greater than `|d|`, else `a - c` when `d` is
/// ordered-negative, else `a + c`. Returns the callee's answer. Cdecl/3
/// (float bits). The wrap loops run once per 2π of input magnitude; the
/// contract bounds inputs so they always terminate quickly.
lf_checker_rt::export!(cdecl, rw_00a359e0(a_bits: u32, b_bits: u32, c_bits: u32) -> u32 {
    unsafe {
        const PI: f32 = f32::from_bits(0x4049_0FDB);
        const TWO_PI: f32 = f32::from_bits(0x40C9_0FDB);
        const NEG_PI: f32 = f32::from_bits(0xC049_0FDB);
        const PICK: u32 = 1;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let a = f32::from_bits(a_bits);
        let b = f32::from_bits(b_bits);
        let c = f32::from_bits(c_bits);
        let mut d = sub(b, a);
        while d > PI {
            d = sub(d, TWO_PI);
        }
        while NEG_PI > d {
            d = add(d, TWO_PI);
        }
        let ad = f32::from_bits(d.to_bits() & 0x7FFF_FFFF);
        if c > ad {
            lf_checker_rt::callee_cdecl!(PICK, u32, b_bits)
        } else if d < 0.0 {
            lf_checker_rt::callee_cdecl!(PICK, u32, sub(a, c).to_bits())
        } else {
            lf_checker_rt::callee_cdecl!(PICK, u32, add(a, c).to_bits())
        }
    }
});
