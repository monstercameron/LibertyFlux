// original: 0x006A1CE0 gtaDrawable::vf7

/// The object selects a group at `this + 0x40 + object_index * 4`. The data object
/// contains a table root at `data_object + 4`; the selected group index chooses an
/// 8-byte table entry and `lod_index` selects another 8-byte row. The function
/// calls the global render-state helper once with `helper_argument`, then scans
/// the group's 16-bit item count. Nonzero item entries can pass a bitset filter;
/// eligible rows select a child pointer and call the fastcall worker with that
/// pointer in ECX and zero in EDX. The last zero-extended count is returned. The
/// function uses thiscall and consumes five stack arguments. Its read of the
/// global render-state pointer is supplied by the contract.
lf_checker_rt::export!(thiscall, rw_006a1ce0(
    this: u32,
    helper_argument: u32,
    lod_index: u32,
    object_index: u32,
    data_object: u32,
    unused: u32,
) -> u32 {
    let _ = unused;
    
    let read32 = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let read16 = |address: u32| unsafe { (address as *const u16).read_unaligned() };
    
    
    const DRAW_GROUPS_OFFSET: u32 = 0x40;
    const GROUP_ITEMS_OFFSET: u32 = 0x00;
    const GROUP_COUNT_OFFSET: u32 = 0x04;
    const DATA_ROOT_OFFSET: u32 = 0x04;
    const TABLE_STRIDE: u32 = 8;
    const ITEM_STRIDE: u32 = 4;
    const RENDER_STATE_VA: u32 = 0x017F583C;

    let group_slot = this
        .wrapping_add(DRAW_GROUPS_OFFSET)
        .wrapping_add(object_index.wrapping_mul(ITEM_STRIDE));
    let group = read32(group_slot);

    let table_root = read32(data_object.wrapping_add(DATA_ROOT_OFFSET));
    let lod_table = read32(
        table_root.wrapping_add(object_index.wrapping_mul(TABLE_STRIDE))
    );
    let selected_row_slot = lod_table.wrapping_add(lod_index.wrapping_mul(TABLE_STRIDE));

    let render_state = unsafe { lf_checker_rt::global::<u32>(RENDER_STATE_VA).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, render_state, helper_argument);

    let initial_count = read16(group.wrapping_add(GROUP_COUNT_OFFSET)) as u32;
    if initial_count == 0 {
        return 0;
    }
    

    let mut item_index = 0u32;
    loop {
        let item_table = read32(group.wrapping_add(GROUP_ITEMS_OFFSET));
        let item_present = read32(
            item_table.wrapping_add(item_index.wrapping_mul(ITEM_STRIDE))
        ) != 0;

        if item_present {
            let mask_allows_item = if lod_index == 0 {
                true
            } else {
                let bit_words = read32(lod_index);
                let bit = object_index & 0x1F;
                let word_index = (object_index as i32) >> 5;
                let word_offset = word_index.wrapping_mul(4) as u32;
                (read32(bit_words.wrapping_add(word_offset)) & (1u32 << bit)) != 0
            };

            if mask_allows_item {
                let child_table = read32(selected_row_slot);
                let child = read32(
                    child_table.wrapping_add(object_index.wrapping_mul(ITEM_STRIDE))
                );
                if child != 0 {
                    let _ = lf_checker_rt::callee_fastcall!(2, u32, child, 0);
                }
            }
        }

        let current_count = read16(group.wrapping_add(GROUP_COUNT_OFFSET)) as u32;
        let next_index = item_index.wrapping_add(1);
        if (next_index as i32) < (current_count as i32) {
            item_index = next_index;
            continue;
        }
        return current_count;
    }
    
});