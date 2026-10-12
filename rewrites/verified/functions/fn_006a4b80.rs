// original: 0x006A4B80 process_blend_shape_resource_buffers

/// Scans each group and its item pointers, then calls the resource's virtual
/// slot at `0x04` while its state byte is zero. It clears the resource buffer using
/// the element count and stride, and may call slot `0x10` if the state changed. A
/// null resource skips the virtual calls but faults on the later count read. The
/// function takes its object in ECX and has no declared scalar return value.
lf_checker_rt::export!(thiscall, rw_006a4b80(
    this: u32,
) -> () {
    const GROUP_ARRAY_OFFSET: u32 = 0x04;
    const GROUP_COUNT_OFFSET: u32 = 0x08;
    const ITEM_ARRAY_OFFSET: u32 = 0x88;
    const ITEM_COUNT_OFFSET: u32 = 0x8C;
    const RESOURCE_POINTER_OFFSET: u32 = 0x50;
    const RESOURCE_INDEX_OFFSET: u32 = 0x68;
    const RESOURCE_STATE_OFFSET: u32 = 0x06;
    const RESOURCE_COUNT_OFFSET: u32 = 0x04;
    const RESOURCE_DEST_OFFSET: u32 = 0x08;
    const RESOURCE_STRIDE_OFFSET: u32 = 0x0C;
    const VTABLE_FIRST_SLOT: u32 = 0x04;
    const VTABLE_STATE_SLOT: u32 = 0x10;
    const MEMSET_ID: u32 = 3;
    const POINTER_SIZE: u32 = 4;

    let read_u8 = |address: u32| unsafe { (address as *const u8).read_unaligned() };
    let read_u16 = |address: u32| unsafe { (address as *const u16).read_unaligned() };
    let read_u32 = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let call_virtual = |object: u32, slot: u32| -> u32 {
        let vtable = read_u32(object);
        let target = read_u32(vtable.wrapping_add(slot));
        let method: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(target as usize)
        };
        method(object)
    };

    let group_array = read_u32(this.wrapping_add(GROUP_ARRAY_OFFSET));
    let group_count = read_u16(this.wrapping_add(GROUP_COUNT_OFFSET)) as u32;
    let mut group_index = 0u32;
    while group_index < group_count {
        let group = read_u32(group_array.wrapping_add(group_index.wrapping_mul(POINTER_SIZE)));
        let item_array = read_u32(group.wrapping_add(ITEM_ARRAY_OFFSET));
        let item_count = read_u16(group.wrapping_add(ITEM_COUNT_OFFSET)) as u32;
        let mut item_index = 0u32;

        while item_index < item_count {
            let item = read_u32(item_array.wrapping_add(item_index.wrapping_mul(POINTER_SIZE)));
            let resource_index = read_u32(item.wrapping_add(RESOURCE_INDEX_OFFSET));
            let resource = read_u32(
                item.wrapping_add(RESOURCE_POINTER_OFFSET)
                    .wrapping_add(resource_index.wrapping_mul(POINTER_SIZE))
            );

            let state_before_first = if resource == 0 {
                0
            } else {
                read_u8(resource.wrapping_add(RESOURCE_STATE_OFFSET))
            };
            if resource != 0 && state_before_first == 0 && true {
                    let _ = call_virtual(resource, VTABLE_FIRST_SLOT);
            }

            let state_before_second = if resource == 0 {
                0
            } else {
                read_u8(resource.wrapping_add(RESOURCE_STATE_OFFSET))
            };
            if resource != 0 && state_before_second == 0 {
                let _ = call_virtual(resource, VTABLE_FIRST_SLOT);
            }

            let element_count = read_u16(resource.wrapping_add(RESOURCE_COUNT_OFFSET)) as u32;
            let stride = read_u32(resource.wrapping_add(RESOURCE_STRIDE_OFFSET));
            let byte_count = element_count.wrapping_mul(stride);
            let destination = read_u32(resource.wrapping_add(RESOURCE_DEST_OFFSET));
            let _ = lf_checker_rt::callee_cdecl!(MEMSET_ID, u32, destination, 0u32, byte_count);

            if resource != 0 && read_u8(resource.wrapping_add(RESOURCE_STATE_OFFSET)) != 0 {
                if state_before_second == 0 {
                    let _ = call_virtual(resource, VTABLE_STATE_SLOT);
                }
                if read_u8(resource.wrapping_add(RESOURCE_STATE_OFFSET)) != 0
                    && state_before_first == 0
                {
                    let _ = call_virtual(resource, VTABLE_STATE_SLOT);
                }
            }
            item_index = item_index.wrapping_add(1);
        }
        group_index = group_index.wrapping_add(1);
    }
});
