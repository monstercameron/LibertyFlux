// original: 0x00e675e0 update_display_ratio_b
/// Recomputes the second display-aspect ratio from its two global operands.
///
/// Divides the width stored in the first global word by the height in the
/// second and stores the quotient in the ratio global. Single-precision
/// divide, so zeros and NaNs behave exactly as the hardware instruction.
export!(cdecl, rw_00e675e0() -> () {
    unsafe {
        const WIDTH_BITS: u32 = 0x0103CDD0;
        const HEIGHT_BITS: u32 = 0x0103CDD4;
        const RATIO_OUT: u32 = 0x012F9D38;
        let width = global::<f32>(WIDTH_BITS).read();
        let height = global::<f32>(HEIGHT_BITS).read();
        global::<f32>(RATIO_OUT).write(width / height);
    }
});
