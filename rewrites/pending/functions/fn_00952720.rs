// original: 0x00952720 scaled_float_report
/// Combines two probed floats with scaled integer inputs and reports the sum.
///
/// A gathering call fills two locals (only the integer one is used), a lookup
/// yields a field word, and two float probes supply the base values. The
/// integer input scales by a thousandth, the field word scales by a thousandth
/// as an unsigned value, and the result is base one plus the scaled integer
/// minus the scaled field plus base two. Returns the sum on the x87 stack.
export!(cdecl, rw_00952720() -> f64 {
    unsafe {
        const GATHER_A: u32 = 0x011F_7030;
        const GATHER_B: u32 = 0x011F_6F34;
        const LOOKUP_OBJ: u32 = 0x011F_6954;
        const FIELD_OFF: u32 = 0x3C;
        const SCALE: u32 = 0x00FE_86B4;
        let g0 = *global::<u32>(GATHER_A);
        let g1 = *global::<u32>(GATHER_B);
        let obj = *global::<u32>(LOOKUP_OBJ);
        let mut out_b: u32 = 0;
        let mut out_a: u32 = 0;
        callee_cdecl!(
            1,
            u32,
            g0,
            &mut out_b as *mut u32 as u32,
            &mut out_a as *mut u32 as u32,
            g1,
        );
        let found = callee_thiscall!(2, u32, obj, g1);
        let field = *((found.wrapping_add(FIELD_OFF)) as *const u32);
        let base1: f32 = callee_cdecl!(3, f32,);
        let k = *(relocated(SCALE) as *const f32);
        let scaled_int = (out_b as i32) as f32 * k;
        let scaled_field = field as f32 * k;
        let mid = base1 + scaled_int - scaled_field;
        let base2: f32 = callee_thiscall!(4, f32, obj, g1);
        (mid + base2) as f64
    }
});
