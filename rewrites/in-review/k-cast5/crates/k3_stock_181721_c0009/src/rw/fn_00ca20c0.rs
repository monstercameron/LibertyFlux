// original: 0x00ca20c0 time_seed_130

/// Stamp the 0x130 timer and pick a random span for it.
///
/// Same shape as `time_seed_138` with the stamp at `+0x130`, the span at
/// `+0x134` and a wider pair of global ranges.
///
/// Original: 0x00ca20c0 (thiscall, one stack word = bias).
lf_checker_rt::export!(thiscall, rw_00ca20c0(this: u32, bias: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const STAMP: u32 = 0x130;
        const SPAN: u32 = 0x134;
        const KIND: u32 = 0x12c;
        const RATE_BITS: u32 = 0x3800_0000; // 2^-16
        wr32(this + STAMP, *lf_checker_rt::global::<u32>(0x0117_35b4));
        let kind = rd32(this + KIND);
        let (hi, lo) = if kind == 2 || kind == 0x10 || kind == 0x11 {
            (
                *lf_checker_rt::global::<u32>(0x0104_5914),
                *lf_checker_rt::global::<u32>(0x0104_5910),
            )
        } else {
            (
                *lf_checker_rt::global::<u32>(0x0104_590c),
                *lf_checker_rt::global::<u32>(0x0104_5908),
            )
        };
        let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        let frac = mul((r & 0xffff) as f32, f32::from_bits(RATE_BITS));
        let width = mul(frac, (hi.wrapping_sub(lo) as i32) as f32);
        let span = (width as i32 as u32).wrapping_add(lo).wrapping_add(bias);
        wr32(this + SPAN, span);
        bias
    }
});
