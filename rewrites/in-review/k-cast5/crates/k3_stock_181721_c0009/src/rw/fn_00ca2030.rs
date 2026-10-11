// original: 0x00ca2030 time_check_130

/// Test whether the 0x130 timer has run past the global clock.
///
/// Same shape as `time_check_138` with the stamp at `+0x130` and the span
/// at `+0x134.
///
/// Original: 0x00ca2030 (thiscall, one stack word = float bits, returns al).
lf_checker_rt::export!(thiscall, rw_00ca2030(this: u32, arg: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe {
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        const STAMP: u32 = 0x130;
        const SPAN: u32 = 0x134;
        let now = *lf_checker_rt::global::<u32>(0x0117_35b4);
        let stamp = rd32(this + STAMP);
        if now < stamp {
            return 1;
        }
        let count = rd32(this + SPAN);
        let f = (count as f64) as f32;
        let scaled = mul(f, div(1.0, f32::from_bits(arg)));
        let end = ((scaled as i64) as u32).wrapping_add(stamp);
        if now > end {
            1
        } else {
            0
        }
    }
});
