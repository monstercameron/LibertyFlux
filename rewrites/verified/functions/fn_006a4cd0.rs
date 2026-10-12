// original: 0x006A4CD0 visit_blend_shape_resource_slot_10

/// Walks the object's group array, then each group's item array, resolves each
/// resource through the item's indexed pointer table, and invokes the resource
/// virtual method at byte slot 0x10. The receiver is passed in ECX;
/// the group and item counts are unsigned 16-bit values. The routine has no
/// scalar return value and preserves the original fault behavior for bad pointers.
lf_checker_rt::export!(thiscall, rw_006a4cd0(
    this: u32,
) -> () {
    const GROUP_ARRAY: u32 = 0x04;
    const GROUP_COUNT: u32 = 0x08;
    const ITEM_ARRAY: u32 = 0x88;
    const ITEM_COUNT: u32 = 0x8C;
    const RESOURCE_POINTERS: u32 = 0x50;
    const RESOURCE_INDEX: u32 = 0x68;
    const VTABLE_SLOT: u32 = 0x10;
    const POINTER_BYTES: u32 = 4;

    let read_u16 = |address: u32| unsafe { (address as *const u16).read_unaligned() };
    let read_u32 = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let group_array = read_u32(this.wrapping_add(GROUP_ARRAY));
    let group_count = read_u16(this.wrapping_add(GROUP_COUNT)) as u32;
    let mut group_index = 0u32;

    while group_index < group_count {
        let group = read_u32(group_array.wrapping_add(group_index.wrapping_mul(POINTER_BYTES)));
        let item_array = read_u32(group.wrapping_add(ITEM_ARRAY));
        let item_count = read_u16(group.wrapping_add(ITEM_COUNT)) as u32;
        let mut item_index = 0u32;

        while item_index < item_count {
            let item = read_u32(item_array.wrapping_add(item_index.wrapping_mul(POINTER_BYTES)));
            let resource_index = read_u32(item.wrapping_add(RESOURCE_INDEX));
            let resource = read_u32(
                item.wrapping_add(RESOURCE_POINTERS)
                    .wrapping_add(resource_index.wrapping_mul(POINTER_BYTES)),
            );
            let vtable = read_u32(resource);
            let target = read_u32(vtable.wrapping_add(VTABLE_SLOT));
            let method: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(target as usize)
            };
            let _ = method(resource);
            item_index = item_index.wrapping_add(1);
        }

        group_index = group_index.wrapping_add(1);
    }
});
