// original: 0x00B82B80 script_vm_find_first_matching_tagged_record

/// Search at most 140 records with a 44-byte stride. Compare the byte at
/// offset 0x18 with the low byte of `tag_word`, ask the predicate for matches,
/// and return the first accepted index or `u32::MAX`.
lf_checker_rt::export!(thiscall, rw_00b82b80(this: u32, predicate_context: u32, tag_word: u32) -> u32 {
    const RECORD_COUNT: u32 = 0x8c;
    const RECORD_STRIDE: usize = 0x2c;
    const TAG_OFFSET: usize = 0x18;
    let rows = this as *const u8;
    let wanted_tag = tag_word as u8;
    for index in 0..RECORD_COUNT {
        let record = unsafe { rows.add(index as usize * RECORD_STRIDE) };
        let record_tag = unsafe { core::ptr::read_unaligned(record.add(TAG_OFFSET)) };
        if record_tag == wanted_tag {
            let accepted = unsafe {
                lf_checker_rt::callee_cdecl!(1, u32, record as u32, predicate_context)
            };
            if accepted == 0 {
                return index;
            }
        }
    }
    u32::MAX
});
