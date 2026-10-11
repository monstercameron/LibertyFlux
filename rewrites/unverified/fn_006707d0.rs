// original: 0x006707D0 streaming_pack_rgb_f32

/// Clamp three binary32 channels to [0, 1], multiply each by 255 in that
/// operand order, truncate toward zero, and store an opaque 0xFFRRGGBB word.
/// ECX points to the output word and the one stack argument points to three
/// input floats in red, green, blue order. NaNs survive the clamp and the
/// x86 truncating conversion contributes a zero low byte.
///
/// Original: 0x006707D0 (thiscall, one stack pointer, packed u32 return).
lf_checker_rt::export!(thiscall, rw_006707d0(this: u32, rgb: u32) -> u32 {
    unsafe {
        const MAX_COMPONENT: f32 = 1.0;
        const SCALE_255: f32 = 255.0;
        #[inline(always)]
        fn scale(component: f32) -> u8 {
            let limited = if component < 0.0 { 0.0 }
                else if component > MAX_COMPONENT { MAX_COMPONENT }
                else { component };
            let left = core::hint::black_box(limited);
            let right = core::hint::black_box(SCALE_255);
            let scaled = left * right;
            let truncated = if scaled.is_nan() { i32::MIN } else { scaled.trunc() as i32 };
            truncated as u8
        }
        let channels = rgb as *const f32;
        let red = scale(channels.add(0).read_unaligned());
        let green = scale(channels.add(1).read_unaligned());
        let blue = scale(channels.add(2).read_unaligned());
        let packed = 0xFF00_0000u32 | ((red as u32) << 16)
            | ((green as u32) << 8) | blue as u32;
        (this as *mut u32).write_unaligned(packed);
        this
    }
});
