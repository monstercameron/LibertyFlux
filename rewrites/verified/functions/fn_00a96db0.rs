// original: 0x00a96db0 fade_sample_append
/// Appends one converted integer triple to the sample table.
///
/// Converts the three integer arguments to floats at the slot selected by
/// the counter, then bumps the counter. Returns twice the slot index used.
export!(stdcall, rw_00a96db0(dst: u32, counter: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let idx = *(counter as *const u32);
        let slot = dst.wrapping_add(idx.wrapping_mul(16));
        *(slot as *mut f32) = (a as i32) as f32;
        *((slot + 4) as *mut f32) = (b as i32) as f32;
        *((slot + 8) as *mut f32) = (c as i32) as f32;
        *(counter as *mut u32) = idx.wrapping_add(1);
        idx.wrapping_mul(2)
    }
});
