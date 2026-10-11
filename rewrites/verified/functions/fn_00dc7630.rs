// original: 0x00DC7630 task_record_set_type

/// Move the current task type into the previous-type field and install a new type.
///
/// The thiscall receiver is a task record. The function copies the 32-bit value
/// at offset 0x84 to offset 0x88, marks the record's byte at offset 0x80 as set,
/// then stores the stack argument at offset 0x84. It returns no value and pops
/// its one stack argument.
lf_checker_rt::export!(thiscall, rw_00DC7630(record: u32, new_type: u32) -> u32 {
    unsafe {
        const TYPE_ACTIVE: u32 = 0x80;
        const CURRENT_TYPE: u32 = 0x84;
        const PREVIOUS_TYPE: u32 = 0x88;

        let current_type = (record.wrapping_add(CURRENT_TYPE) as *const u32).read_unaligned();
        (record.wrapping_add(PREVIOUS_TYPE) as *mut u32).write_unaligned(current_type);
        (record.wrapping_add(TYPE_ACTIVE) as *mut u8).write(1);
        (record.wrapping_add(CURRENT_TYPE) as *mut u32).write_unaligned(new_type);
    }
    0
});
