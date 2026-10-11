// original: 0x006948B0 blend_shape_copy_source_records_006948B0 (proposed)

/// Update a blend-shape component's record count and copy its source records.
///
/// ECX points to the destination component. The first stack argument points
/// to a source object whose record pointer is at `+0` and unsigned 16-bit
/// count is at `+4`. When the count changes, the component's count words at
/// `+4` and `+6` are updated and a new `count * 20` byte block is requested
/// through the intercepted allocator helper; a zero count clears the owned
/// pointer at `+0`. Each record copies 20 bytes. The final source word is the
/// EAX return value, or zero when there are no records.
lf_checker_rt::export!(thiscall, rw_006948B0(component: u32, source_context: u32) -> u32 {
    unsafe {
        const COMPONENT_RECORDS_OFFSET: u32 = 0;
        const COMPONENT_COUNT_OFFSET: u32 = 4;
        const COMPONENT_SECOND_COUNT_OFFSET: u32 = 6;
        const SOURCE_RECORDS_OFFSET: u32 = 0;
        const SOURCE_COUNT_OFFSET: u32 = 4;
        const RECORD_SIZE: u32 = 20;

        let source_count = *((source_context.wrapping_add(SOURCE_COUNT_OFFSET)) as *const u16);
        let current_count = *((component.wrapping_add(COMPONENT_COUNT_OFFSET)) as *const u16);
        if source_count != current_count {
            *((component.wrapping_add(COMPONENT_COUNT_OFFSET)) as *mut u16) = source_count;
            *((component.wrapping_add(COMPONENT_SECOND_COUNT_OFFSET)) as *mut u16) = source_count;
            let new_records = if source_count == 0 {
                0
            } else {
                lf_checker_rt::callee_stdcall!(1, u32, source_count as u32)
            };
            *((component.wrapping_add(COMPONENT_RECORDS_OFFSET)) as *mut u32) = new_records;
        }

        let destination_records = *((component.wrapping_add(COMPONENT_RECORDS_OFFSET)) as *const u32);
        let source_records = *((source_context.wrapping_add(SOURCE_RECORDS_OFFSET)) as *const u32);
        let mut record_index = 0u32;
        let mut last_value = 0u32;
        while record_index < source_count as u32 {
            let record_offset = record_index.wrapping_mul(RECORD_SIZE);
            let source_record = source_records.wrapping_add(record_offset);
            let destination_record = destination_records.wrapping_add(record_offset);
            let first_pair = *((source_record) as *const u64);
            let second_pair = *((source_record.wrapping_add(8)) as *const u64);
            last_value = *((source_record.wrapping_add(16)) as *const u32);
            *((destination_record) as *mut u64) = first_pair;
            *((destination_record.wrapping_add(8)) as *mut u64) = second_pair;
            *((destination_record.wrapping_add(16)) as *mut u32) = last_value;
            record_index += 1;
        }
        last_value
    }
});
