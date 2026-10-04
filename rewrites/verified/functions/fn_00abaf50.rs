// original: 0x00abaf50 input_ui_bounds_accumulate (proposed)

/// Accumulate component-wise bounds over a strided item range, then report.
///
/// `arg1` points at the first item and `arg2` one past the last; items are
/// 0x20 bytes apart. For each item a measure call fills two triples of
/// floats (a high triple and a low triple). Three running maxima start at a
/// global seed and take the component-wise maximum with each high triple;
/// three running minima start at a second global seed and take the
/// component-wise minimum with each low triple. Comparisons follow the
/// original's compare-then-select order exactly, including NaN inputs (a NaN
/// candidate replaces the running value; a NaN running value is replaced by
/// any ordered candidate).
///
/// Afterwards `*arg4` is set to 1 and a sink call receives the minimum
/// triple, the maximum triple, `arg0` twice, `arg1`, `arg2`, `arg3`, `arg4`
/// and `arg5`. The function returns the sink's answer. The two seed floats
/// are read from the original image. Original is cdecl with six stack words.
lf_checker_rt::export!(cdecl, rw_00abaf50(
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x20;
        const SEED_HI: u32 = 0xfe8e1c;
        const SEED_LO: u32 = 0xfe8d18;
        const CAL_MEASURE: u32 = 1;
        const CAL_SINK: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        /// Maximum in the original's compare order: keep `acc` only when it
        /// is ordered-above `new`, else take `new`.
        #[inline(always)]
        fn fold_max(acc: f32, new: f32) -> f32 {
            if core::hint::black_box(acc) > core::hint::black_box(new) {
                acc
            } else {
                new
            }
        }
        /// Minimum in the original's compare order: keep `acc` only when
        /// `new` is ordered-above it, else take `new`.
        #[inline(always)]
        fn fold_min(acc: f32, new: f32) -> f32 {
            if core::hint::black_box(new) > core::hint::black_box(acc) {
                acc
            } else {
                new
            }
        }

        let seed_hi = rdf(lf_checker_rt::relocated(SEED_HI));
        let seed_lo = rdf(lf_checker_rt::relocated(SEED_LO));
        let mut hi = [seed_hi; 3];
        let mut lo = [seed_lo; 3];
        let mut p = arg1;
        while p != arg2 {
            let mut out_hi = [0u32; 3];
            let mut out_lo = [0u32; 3];
            lf_checker_rt::callee_cdecl!(
                CAL_MEASURE,
                u32,
                p,
                out_lo.as_mut_ptr() as u32,
                out_hi.as_mut_ptr() as u32
            );
            for k in 0..3 {
                hi[k] = fold_max(hi[k], f32::from_bits(out_hi[k]));
                lo[k] = fold_min(lo[k], f32::from_bits(out_lo[k]));
            }
            p = p.wrapping_add(STRIDE);
        }
        ((arg4) as *mut u32).write_unaligned(1);
        lf_checker_rt::callee_cdecl!(
            CAL_SINK,
            u32,
            lo.as_mut_ptr() as u32,
            hi.as_mut_ptr() as u32,
            arg0,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5
        )
    }
});
