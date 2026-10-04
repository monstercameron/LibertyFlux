// original: 0x0091c9f0 input_axis_calibrate (proposed)

/// Calibrate one input axis and store its range block.
///
/// `out` points to four floats the function fills: `LOW` at `+0`, `CENTER`
/// at `+4`, `HIGH` at `+8`, `SCALE` at `+12`. `fa` is the axis resting value,
/// `fb` its travel, `sel` a selector passed to the range solver (callee 2).
///
/// Callee 1 answers a row id into the axis table (72 bytes per row, in
/// writable game data): the flag word at `+0`, the low anchor at `+0x10` and
/// the high anchor at `+0x14`. `LOW` is the low anchor minus `GAIN_LO`.
/// `HIGH` is normally the high anchor plus `GAIN_HI`, but when the mode byte
/// is `0x6a` or the override byte is nonzero it is the low anchor plus the
/// solver's out value plus `GAIN_HI`. When the flag word is exactly zero the
/// pair is symmetrised around `fa`: with `m` the smaller of `fa - LOW` and
/// `HIGH - fa` (ordered comparison, unordered takes the first), and only
/// when `m` is not below zero, `LOW` becomes `fa - m` and `HIGH` `m + fa`.
/// `CENTER` is `fb` minus `GAIN_LO`. Callee 3 answers a factor `f` (x87);
/// `SCALE` is `f * sel_answer + fa + GAIN_LO`, where the solver's integer
/// answer converts as a signed dword. The function returns callee 3's `eax`
/// (the factor bits) untouched.
///
/// The two gains are read-only data constants, embedded here by value.
/// Cdecl, four stack words, result in `eax`, three outgoing calls.
lf_checker_rt::export!(cdecl, rw_0091c9f0(out: u32, fa: f32, fb: f32, sel: u32) -> u32 {
    unsafe {
        const GAIN_LO: f32 = f32::from_bits(0x3B9A0275);
        const GAIN_HI: f32 = f32::from_bits(0x3C1A0275);
        const TABLE: u32 = 0x0119BF28;
        const FLAGS: u32 = 0x0116C250;
        const ROW_BYTES: u32 = 72;
        const FLAG_MODE: u32 = 0x6A;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }

        let row_id: u32 = lf_checker_rt::callee_cdecl!(1, u32);
        let row = lf_checker_rt::relocated(TABLE)
            .wrapping_add(row_id.wrapping_mul(9).wrapping_mul(8));
        let mut solved: u32 = 0;
        let n: u32 = lf_checker_rt::callee_cdecl!(
            2, u32, fa.to_bits(), fb.to_bits(), sel,
            &mut solved as *mut u32 as u32
        );
        let flag_word = rd32(row);
        let low_anchor = rdf(row + 0x10);
        let low = sub(low_anchor, GAIN_LO);
        let flags = rd32(lf_checker_rt::relocated(FLAGS));
        let high = if (flags & 0xFF) == FLAG_MODE || (flags >> 24) != 0 {
            add(add(low_anchor, f32::from_bits(solved)), GAIN_HI)
        } else {
            add(rdf(row + 0x14), GAIN_HI)
        };
        let (low, high) = if flag_word == 0 {
            let d0 = sub(fa, low);
            let d1 = sub(high, fa);
            let m = if d0 > d1 { d1 } else { d0 };
            if 0.0f32 > m {
                (low, high)
            } else {
                (sub(fa, m), add(m, fa))
            }
        } else {
            (low, high)
        };
        wrf(out, low);
        wrf(out + 8, high);
        wrf(out + 4, sub(fb, GAIN_LO));
        let f: f32 = lf_checker_rt::callee_cdecl!(3, f32);
        let acc = add(add(mul(f, (n as i32) as f32), fa), GAIN_LO);
        wrf(out + 12, acc);
        f.to_bits()
    }
});
