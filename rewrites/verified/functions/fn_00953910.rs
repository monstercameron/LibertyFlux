// original: 0x00953910 clock_span_seconds
/// Ticks elapsed between the two shared clock words, scaled to seconds.
///
/// The unsigned span converts exactly through a wider float and the shared
/// millisecond scale factor; the result returns in ST0.
export!(cdecl, rw_00953910() -> f32 {
    unsafe {
        let span = (*global::<u32>(0x11F702C)).wrapping_sub(*global::<u32>(0x11F7028));
        let wide = (span as i32) as f64
            + f64::from_bits(if span >> 31 == 0 { 0 } else { 0x41F0000000000000 });
        (wide as f32) * f32::from_bits(0x3A83126F)
    }
});
