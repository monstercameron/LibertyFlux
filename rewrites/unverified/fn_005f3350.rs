// original: 0x005f3350 CopyIndexedPhysicsRecord

/// Copies twelve sparse 32-bit fields from the source record into the selected
/// 64-byte slot in this object's record array, whose base is at `+0x84`. The
/// slot index and source pointer are stack arguments; EAX returns the last
/// copied source word.
lf_checker_rt::export!(thiscall, rw_005f3350(this: u32, slot_index: u32, source: u32) -> u32 {
    const RECORD_ARRAY: u32 = 0x84;
    const RECORD_STRIDE: u32 = 0x40;
    const FIELD_COPIES: [(u32, u32); 12] = [
        (0x00, 0x00), (0x04, 0x04), (0x08, 0x08),
        (0x10, 0x10), (0x14, 0x14), (0x18, 0x18),
        (0x20, 0x20), (0x24, 0x24), (0x28, 0x28),
        (0x30, 0x30), (0x34, 0x34), (0x38, 0x38),
    ];
    let slots = unsafe { (this.wrapping_add(RECORD_ARRAY) as *const u32).read_unaligned() };
    let slot = slots.wrapping_add(slot_index.wrapping_mul(RECORD_STRIDE));
    let mut last_copied_word = 0;
    for (source_offset, slot_offset) in FIELD_COPIES {
        last_copied_word = unsafe {
            (source.wrapping_add(source_offset) as *const u32).read_unaligned()
        };
        unsafe { (slot.wrapping_add(slot_offset) as *mut u32).write_unaligned(last_copied_word) };
    }
    last_copied_word
});
