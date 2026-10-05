// original: 0x008FC4E0 text_scale_convert_exp
/// Widen a float to double, run the double helper, narrow back.
///
/// Converts the float argument to f64 exactly, passes the double to
/// the helper routine (which answers in ST0), then narrows the
/// answer back to float with round-to-nearest-even and returns it in
/// ST0. Cdecl, one stack argument, ST0 return channel.
export!(cdecl, rw_008fc4e0(f: u32) -> f32 {
    unsafe {
        let d = f32::from_bits(f) as f64;
        let bits = d.to_bits();
        let ans: f64 = callee_cdecl!(1, f64,
            (bits & 0xFFFFFFFF) as u32, (bits >> 32) as u32);
        ans as f32
    }
});
