// original: 0x006A4D30 advance_blend_shape_buffer_indices

/// For every group and item, saves the old resource index at offset 0x64,
/// increments the signed 32-bit index at offset 0x68 with wrapping arithmetic,
/// and stores the signed remainder after division by three. The object and its
/// group and item pointer arrays use 32-bit layout; both counts are unsigned
/// 16-bit values. This is a thiscall routine with no declared scalar result.
lf_checker_rt::export!(thiscall, rw_006a4d30(
    this: u32,
) -> () {
    const GROUP_ARRAY: u32 = 0x04;
    const GROUP_COUNT: u32 = 0x08;
    const ITEM_ARRAY: u32 = 0x88;
    const ITEM_COUNT: u32 = 0x8C;
    const PREVIOUS_INDEX: u32 = 0x64;
    const RESOURCE_INDEX: u32 = 0x68;
    const POINTER_BYTES: u32 = 4;
    const BUFFER_COUNT: i32 = 3;

    let read_u16 = |address: u32| unsafe { (address as *const u16).read_unaligned() };
    let read_u32 = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let write_u32 = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };

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
            let old_index = read_u32(item.wrapping_add(RESOURCE_INDEX));
            write_u32(item.wrapping_add(PREVIOUS_INDEX), old_index);
            let next_index = (old_index as i32).wrapping_add(1);
            let next_buffer = next_index % BUFFER_COUNT;
            write_u32(item.wrapping_add(RESOURCE_INDEX), next_buffer as u32);
            item_index = item_index.wrapping_add(1);
        }
        group_index = group_index.wrapping_add(1);
    }
});
