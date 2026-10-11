// original: 0x008E5340 audio_event_find_free_slot

/// Find an eligible slot in the entity's fourteen-entry audio table.
///
/// Each entry has a signed key at `0x350 + 12*i` and a one-byte occupied flag
/// at `0x359 + 12*i`. The first entry whose key is greater than -1 and whose
/// flag is nonzero is selected. It is eligible only when the signed limit at
/// `0x348` is less than the selected index; then the incoming value is stored
/// in that entry's key and AL returns one. A missing or ineligible entry
/// returns zero. This is a 32-bit thiscall method with one stack word and
/// the callee pops 4 bytes cleanup.
lf_checker_rt::export!(thiscall, rw_008e5340(this: u32, value: u32) -> u32 {
    unsafe {
        const ENTRY_BASE: usize = 0x350;
        const ENTRY_STRIDE: usize = 12;
        const ENTRY_COUNT: usize = 14;
        const FLAG_OFFSET: usize = 9;
        const SIGNED_LIMIT: usize = 0x348;

        let limit = (this as usize as *const i32).add(SIGNED_LIMIT / 4).read_unaligned();
        let mut index = 0usize;
        while index < ENTRY_COUNT {
            let entry = (this as usize as *mut u8).add(ENTRY_BASE + index * ENTRY_STRIDE);
            let key = (entry as *const i32).read_unaligned();
            let occupied = entry.add(FLAG_OFFSET).read();
            if key > -1 && occupied != 0 {
                if limit >= index as i32 {
                    return 0;
                }
                (entry as *mut u32).write_unaligned(value);
                return 1;
            }
            index += 1;
        }
        0
    }
});
