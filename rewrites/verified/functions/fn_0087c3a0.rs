// original: 0x0087c3a0 rage::crmtNodeBlend::vf14
/// Clamp a float argument into [0, MAX] and store it at `this+0x20`.
///
/// MAX is the global 1.0. Both bounds use ordered comparisons: values below
/// 0 become +0.0, values above MAX become MAX, and a NaN fails both tests
/// and is stored unchanged. -0.0 is stored unchanged. Computes no return
/// value (the original leaves the incoming accumulator untouched).
///
/// Original: thiscall/1, no calls.
export!(thiscall, rw_0087c3a0(this: u32, x_bits: u32) -> u32 {
    /// Global clamp upper bound, 1.0 (file VA; read relocated).
    const MAX_OFF: u32 = 0x00FE88E8;
    /// Destination of the clamped value.
    const DST_OFF: u32 = 0x20;
    unsafe {
        let x = f32::from_bits(x_bits);
        let max = *global::<f32>(MAX_OFF);
        let clamped = if x < 0.0 {
            0.0f32
        } else if x > max {
            max
        } else {
            x
        };
        ((this + DST_OFF) as *mut f32).write_unaligned(clamped);
        0
    }
});
