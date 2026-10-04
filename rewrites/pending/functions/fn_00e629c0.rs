// original: 0x00e629c0 audio_store_level_ratio
/// Store the ratio of two audio level words into the level slot.
///
/// Divides the reference level by the measured level as 32-bit floats and
/// stores the quotient. Pure computation over globals, no calls.
export!(cdecl, rw_00e629c0() -> u32 {
    unsafe {
        const NUM_ADDR: u32 = 0x0103_0B80;
        const DEN_ADDR: u32 = 0x0103_0B84;
        const OUT_ADDR: u32 = 0x0116_1514;
        let num = *(global::<f32>(NUM_ADDR) as *const f32);
        let den = *(global::<f32>(DEN_ADDR) as *const f32);
        *(global::<f32>(OUT_ADDR)) = num / den;
        0
    }
});
