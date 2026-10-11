// original: 0x005f33e0 CopyStreamableRecordFields

/// Copies nine 32-bit words from the source record into this object's sparse
/// streamable-record fields. The gaps are preserved, and EAX returns the last
/// copied source word. The source is the method's one stack argument.
lf_checker_rt::export!(thiscall, rw_005f33e0(this: u32, source: u32) -> u32 {
    const FIELD_COPIES: [(u32, u32); 9] = [
        (0x00, 0x1a0), (0x04, 0x1a4), (0x08, 0x1a8),
        (0x10, 0x1b0), (0x14, 0x1b4), (0x18, 0x1b8),
        (0x20, 0x1c0), (0x24, 0x1c4), (0x28, 0x1c8),
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
