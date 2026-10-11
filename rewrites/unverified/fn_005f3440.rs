// original: 0x005f3440 CopyExtendedRecordFields

/// Copies twelve 32-bit words from the source record to this object's sparse
/// extended-record fields. All intervening gaps are left as they were, and
/// EAX returns the last copied source word. The source is one stack argument.
lf_checker_rt::export!(thiscall, rw_005f3440(this: u32, source: u32) -> u32 {
    const FIELD_COPIES: [(u32, u32); 12] = [
        (0x00, 0x200), (0x04, 0x204), (0x08, 0x208),
        (0x10, 0x210), (0x14, 0x214), (0x18, 0x218),
        (0x20, 0x220), (0x24, 0x224), (0x28, 0x228),
        (0x30, 0x230), (0x34, 0x234), (0x38, 0x238),
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
