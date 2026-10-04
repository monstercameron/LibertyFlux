// original: 0x00a1de50 cam_dual_channel_sample (proposed)

/// Samples two channels through getter/converter pairs, twice each.
///
/// `a0` selects the subject (null skips the sampling and goes straight to
/// the report), `a1`/`a2` are float out-slots. The first verse runs getter
/// A and getter B on `a0`, converts each integer answer and stores the
/// NEGATED floats (`-(v as float)`, negating the integer first, wrapping)
/// to `a1`/`a2`. When the extended byte at `a0 + EXT_OFF` is set, a second
/// verse repeats both getters with selector arguments 1 and 0 through the
/// alternate converters, overwriting the slots. The function returns 1
/// (`al`) when either slot holds a nonzero float (NaN counts as nonzero),
/// otherwise 0.
///
/// Original: 0x00a1de50 (stdcall, three stack words).
lf_checker_rt::export!(stdcall, rw_00a1de50(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const GET_A: u32 = 1;
        const CONV_A: u32 = 2;
        const GET_B: u32 = 3;
        const CONV_B: u32 = 4;
        const GET_A2: u32 = 5;
        const CONV_A2: u32 = 6;
        const GET_B2: u32 = 7;
        const CONV_B2: u32 = 8;
        const EXT_OFF: u32 = 0x3289;
        #[inline(always)]
        fn neg_float(v: u32) -> f32 {
            (v as i32).wrapping_neg() as f32
        }
        unsafe fn wrf(x: u32, v: f32) {
            unsafe { (x as *mut u32).write_unaligned(v.to_bits()) }
        }
        if a0 != 0 {
            let t = lf_checker_rt::callee_thiscall!(GET_A, u32, a0);
            let v = lf_checker_rt::callee_cdecl!(CONV_A, u32, t);
            wrf(a1, neg_float(v));
            let t = lf_checker_rt::callee_thiscall!(GET_B, u32, a0);
            let v = lf_checker_rt::callee_cdecl!(CONV_B, u32, t);
            wrf(a2, neg_float(v));
            if ((a0 + EXT_OFF) as *const u8).read() != 0 {
                let t = lf_checker_rt::callee_thiscall!(GET_A2, u32, a0, 1);
                let v = lf_checker_rt::callee_cdecl!(CONV_A2, u32, t);
                wrf(a1, neg_float(v));
                let t = lf_checker_rt::callee_thiscall!(GET_B2, u32, a0, 0);
                let v = lf_checker_rt::callee_cdecl!(CONV_B2, u32, t);
                wrf(a2, neg_float(v));
            }
        }
        let f0 = f32::from_bits((a1 as *const u32).read_unaligned());
        if f0 != 0.0 {
            return 1;
        }
        let f1 = f32::from_bits((a2 as *const u32).read_unaligned());
        u32::from(f1 != 0.0)
    }
});
