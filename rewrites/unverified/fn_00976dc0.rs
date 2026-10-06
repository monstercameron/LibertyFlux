// original: 0x00976dc0 audio_byte_to_level (proposed)

/// Map an optional level byte to a float, returned on ST0.
///
/// A null object gives 1.0. Otherwise the byte at +0x1A is converted to
/// float exactly and multiplied by the scale constant from read-only data
/// (single multiply, order pinned: value times constant).
/// Original: 0x00976DC0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00976dc0(obj: u32) -> f32 {
    unsafe {
        const LEVEL_OFF: u32 = 0x1A;
        const SCALE: f32 = f32::from_bits(0x3c23d70a);
        if obj == 0 {
            return 1.0;
        }
        let v = ((obj.wrapping_add(LEVEL_OFF)) as *const u8).read() as f32;
        core::hint::black_box(v) * core::hint::black_box(SCALE)
    }
});
