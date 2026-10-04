// original: 0x00bfa8c0 store_scaled_vec3_bytes_1c
//! rs20f5 @0xBFA8C0: scale a float vec3 by the shared factor, truncate each
//! component toward zero and store the low bytes at +0x1c (thiscall/1).
//! Returns the last conversion, like the original's leftover EAX.
/// Emulates `cvttss2si`: truncation toward zero, with NaN, infinities and
/// out-of-range values all producing `0x80000000` (Rust's `as` saturates
/// instead, so the edges need the explicit guard).
#[inline(always)]
fn cvttss2si_rs20(f: f32) -> i32 {
    if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        i32::MIN
    } else {
        f as i32
    }
}

export!(thiscall, rw_rs20f5(this: *mut u8, src: *const u32) -> u32 {
    unsafe {
        let scale = *global::<f32>(0xFE8BC4);
        let mut last = 0i32;
        for i in 0..3usize {
            last = cvttss2si_rs20(f32::from_bits(*src.add(i)) * scale);
            *this.add(0x1c + i) = last as u8;
        }
        last as u32
    }
});
