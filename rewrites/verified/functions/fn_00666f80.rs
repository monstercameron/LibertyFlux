// original: 0x00666F80 insert_keyframe

/// Inserts an empty 48-byte slot at `index` in the keyframe array described
/// by the object. The current count is an unsigned 16-bit field, zero-extended
/// before a signed comparison with the 32-bit index. When count exceeds the
/// index, records from the end down through the insertion point move one slot
/// right, with all twelve words of each record copied. The 16-bit count then
/// increments with wrapping, and the function returns `data + index * 48`.
/// The caller must provide spare capacity and an in-range index when a shift
/// is required; those preconditions match the surrounding collection API.
lf_checker_rt::export!(thiscall, rw_00666f80(array: *mut u8, index: u32) -> u32 {
    const DATA_POINTER_OFFSET: u32 = 0;
    const COUNT_OFFSET: u32 = 4;
    const RECORD_BYTES: u32 = 0x30;
    const WORD_BYTES: u32 = 4;
    const WORDS_PER_RECORD: u32 = 12;

    let array_address: u32 = array as u32;
    let data_address = unsafe { *((array_address + DATA_POINTER_OFFSET) as *const u32) };
    let current_count = unsafe { *((array_address + COUNT_OFFSET) as *const u16) };
    if i32::from(current_count) > index as i32 {
        let mut source_index = u32::from(current_count) - 1;
        loop {
            let source_record = data_address.wrapping_add(source_index.wrapping_mul(RECORD_BYTES));
            let destination_record = source_record.wrapping_add(RECORD_BYTES);
            for word_index in 0..WORDS_PER_RECORD {
                let byte_offset = word_index * WORD_BYTES;
                let value = unsafe { *((source_record.wrapping_add(byte_offset)) as *const u32) };
                unsafe { *((destination_record.wrapping_add(byte_offset)) as *mut u32) = value; }
            }
            if source_index == index { break; }
            source_index -= 1;
        }
    }
    let updated_count: u16 = current_count.wrapping_add(1);
    unsafe { *((array_address + COUNT_OFFSET) as *mut u16) = updated_count; }
    data_address.wrapping_add(index.wrapping_mul(RECORD_BYTES))
});
