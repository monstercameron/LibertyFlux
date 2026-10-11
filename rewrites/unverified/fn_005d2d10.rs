// original: 0x005D2D10 blip_label_for_display_mode

/// Return the relocated label string for the low three bits of the display
/// mode stored at `this + 0x11`. Modes zero through six select fixed labels;
/// mode seven returns a null pointer. The input byte is sign-extended by the
/// original before masking, which has the same low-three-bit result as this
/// implementation. The function has no stack arguments and does not write
/// memory.
lf_checker_rt::export!(thiscall, rw_005d2d10(this: u32) -> u32 {
    unsafe {
        const DISPLAY_MODE: usize = 0x11;
        const ORDER_LABELS: [u32; 7] = [
            0x00F8FD38, 0x00F8FD18, 0x00F8FDC8, 0x00F8FDAC,
            0x00F8FD90, 0x00F8FD74, 0x00F8FDFC,
        ];

        let mode = *((this + DISPLAY_MODE as u32) as *const u8) & 0x07;
        if mode < ORDER_LABELS.len() as u8 {
            lf_checker_rt::relocated(ORDER_LABELS[mode as usize])
        } else {
            0
        }
    }
});
