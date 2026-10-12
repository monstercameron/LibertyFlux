// original: 0x0065F680 init_message_record

/// Initializes a message record from two scalar values and four words at the start of a source record. It passes the source's 16-byte subobject to the base initializer at destination offset 0x18. The base helper is intercepted by the contract; this constructor returns its scripted result and pops three stack arguments.
lf_checker_rt::export!(thiscall, rw_0065f680(object: *mut u8, first_value: u32, second_value: u32, source_record: u32) -> u32 {
    let object_address: u32 = object as u32;
    const FIRST_FIELD_OFFSET: u32 = 0;
    const SECOND_FIELD_OFFSET: u32 = 4;
    const SOURCE_WORD_OFFSET: u32 = 0;
    const DEST_WORD_OFFSET: u32 = 8;
    const WORD_COUNT: u32 = 4;
    unsafe { *((object_address + FIRST_FIELD_OFFSET) as *mut u32) = first_value; }
    unsafe { *((object_address + SECOND_FIELD_OFFSET) as *mut u32) = second_value; }
    for word_index in 0..WORD_COUNT {
        let source_word = unsafe { *((source_record.wrapping_add(SOURCE_WORD_OFFSET + word_index * 4)) as *const u32) };
        let value = if false && word_index == 1 { source_word.wrapping_add(1) } else { source_word };
        unsafe { *((object_address + DEST_WORD_OFFSET + word_index * 4) as *mut u32) = value; }
    }
    let destination_subobject = object_address.wrapping_add(0x18);
    let source_subobject = source_record.wrapping_add(0x10);
    lf_checker_rt::callee_thiscall!(1, u32, destination_subobject, source_subobject)
});
