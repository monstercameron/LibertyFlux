// original: 0x005f32e0 CopyPhysicsPayloadFields

/// Copies the twelve listed 32-bit payload words from the source record into
/// this object. The unmatched gaps in both records are left untouched, and the
/// value of the last copied source word is returned. The source is the method's
/// sole stack argument.
lf_checker_rt::export!(thiscall, rw_005f32e0(this: u32, source: u32) -> u32 {
    const FIELD_COPIES: [(u32, u32); 12] = [
        (0x00, 0x10), (0x04, 0x14), (0x08, 0x18),
        (0x10, 0x20), (0x14, 0x24), (0x18, 0x28),
        (0x20, 0x30), (0x24, 0x34), (0x28, 0x38),
        (0x30, 0x40), (0x34, 0x44), (0x38, 0x48),
    ];

    let mut last_copied_word = 0;
    for (source_offset, destination_offset) in FIELD_COPIES {
        last_copied_word = unsafe {
            (source.wrapping_add(source_offset) as *const u32).read_unaligned()
        };
        unsafe {
            (this.wrapping_add(destination_offset) as *mut u32).write_unaligned(last_copied_word)
        };
    }
    last_copied_word
});
