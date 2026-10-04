// original: 0x00ca1fa0 time_seed_138

/// Stamp the 0x138 timer and pick a random span for it.
///
/// Writes the global clock into `+0x138`. A span range is chosen from two
/// global pairs by the kind word at `+0x12c` (kinds 2, 0x10 and 0x11 take
/// the wide pair, anything else the narrow pair); the span is a uniform
/// fraction of the range from the low 16 bits of the random helper
/// (callee 1) scaled by 2^-16, truncated, plus the range low plus the
/// caller's bias word. Stored at `+0x13c`. Returns the bias word.
///
/// The float multiplications keep the original's operand order.
///
/// Original: 0x00ca1fa0 (thiscall, one stack word = bias).
lf_checker_rt::export!(thiscall, rw_00ca1fa0(this: u32, bias: u32) -> u32 {
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
        const STAMP: u32 = 0x138;
        const SPAN: u32 = 0x13c;
        const KIND: u32 = 0x12c;
        const RATE_BITS: u32 = 0x3800_0000; // 2^-16
        wr32(this + STAMP, *lf_checker_rt::global::<u32>(0x0117_35b4));
        let kind = rd32(this + KIND);
        let (hi, lo) = if kind == 2 || kind == 0x10 || kind == 0x11 {
            (
                *lf_checker_rt::global::<u32>(0x0104_58fc),
                *lf_checker_rt::global::<u32>(0x0104_58f8),
            )
        } else {
            (
                *lf_checker_rt::global::<u32>(0x0104_58f4),
                *lf_checker_rt::global::<u32>(0x0104_58f0),
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
