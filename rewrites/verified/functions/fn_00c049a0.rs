// original: 0x00c049a0 stream_short3_set (proposed)

/// Pack three caller floats into words at `+0x18`, `+0x1a`, `+0x1c`.
///
/// `this` points to the object, `src` to three caller-owned floats. Each is
/// multiplied by `K_PACK` (`273.06`) and converted with truncation toward
/// zero (`cvttss2si`: NaN, infinities and out-of-range values become
/// `i32::MIN`); the low 16 are stored. Returns the third conversion
/// (what the original leaves in `eax`).
///
/// Original: 0x00c049a0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c049a0(this: u32, src: u32) -> u32 {
    unsafe {
        const OFF0: u32 = 0x18;
        const OFF1: u32 = 0x1a;
        const OFF2: u32 = 0x1c;
        const K_PACK: f32 = f32::from_bits(0x43888777);
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
        ((this.wrapping_add(OFF0)) as *mut i16).write_unaligned(c0 as i16);
        let c1 = cvtt(mul(rdf(src.wrapping_add(4)), K_PACK));
        ((this.wrapping_add(OFF1)) as *mut i16).write_unaligned(c1 as i16);
        let c2 = cvtt(mul(rdf(src.wrapping_add(8)), K_PACK));
        ((this.wrapping_add(OFF2)) as *mut i16).write_unaligned(c2 as i16);
        c2 as u32
    }
});
