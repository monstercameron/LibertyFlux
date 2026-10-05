// original: 0x00b1b5e0 lerp_by_ratio (proposed)

/// Linearly interpolates between two floats by a measured ratio.
///
/// Stdcall of five stack words, all pointers to floats (callee pops 20
/// bytes): the ratio is `*p4 / (*p1 - *p0)` and the result is
/// `ratio * (*p3 - *p2) + *p2`, computed in that operand order. The
/// result is returned in ST0. The store of the result over the third
/// stack slot is scratch reuse, not an out-parameter.
lf_checker_rt::export!(stdcall, rw_00b1b5e0(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> f32 {
    unsafe {
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let a0 = (p0 as *const f32).read_unaligned();
        let a1 = (p1 as *const f32).read_unaligned();
        let b = (p2 as *const f32).read_unaligned();
        let c = (p3 as *const f32).read_unaligned();
        let d = (p4 as *const f32).read_unaligned();
        add(mul(div(d, sub(a1, a0)), sub(c, b)), b)
    }
});
