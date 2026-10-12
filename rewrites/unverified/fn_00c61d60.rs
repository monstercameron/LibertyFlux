// original: 0x00C61D60 CCamCutscene::reset_display_scales

/// Sets the two cutscene display scale globals to the bit pattern for 1.0
/// and returns true in `AL`. The globals are addressed through the checker
/// runtime so their relocated image addresses are used in the worker.
lf_checker_rt::export!(thiscall, rw_00c61d60(_this: u32) -> u8 {
    const HORIZONTAL_SCALE_VA: u32 = 0x0103_234C;
    const VERTICAL_SCALE_VA: u32 = 0x0103_2350;
    const ONE_BITS: u32 = 1.0f32.to_bits();

    unsafe {
        lf_checker_rt::global::<u32>(HORIZONTAL_SCALE_VA).write_unaligned(ONE_BITS);
        lf_checker_rt::global::<u32>(VERTICAL_SCALE_VA).write_unaligned(ONE_BITS);
    }
    1
});
