// original: 0x00880F10 aud_voice_pc_adpcm_get_sample_count (proposed)

/// Return the configured sample count for one channel plus one frame.
///
/// The receiver stores the channel's sample count at offset `0x70`. The
/// image's sample multiplier is loaded from a relocated data word. Both the
/// increment and multiplication wrap at 32 bits.
lf_checker_rt::export!(thiscall, rw_00880f10(this: u32) -> u32 {
    unsafe {
        const CHANNEL_SAMPLES_OFFSET: u32 = 0x70;
        const SAMPLE_MULTIPLIER_VA: u32 = 0x01030008;
        let channel_samples = (this.wrapping_add(CHANNEL_SAMPLES_OFFSET) as *const u32).read_unaligned();
        let multiplier = lf_checker_rt::global::<u32>(SAMPLE_MULTIPLIER_VA).read_volatile();
        channel_samples.wrapping_add(1).wrapping_mul(multiplier)
    }
});
