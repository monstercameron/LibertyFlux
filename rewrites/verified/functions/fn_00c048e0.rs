// original: 0x00c048e0 stream_dir_set_b (proposed)

/// Pack a direction's x/y into bytes and its z sign into a flag bit.
///
/// `this` points to the object, `src` to three caller-owned floats. The first
/// two are multiplied by 127 and truncated (`cvttss2si`) into the bytes at
/// `+0x20` and `+0x21`; bit `0x80` of the byte at `+0x17`
/// is set to whether the third float is strictly below zero (an unordered
/// NaN comparison clears it). Returns the leftover in `eax`: the second
/// conversion with its low byte replaced as the original leaves it.
///
/// Original: 0x00c048e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c048e0(this: u32, src: u32) -> u32 {
    unsafe {
        const OFF0: u32 = 0x20;
        const OFF1: u32 = 0x21;
        const OFF_FLAGS: u32 = 0x17;
        const BIT: u8 = 0x80;
        const K_PACK: f32 = f32::from_bits(0x42fe0000);
        /// Truncating float-to-int exactly like `cvttss2si`: round toward
        /// zero, and `i32::MIN` for NaN, infinities and out-of-range values
        /// (Rust's `as` would saturate instead).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            if x >= 2147483648.0f32 || x < -2147483648.0f32 {
                i32::MIN
            } else {
                x as i32
            }
        }

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        let c0 = cvtt(mul(rdf(src), K_PACK));
        ((this.wrapping_add(OFF0)) as *mut u8).write(c0 as u8);
        let c1 = cvtt(mul(rdf(src.wrapping_add(4)), K_PACK));
        ((this.wrapping_add(OFF1)) as *mut u8).write(c1 as u8);
        let z = rdf(src.wrapping_add(8));
        let below = u32::from(0.0f32 > z);
        let old = ((this.wrapping_add(OFF_FLAGS)) as *const u8).read();
let new = (old & 0x7f) | ((below as u8) << 7);
        ((this.wrapping_add(OFF_FLAGS)) as *mut u8).write(new);
        (c1 as u32 & 0xffff_ff00) | (u32::from(old) & 0x7f)
    }
});
