// original: 0x00b415c0 compute_quant_bounds_from_floats
/// Derive the six quantized bound words of an object from its floats.
///
/// Reads five object floats, forms three differences and three sums against
/// the anchor float and constant 1.0, scales each by 8 and truncates toward
/// zero into the min/max word slots. Returns the last truncated value.
export!(thiscall, rw_b415c0(obj: u32) -> u32 {
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
    unsafe {
        let f = |off: usize| (obj as *const f32).byte_add(off).read();
        let anchor = f(0x118);
        let fx = f(0x20);
        let fy = f(0x24);
        let fz = f(0x11c);
        let fw = f(0x120);
        const SCALE: f32 = 8.0;
        let q0 = cvttss2si((fx - anchor) * SCALE);
        let q1 = cvttss2si((fy - anchor) * SCALE);
        let q2 = cvttss2si((fz - 1.0) * SCALE);
        let q3 = cvttss2si((fx + anchor) * SCALE);
        let q4 = cvttss2si((anchor + fy) * SCALE);
        let q5 = cvttss2si((fw + 1.0) * SCALE);
        (obj as *mut u16).byte_add(0x260).write(q0 as u16);
        (obj as *mut u16).byte_add(0x264).write(q1 as u16);
        (obj as *mut u16).byte_add(0x268).write(q2 as u16);
        (obj as *mut u16).byte_add(0x262).write(q3 as u16);
        (obj as *mut u16).byte_add(0x266).write(q4 as u16);
        (obj as *mut u16).byte_add(0x26a).write(q5 as u16);
        q5 as u32
    }
});
