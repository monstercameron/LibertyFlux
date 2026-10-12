// original: 0x00C84F90 scrollbar_table_add_entity

/// Add one entity record to the active scrollbar table. The global active
/// table index selects a four-byte counter; counters at signed values of 16
/// or more leave the table unchanged. The routine stores the supplied entity
/// word, ensures the source has a data object, copies its selected fields into
/// a 0x50-byte global record, stores the float argument's bits, then advances
/// the counter. This cdecl routine takes a source pointer, an entity word, and
/// one single-precision value on the stack.
lf_checker_rt::export!(cdecl, rw_00c84f90(source: u32, entity_word: u32, value_bits: u32) -> u32 {
    const ACTIVE_TABLE_INDEX_GLOBAL: u32 = 0x0117_4790;
    const COUNTS_GLOBAL: u32 = 0x016F_AC40;
    const RECORDS_GLOBAL: u32 = 0x016F_AC50;
    const ENTITY_WORDS_GLOBAL: u32 = 0x016F_AC90;
    const RECORD_STRIDE: u32 = 0x50;
    const SOURCE_DATA_POINTER_OFFSET: u32 = 0x20;
    const ENSURE_SOURCE_DATA: u32 = 1;
    const BIND_SOURCE_DATA: u32 = 2;
    const MAX_RECORDS: i32 = 16;

    let active_index = unsafe {
        lf_checker_rt::global::<u32>(ACTIVE_TABLE_INDEX_GLOBAL).read_unaligned()
    };
    let counter_address = COUNTS_GLOBAL.wrapping_add(active_index.wrapping_mul(4));
    let count = unsafe { lf_checker_rt::global::<i32>(counter_address).read_unaligned() };
    if count >= MAX_RECORDS {
        return 0;
    }
    let count_word = count as u32;

    let entity_slot = count_word.wrapping_add(active_index.wrapping_shl(4));
    let entity_address = ENTITY_WORDS_GLOBAL
        .wrapping_add(entity_slot.wrapping_mul(RECORD_STRIDE));
    unsafe {
        lf_checker_rt::global::<u32>(entity_address).write_unaligned(entity_word);
    }

    let mut source_data = unsafe {
        (source.wrapping_add(SOURCE_DATA_POINTER_OFFSET) as *const u32).read_unaligned()
    };
    if source_data == 0 {
        let _ = lf_checker_rt::callee_thiscall!(ENSURE_SOURCE_DATA, u32, source);
        source_data = unsafe {
            (source.wrapping_add(SOURCE_DATA_POINTER_OFFSET) as *const u32).read_unaligned()
        };
        let _ = lf_checker_rt::callee_thiscall!(
            BIND_SOURCE_DATA,
            u32,
            source.wrapping_add(0x10),
            source_data,
        );
    }

    let record_index = active_index.wrapping_add(count_word);
    let record_address = RECORDS_GLOBAL.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
    let source_offsets = [0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
    let destination_offsets = [0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
    let mut field_index = 0;
    while field_index < source_offsets.len() {
        let value = unsafe {
            (source_data.wrapping_add(source_offsets[field_index]) as *const u32).read_unaligned()
        };
        unsafe {
            lf_checker_rt::global::<u32>(
                record_address.wrapping_add(destination_offsets[field_index]),
            )
            .write_unaligned(value);
        }
        field_index += 1;
    }
    unsafe {
        lf_checker_rt::global::<u32>(record_address.wrapping_add(0x40))
            .write_unaligned(entity_word);
        lf_checker_rt::global::<u32>(record_address.wrapping_add(0x44))
            .write_unaligned(value_bits);
        lf_checker_rt::global::<i32>(counter_address).write_unaligned(count + 1);
    }
    0
});
