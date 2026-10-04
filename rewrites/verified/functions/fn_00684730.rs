// original: 0x00684730 rage::crFrameDofInt::vf7
/// Integer degree-of-freedom scale step.
///
/// Multiplies the live integer (converted to float) by the factor and
/// truncates back to an integer. Matches the original's truncate
/// instruction exactly: NaN and positive overflow produce 0x80000000
/// (Rust's saturating cast already yields 0x80000000 for negative
/// overflow, so only the other two cases need a guard). Returns the result.
export!(thiscall, rw_00684730(this_: *mut u8, factor: f32) -> u32 {
    unsafe {
        let slot = this_.add(VALUE_OFF) as *mut i32;
        let scaled = (*slot as f32) * factor;
        let out: u32 = if scaled.is_nan() || scaled >= 2147483648.0 {
            0x8000_0000
        } else {
            scaled as i32 as u32
        };
        *slot = out as i32;
        out
    }
});
