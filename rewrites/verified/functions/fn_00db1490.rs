// original: 0x00DB1490 UILayoutFrame::vf62
/// Checks whether a layout frame can resolve every item in an indexed list.
/// It first calls vtable slot 60 and tests only AL; if that byte is zero, it
/// returns the address of `UILayoutFrame::field_bc`. Otherwise it calls vtable
/// slot 19, passes that result to the static index registry, and follows the
/// registry record's list pointer. A null registry record or null list returns
/// the fallback field address. The list count is an unsigned 16-bit value; for
/// each entry, the function passes the pointed-to item's `field_4` key to the
/// indexed-query helper and tests only AL. Any zero low byte returns the
/// fallback; if every query succeeds, including an empty list, it returns the
/// list pointer. The two virtual calls use the receiver in ECX, and the registry
/// lookup uses its static object in ECX with one stack argument.
lf_checker_rt::export!(thiscall, rw_00DB1490(receiver: *mut u8) -> u32 {
    const FIRST_QUERY_SLOT_BYTE_OFFSET: usize = 0xF0;
    const SECOND_QUERY_SLOT_BYTE_OFFSET: usize = 0x4C;
    const REGISTRY_OBJECT_VA: u32 = 0x0190_8C94;
    const REGISTRY_LIST_OFFSET: usize = 4;
    const LIST_ARRAY_OFFSET: usize = 0;
    const LIST_COUNT_OFFSET: usize = 4;
    const ITEM_KEY_OFFSET: usize = 4;
    const FIELD_BC_OFFSET: u32 = 0xBC;

    let receiver_address = receiver as usize as u32;
    let fallback_address = receiver_address.wrapping_add(FIELD_BC_OFFSET);
    // SAFETY: the receiver and vtable use the original 32-bit object layout.
    let vtable_address = unsafe { receiver.cast::<u32>().read() };
    let first_slot = unsafe {
        (vtable_address as *const u32)
            .add(FIRST_QUERY_SLOT_BYTE_OFFSET / core::mem::size_of::<u32>())
            .read()
    };
    let first_query: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(first_slot as usize) };
    if first_query(receiver_address) as u8 == 0 {
        return fallback_address;
    }

    let second_slot = unsafe {
        (vtable_address as *const u32)
            .add(SECOND_QUERY_SLOT_BYTE_OFFSET / core::mem::size_of::<u32>())
            .read()
    };
    let second_query: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(second_slot as usize) };
    let lookup_key = second_query(receiver_address);
    let registry_record = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        lf_checker_rt::relocated(REGISTRY_OBJECT_VA),
        lookup_key
    );
    if registry_record == 0 {
        return fallback_address;
    }

    let registry_list = unsafe {
        (registry_record as *const u8)
            .add(REGISTRY_LIST_OFFSET)
            .cast::<u32>()
            .read_unaligned()
    };
    if registry_list == 0 {
        return fallback_address;
    }

    let list = registry_list as *const u8;
    // SAFETY: the list record holds the array pointer and unsigned count words.
    let (array_address, item_count) = unsafe {
        (
            list.add(LIST_ARRAY_OFFSET)
                .cast::<u32>()
                .read_unaligned(),
            list.add(LIST_COUNT_OFFSET)
                .cast::<u16>()
                .read_unaligned(),
        )
    };
    let items = array_address as *const u32;
    for item_index in 0..usize::from(item_count) {
        // SAFETY: each list entry points to an item with a 32-bit key at +4.
        let item_address = unsafe { items.add(item_index).read_unaligned() };
        let key = unsafe {
            (item_address as *const u8)
                .add(ITEM_KEY_OFFSET)
                .cast::<u32>()
                .read_unaligned()
        };
        let query_result = lf_checker_rt::callee_cdecl!(4, u32, key);
        if query_result as u8 == 0 {
            return fallback_address;
        }
    }
    registry_list
});
