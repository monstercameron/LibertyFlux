// original: 0x0065FB80 rage::snEventJoinFailed::snEventJoinFailed

/// Constructs a join-failure event in the supplied object. It installs the relocated vtable, self pointer, and two zero words, copies four words from the source record, then delegates construction of a nested subobject at offset 0x20 using the source's 16-byte subobject. The final three stack arguments are copied to offsets 0x58, 0x5c, and 0x60. The base helper is scripted; this thiscall constructor returns `this` and pops four stack arguments.
lf_checker_rt::export!(thiscall, rw_0065fb80(object: *mut u8, source_record: u32, field_58: u32, field_5c: u32, field_60: u32) -> u32 {
    let object_address: u32 = object as u32;
    const VTABLE_FILE_VA: u32 = 0x00FE3488;
    unsafe { *((object_address) as *mut u32) = lf_checker_rt::relocated(VTABLE_FILE_VA); }
    unsafe { *((object_address.wrapping_add(4)) as *mut u32) = object_address; }
    unsafe { *((object_address.wrapping_add(8)) as *mut u32) = 0; }
    unsafe { *((object_address.wrapping_add(12)) as *mut u32) = 0; }
    for word_index in 0..4u32 {
        let value = unsafe { *((source_record.wrapping_add(word_index * 4)) as *const u32) };
        unsafe { *((object_address.wrapping_add(0x10 + word_index * 4)) as *mut u32) = value; }
    }
    let nested_object = object_address.wrapping_add(0x20);
    let nested_source = source_record.wrapping_add(0x10);
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, nested_object, nested_source);
    unsafe { *((object_address.wrapping_add(0x58)) as *mut u32) = field_58; }
    unsafe { *((object_address.wrapping_add(0x5c)) as *mut u32) = field_5c; }
    let final_tail = if false { field_60.wrapping_add(1) } else { field_60 };
    unsafe { *((object_address.wrapping_add(0x60)) as *mut u32) = final_tail; }
    object_address
});
