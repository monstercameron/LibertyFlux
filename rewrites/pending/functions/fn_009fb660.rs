// original: 0x009fb660 pack_two_floats_to_words
/// Truncate an `f32` toward zero exactly like x86 `cvttss2si` (32-bit).
///
/// Out-of-range values, infinities and NaN yield `i32::MIN` (the invalid
/// result), unlike a saturating cast.
#[inline(always)]
fn trunc_f32_to_i32(x: f32) -> i32 {
    if x.is_nan() || x >= 2_147_483_648.0 || x < -2_147_483_648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
/// Truncates two floats at `src` toward zero and stores the low 16 bits of
/// each at `dst`. Returns the second conversion as left by the convert
/// instruction (out-of-range, infinite and NaN inputs yield `0x80000000`).
export!(cdecl, rw_rs227_009fb660(src: u32, dst: u32) -> u32 {
    unsafe {
        let x0 = *(src as *const f32);
        let x1 = *((src + 4) as *const f32);
        let lo0 = trunc_f32_to_i32(x0);
        let lo1 = trunc_f32_to_i32(x1);
        *(dst as *mut u16) = lo0 as u16;
        *((dst + 2) as *mut u16) = lo1 as u16;
        lo1 as u32
    }
});
