// original: 0x006A1D50 gtaDrawable::vf11

/// Calls the render-state helper with the image global in ECX and the first
/// stack argument. It selects a group from the object, scans its records from the
/// last index to zero, applies the optional bit mask and category filter, then calls
/// each eligible object's virtual slot at offset `0x08`. The source function has no
/// computed return value. Two incoming stack slots are scratch storage in the
/// original; the Rust rewrite keeps those intermediate values in locals.
lf_checker_rt::export!(thiscall, rw_006a1d50(
    this: u32,
    helper_argument: u32,
    group_index: u32,
    category_filter: u32,
    mask_header: u32,
) -> () {
    const GROUPS_OFFSET: u32 = 0x40;
    const POINTER_SIZE: u32 = 4;
    const GROUP_COUNT_OFFSET: u32 = 0x04;
    const RECORD_COUNT_OFFSET: u32 = 0x1A;
    const CATEGORY_POINTER_OFFSET: u32 = 0x08;
    const CATEGORY_INDEX_OFFSET: u32 = 0x10;
    const ENTITY_ARRAY_OFFSET: u32 = 0x04;
    const VTABLE_SLOT_OFFSET: u32 = 0x08;
    const RENDER_STATE_VA: u32 = 0x017F583C;
    const ALL_CATEGORIES: u32 = u32::MAX;

    let read_u8 = |address: u32| unsafe { (address as *const u8).read_unaligned() };
    let read_u16 = |address: u32| unsafe { (address as *const u16).read_unaligned() };
    let read_u32 = |address: u32| unsafe { (address as *const u32).read_unaligned() };

    let render_state = unsafe { lf_checker_rt::global::<u32>(RENDER_STATE_VA).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, render_state, helper_argument);

    let selected_group = read_u32(
        this.wrapping_add(GROUPS_OFFSET)
            .wrapping_add(group_index.wrapping_mul(POINTER_SIZE))
    );
    let count = read_u16(selected_group.wrapping_add(GROUP_COUNT_OFFSET)) as u32;
    let mut record_index = count.wrapping_sub(1) as i32;
    if record_index < 0 {
        return;
    }

    let mut record_bit = 1u32.rotate_left(record_index as u32);
    while record_index >= 0 {
        let record_is_enabled = if mask_header == 0 {
            true
        } else {
            let mask_words = read_u32(mask_header);
            let word_offset = ((record_index as u32) >> 5).wrapping_mul(POINTER_SIZE);
            (read_u32(mask_words.wrapping_add(word_offset)) & record_bit) != 0
        };

        if record_is_enabled {
            let record_table = read_u32(selected_group);
            let record = read_u32(
                record_table.wrapping_add((record_index as u32).wrapping_mul(POINTER_SIZE))
            );
            let object_count = read_u16(record.wrapping_add(RECORD_COUNT_OFFSET)) as u32;
            let mut object_index = object_count.wrapping_sub(1) as i32;

            while object_index >= 0 {
                let matches_category = if category_filter == ALL_CATEGORIES {
                    true
                } else {
                    let category_indices = read_u32(record.wrapping_add(CATEGORY_INDEX_OFFSET));
                    let category_index = read_u16(
                        category_indices.wrapping_add((object_index as u32).wrapping_mul(2))
                    ) as u32;
                    let category_root = read_u32(this.wrapping_add(CATEGORY_POINTER_OFFSET));
                    let category_table = read_u32(category_root.wrapping_add(CATEGORY_POINTER_OFFSET));
                    let category_object = read_u32(
                        category_table.wrapping_add(category_index.wrapping_mul(POINTER_SIZE))
                    );
                    read_u8(category_object.wrapping_add(9)) as u32 == category_filter
                };

                if matches_category {
                    let entity_array = read_u32(record.wrapping_add(ENTITY_ARRAY_OFFSET));
                    let entity = read_u32(
                        entity_array.wrapping_add((object_index as u32).wrapping_mul(POINTER_SIZE))
                    );
                    let vtable = read_u32(entity);
                    let target = read_u32(vtable.wrapping_add(VTABLE_SLOT_OFFSET));
                    let draw: extern "thiscall" fn(u32) -> u32 = unsafe {
                        core::mem::transmute(target as usize)
                    };
                    let _ = draw(entity);
                }
                object_index -= 1;
            }
        }

        record_bit = record_bit.rotate_right(1);
        record_index -= 1;
    }
});
