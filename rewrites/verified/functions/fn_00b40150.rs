// original: 0x00b40150 quantize_bounds_f32_to_i16
/// Quantize six bound floats into three scaled min/max word pairs.
///
/// Each argument is multiplied by 8, truncated toward zero and stored as the
/// low 16 bits; the first three land on the min slots (offsets 0, 4, 8) and
/// the last three on the max slots (offsets 2, 6, 10). Returns the full
/// 32-bit truncated value of the last argument.
export!(thiscall, rw_b40150(out: u32, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> u32 {
    #[inline(always)]
    fn cvttss2si(x: f32) -> i32 {
        if x.is_nan() {
            return i32::MIN;
        }
        let t = x.trunc();
        if t >= 2147483648.0 || t < -2147483648.0 {
            i32::MIN
        } else {
            t as i32
        }
    }
    const SCALE: f32 = 8.0;
    let q0 = cvttss2si(a0 * SCALE);
    let q1 = cvttss2si(a1 * SCALE);
    let q2 = cvttss2si(a2 * SCALE);
    let q3 = cvttss2si(a3 * SCALE);
    let q4 = cvttss2si(a4 * SCALE);
    let q5 = cvttss2si(a5 * SCALE);
    unsafe {
        (out as *mut u16).byte_add(0).write(q0 as u16);
        (out as *mut u16).byte_add(4).write(q1 as u16);
        (out as *mut u16).byte_add(8).write(q2 as u16);
        (out as *mut u16).byte_add(2).write(q3 as u16);
        (out as *mut u16).byte_add(6).write(q4 as u16);
        (out as *mut u16).byte_add(10).write(q5 as u16);
    }
    q5 as u32
});
