// original: 0x00897280 aud_environment_sound_initialize_globals

/// Initialize the audio-environment stream selector and publish the fixed
/// default table pointer. This routine takes no arguments and changes only
/// those two global words; it does not define a return value.
export!(cdecl, rw_00897280() -> u32 {
    unsafe {
        const STREAM_SELECTOR: u32 = 0x0103_03C0;
        const DEFAULT_TABLE_SLOT: u32 = 0x0115_F7F4;
        const DEFAULT_TABLE: u32 = 0x0115_DEF0;

        (lf_checker_rt::global::<u8>(STREAM_SELECTOR)).write(0x0F);
        (lf_checker_rt::global::<u32>(DEFAULT_TABLE_SLOT))
            .write(lf_checker_rt::relocated(DEFAULT_TABLE));
        0
    }
});
