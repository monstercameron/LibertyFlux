// original: 0x00DB1430 UILayoutFrame::vf45
/// Calls vtable slot 62 on the receiver, then checks the returned counted array
/// of records. It returns true only when separate visited records contribute
/// both flag bits 0x02 and 0x08; a record with both bits contributes 0x02
/// because the second test is the `else` branch. The count is an unsigned
/// 16-bit value, and zero or one returns false without reading the array. The
/// high 24 return bits retain the query helper's pointer result, matching the
/// original register behavior while the low byte holds the boolean.
lf_checker_rt::export!(thiscall, rw_00DB1430(receiver: *mut u8) -> u32 {
    const VTABLE_METHOD_BYTE_OFFSET: usize = 0xF8;
    const CONTAINER_ARRAY_OFFSET: usize = 0;
    const CONTAINER_COUNT_OFFSET: usize = 4;
    const ITEM_FLAGS_OFFSET: usize = 0;
    const FIRST_FLAG: u8 = 0x02;
    const SECOND_FLAG: u8 = 0x08;
    const COMPLETE_FLAGS: u8 = FIRST_FLAG | SECOND_FLAG;

    // SAFETY: the receiver and its vtable use the original 32-bit object layout.
    let vtable_address = unsafe { receiver.cast::<u32>().read() };
    let method_slot = (VTABLE_METHOD_BYTE_OFFSET / core::mem::size_of::<u32>()) as usize;
    let method_address = unsafe { (vtable_address as *const u32).add(method_slot).read() };
    let get_records: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(method_address as usize) };
    let container_address = get_records(receiver as usize as u32);

    let container = container_address as *const u8;
    // SAFETY: the scripted query returns a container with a pointer and count.
    let record_count = unsafe {
        container.add(CONTAINER_COUNT_OFFSET).cast::<u16>().read_unaligned()
    };
    let mut observed_flags = 0u8;
    if record_count > 1 {
        // SAFETY: the array holds at least `record_count` four-byte pointers.
        let records_address = unsafe {
            container.add(CONTAINER_ARRAY_OFFSET).cast::<u32>().read_unaligned()
        };
        let records = records_address as *const u32;
        for index in 0..usize::from(record_count) {
            // SAFETY: each array entry points at a readable record word.
            let record_address = unsafe { records.add(index).read() };
            let flags = unsafe {
                (record_address as *const u8)
                    .add(ITEM_FLAGS_OFFSET)
                    .read()
            };
            if flags & FIRST_FLAG != 0 {
                observed_flags |= FIRST_FLAG;
            } else if flags & SECOND_FLAG != 0 {
                observed_flags |= SECOND_FLAG;
            }
        }
    }

    (container_address & 0xFFFF_FF00) | u32::from(observed_flags == COMPLETE_FLAGS)
});
