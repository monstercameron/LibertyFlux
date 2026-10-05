// original: 0x0069D7C0 rage::crAnimChannelRleInt::vf5

/// Samples an RLE integer channel at a float position into an output.
///
/// `this` is the channel object (forwarded to the decoder) and the stack
/// arguments are the position `x` as `f32` bits and the output pointer. The
/// position is shifted by a static rounding bias, truncated to an integer
/// exactly like `cvttss2si` (NaN, infinities and out-of-range values yield
/// `0x80000000`), decoded through the integer decoder (callee 1, thiscall:
/// channel, index) and stored to the output. Returns the decoded value.
///
/// Original: 0x0069D7C0 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_0069D7C0(this: u32, xbits: u32, out: u32) -> u32 {
    unsafe {
        const ROUND_BIAS: u32 = 0xFE8830;
        const DECODE: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate exactly like cvttss2si (out-of-range/NaN -> MIN).
        fn cvtt_trunc(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        let x = f32::from_bits(xbits);
        let bias = unsafe { lf_checker_rt::global::<f32>(ROUND_BIAS).read() };
        let t = fadd(x, bias);
        let idx = cvtt_trunc(t);
        let v = lf_checker_rt::callee_thiscall!(DECODE, u32, this, idx as u32);
        wr32(out, v);
        v
    }
});
