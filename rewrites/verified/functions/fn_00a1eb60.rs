// original: 0x00a1eb60 cam_gate_negate_zero (proposed)

/// Conditionally zeroes a slot and writes -0.0 through an out-pointer.
///
/// `a` and `b` point to records whose floats at `+FIELD_OFF` are combined
/// as `u = b.field - (a.field * c)` in that operation order; `flag` is a
/// gate word, `d` a float argument and `e` an out-pointer. When the flag
/// is zero, or `d` is not strictly above `u`, or the slot `*e` is not
/// strictly above zero, nothing happens. Otherwise the word at
/// `this + CLEAR_OFF` is zeroed and `*e` is set to -0.0. NaN anywhere in
/// the comparisons takes the do-nothing path. Returns nothing.
///
/// Original: 0x00a1eb60 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00a1eb60(this: u32, a: u32, b: u32, c: u32, flag: u32, d: u32, e: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 8;
        const CLEAR_OFF: u32 = 0x310;
        const NEG_ZERO: u32 = 0x8000_0000;
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        unsafe fn rdf(x: u32) -> f32 {
            unsafe { f32::from_bits((x as *const u32).read_unaligned()) }
        }
        if flag == 0 {
            return 0;
        }
        let t = mul(rdf(a + FIELD_OFF), f32::from_bits(c));
        let u = sub(rdf(b + FIELD_OFF), t);
        if !(f32::from_bits(d) > u) {
            return 0;
        }
        if !(rdf(e) > 0.0) {
            return 0;
        }
        ((this + CLEAR_OFF) as *mut u32).write_unaligned(0);
        (e as *mut u32).write_unaligned(NEG_ZERO);
        0
    }
});
