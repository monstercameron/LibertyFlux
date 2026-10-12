// original: 0x00886000 audqt_stream_reader_get_position_ms (proposed)

/// Return the position in milliseconds while the reader is active.
///
/// The active byte is at offset `0x14`, the signed frame count at `0xa8`, and
/// the frames-per-unit divisor at `0xcc`. The original converts the integer
/// to `f32`, divides in that order, then truncates with the x86 SSE conversion
/// result (`i32::MIN` for NaN or an out-of-range result). An inactive reader
/// returns zero.
lf_checker_rt::export!(thiscall, rw_00886000(this: u32) -> u32 {
    unsafe {
        const ACTIVE_OFFSET: u32 = 0x14;
        const FRAME_COUNT_OFFSET: u32 = 0xA8;
        const FRAME_DIVISOR_OFFSET: u32 = 0xCC;
        if (this.wrapping_add(ACTIVE_OFFSET) as *const u8).read() == 0 { return 0; }
        let frame_count = (this.wrapping_add(FRAME_COUNT_OFFSET) as *const i32).read_unaligned();
        let divisor = f32::from_bits((this.wrapping_add(FRAME_DIVISOR_OFFSET) as *const u32).read_unaligned());
        let numerator = core::hint::black_box(frame_count as f32);
        let ratio = core::hint::black_box(numerator) / core::hint::black_box(divisor);
        let value = if !ratio.is_finite() || ratio >= 2147483648.0 || ratio < -2147483648.0 { i32::MIN } else { ratio as i32 };
        value as u32
    }
});
