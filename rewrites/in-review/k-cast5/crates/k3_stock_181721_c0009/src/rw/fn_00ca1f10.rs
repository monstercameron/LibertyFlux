// original: 0x00ca1f10 time_check_138

/// Test whether the 0x138 timer has run past the global clock.
///
/// Returns 1 when the global clock is below the stamp at `+0x138` (unsigned).
/// Otherwise the span at `+0x13c` is scaled by the reciprocal of the
/// argument float, truncated toward zero, added to the stamp, and 1 is
/// returned when the clock is above that end, else 0. Only the low byte is
/// set. The division and multiplication keep the original's operand order.
///
/// The contract bounds the inputs so the float-to-int step never overflows
/// (the x87 invalid result is not modelled); see the narrowed list.
///
/// Original: 0x00ca1f10 (thiscall, one stack word = float bits, returns al).
lf_checker_rt::export!(thiscall, rw_00ca1f10(this: u32, arg: u32) -> u32 {
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
        const STAMP: u32 = 0x138;
        const SPAN: u32 = 0x13c;
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
