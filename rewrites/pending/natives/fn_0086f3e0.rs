// original: 0x0086f3e0 FLOOR
/// Script native `FLOOR` (hash 0x49261BA6).
///
/// Converts one float script argument to an integer with floor rounding
/// and stores it into the return slot. The conversion is a branchless SSE
/// sequence: values with magnitude below 2^23 are rounded to an integer
/// by adding and subtracting a sign-matched power of two, then adjusted
/// down by one when the rounded value exceeds the input; larger
/// magnitudes (already integral) and non-finite inputs pass through the
/// same steps. Truncation toward zero with the x86 convert instruction
/// finishes the job, yielding the integer-minimum bit pattern for
/// out-of-range and NaN inputs. The three constants (sign mask, 2^23,
/// 1.0) are read from the executable's read-only data. No engine call
/// is made.
export!(cdecl, rw_0086f3e0(ctx: *const u8) -> u32 {
    unsafe {
        let arg = *((*(ctx.add(8) as *const u32)) as *const f32);
        let slot = *(ctx as *const u32) as *mut i32;
        let result = floor_sse(
            arg,
            *(global::<f32>(0x00FE8D1C) as *const f32),
            *(global::<f32>(0x00FE8CF8) as *const f32),
            *(global::<f32>(0x00FE88E8) as *const f32),
        );
        *slot = result;
        slot as u32
    }
});

/// The branchless SSE floor core of [`rw_0086f3e0`], kept in its own
/// function so the SSE requirement is declared exactly where it applies.
#[target_feature(enable = "sse")]
unsafe fn floor_sse(x: f32, sign_mask: f32, mag: f32, one: f32) -> i32 {
    use core::arch::x86::*;
    let x = _mm_load_ss(&x);
    let sign = _mm_and_ps(_mm_load_ss(&sign_mask), x);
    let mut m = _mm_load_ss(&mag);
    let abs_lt = _mm_cmplt_ss(_mm_xor_ps(x, sign), m);
    m = _mm_or_ps(_mm_and_ps(m, abs_lt), sign);
    let mut r = _mm_sub_ss(_mm_add_ss(x, m), m);
    let over = _mm_and_ps(
        _mm_cmpnle_ss(_mm_sub_ss(r, x), sign),
        _mm_load_ss(&one),
    );
    r = _mm_sub_ss(r, over);
    _mm_cvttss_si32(r)
}

/// The branchless SSE floor core of [`rw_0086f3e0`], kept in its own
/// function so the SSE requirement is declared exactly where it applies.
#[target_feature(enable = "sse")]
unsafe fn floor_sse(x: f32, sign_mask: f32, mag: f32, one: f32) -> i32 {
    use core::arch::x86::*;
    let x = _mm_load_ss(&x);
    let sign = _mm_and_ps(_mm_load_ss(&sign_mask), x);
    let mut m = _mm_load_ss(&mag);
    let abs_lt = _mm_cmplt_ss(_mm_xor_ps(x, sign), m);
    m = _mm_or_ps(_mm_and_ps(m, abs_lt), sign);
    let mut r = _mm_sub_ss(_mm_add_ss(x, m), m);
    let over = _mm_and_ps(
        _mm_cmpnle_ss(_mm_sub_ss(r, x), sign),
        _mm_load_ss(&one),
    );
    r = _mm_sub_ss(r, over);
    _mm_cvttss_si32(r)
}
