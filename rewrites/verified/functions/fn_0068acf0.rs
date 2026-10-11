// original: 0x0068ACF0 blend_shape_component_copy_record_fields_0068ACF0 (proposed)

/// Copy selected blend-shape values from a source component into matching records.
///
/// The object in ECX points to a component at offset zero. That component has
/// an unsigned 16-bit record count at `+0x10` and a destination pointer array
/// at `+0x0c`. The first stack argument points to a source object whose pointer
/// array is also at `+0x0c`; the other two stack arguments are unused. Each
/// source record with flag `0x10` set is skipped. Otherwise, destination types
/// zero and one copy four words from source offset `+0x10`; every other type
/// copies only the first word. Types zero through two also have destination
/// flag `0x10` cleared. The method does not return a meaningful value.
lf_checker_rt::export!(thiscall, rw_0068ACF0(wrapper: u32, source_wrapper: u32, _reserved0: u32, _reserved1: u32) -> () {
    unsafe {
        const COMPONENT_OFFSET: u32 = 0;
        const RECORDS_OFFSET: u32 = 0x0c;
        const COUNT_OFFSET: u32 = 0x10;
        const RECORD_POINTER_BYTES: u32 = 4;
        const SOURCE_KIND_OFFSET: u32 = 4;
        const DEST_KIND_OFFSET: u32 = 4;
        const VALUE_OFFSET: u32 = 0x10;
        const RECORD_WORD_COUNT: u32 = 4;
        const MIRRORED_FLAG: u8 = 0x10;

        let component = *((wrapper.wrapping_add(COMPONENT_OFFSET)) as *const u32);
        let record_count = *((component.wrapping_add(COUNT_OFFSET)) as *const u16) as u32;
        if record_count == 0 {
            return;
        }

        let destination_records = *((component.wrapping_add(RECORDS_OFFSET)) as *const u32);
        let source_records = *((source_wrapper.wrapping_add(RECORDS_OFFSET)) as *const u32);
        let mut record_index = 0u32;
        while record_index < record_count {
            let pointer_offset = record_index.wrapping_mul(RECORD_POINTER_BYTES);
            let destination = *((destination_records.wrapping_add(pointer_offset)) as *const u32);
            let source = *((source_records.wrapping_add(pointer_offset)) as *const u32);
            let source_flags = *((source.wrapping_add(SOURCE_KIND_OFFSET)) as *const u8);
            if source_flags & MIRRORED_FLAG == 0 {
                let destination_kind = *((destination.wrapping_add(DEST_KIND_OFFSET)) as *const u8) & 0x0f;
                match destination_kind {
                    0 | 1 => {
                        let mut word_index = 0u32;
                        while word_index < RECORD_WORD_COUNT {
                            let value_offset = VALUE_OFFSET.wrapping_add(word_index * 4);
                            let value = *((source.wrapping_add(value_offset)) as *const u32);
                            *((destination.wrapping_add(value_offset)) as *mut u32) = value;
                            word_index += 1;
                        }
                    }
                    _ => {
                        let value = *((source.wrapping_add(VALUE_OFFSET)) as *const u32);
                        *((destination.wrapping_add(VALUE_OFFSET)) as *mut u32) = value;
                    }
                }
                if destination_kind <= 2 {
                    let destination_flags = (destination.wrapping_add(DEST_KIND_OFFSET)) as *mut u8;
                    *destination_flags &= !MIRRORED_FLAG;
                }
            }
            record_index += 1;
        }
    }
});
