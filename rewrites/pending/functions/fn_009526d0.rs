// original: 0x009526d0 scaled_counter_as_float
/// Read an unsigned tick counter, convert it to floating point and scale it
/// by a global factor. The original converts through a signed intermediate
/// with a sign-based two-to-the-32 correction, which is exactly an unsigned
/// conversion.
export!(cdecl, rw_009526d0() -> f64 {
    unsafe {
        let ticks = *global::<u32>(0x011f7030);
        let factor = *global::<f32>(0x00fe86b4);
        let scaled = ticks as f64 as f32 * factor;
        scaled as f64
    }
});
