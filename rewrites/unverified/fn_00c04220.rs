// original: 0x00c04220 stream_init_6a (proposed)

/// Initialise a streaming record (tag `0x6a`) and pack two floats into bytes.
///
/// `this` points to the record, `v` to a three-word vector, `f1`/`f2` are
/// float bits. Writes the tag, the shared streaming global and, through the
/// position callee, the vector; then stores `trunc((f1 - 1) * 510)` at
/// `+0x14` and `trunc((f2 - 15) * 25.5)` at `+0x15` (truncation is
/// `cvttss2si`: NaN, infinities and out-of-range values become `i32::MIN`).
/// Returns the second conversion (what the original leaves in `eax`).
///
/// Original: 0x00c04220 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c04220(this: u32, v: u32, f1: u32, f2: u32) -> u32 {
    unsafe {
        const TAG: u8 = 0x6a;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        const CALLEE_POS: u32 = 1;
        const K_ONE: f32 = 1.0;
        const K_UP1: f32 = f32::from_bits(0x43ff0000);
        const K_BASE2: f32 = f32::from_bits(0x41700000);
        const K_UP2: f32 = f32::from_bits(0x41cc0000);
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
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
        (this as *mut u8).write(TAG);
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        (this.wrapping_add(4) as *mut u32).write_unaligned(g);
        lf_checker_rt::callee_thiscall!(CALLEE_POS, u32, this, v);
        let c0 = cvtt(mul(sub(f32::from_bits(f1), K_ONE), K_UP1));
        ((this.wrapping_add(0x14)) as *mut u8).write(c0 as u8);
        let c1 = cvtt(mul(sub(f32::from_bits(f2), K_BASE2), K_UP2));
        ((this.wrapping_add(0x15)) as *mut u8).write(c1 as u8);
        c1 as u32
    }
});
