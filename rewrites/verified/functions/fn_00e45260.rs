// original: 0x00E45260 K_NICO_GT

/// Builds the multiplayer leaderboard object from the active ranking,
/// mode configuration and row metadata. It resolves the selected row keys,
/// writes mode and display fields, invokes the formatting and allocation
/// helpers in order, fills both arrays, and finishes with the shared object
/// update. The rewrite preserves the query, secondary-key and primary-key
/// paths and their distinct field writes.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn read8(base: u32, offset: u32) -> u8 {
    unsafe { (base.wrapping_add(offset) as *const u8).read() }
}

#[inline(always)]
unsafe fn read16(base: u32, offset: u32) -> u16 {
    unsafe { (base.wrapping_add(offset) as *const u16).read() }
}

#[inline(always)]
unsafe fn read32(base: u32, offset: u32) -> u32 {
    unsafe { (base.wrapping_add(offset) as *const u32).read() }
}

#[inline(always)]
unsafe fn write8(base: u32, offset: u32, value: u8) {
    unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
}

#[inline(always)]
unsafe fn write16(base: u32, offset: u32, value: u16) {
    unsafe { (base.wrapping_add(offset) as *mut u16).write(value) }
}

#[inline(always)]
unsafe fn write32(base: u32, offset: u32, value: u32) {
    unsafe { (base.wrapping_add(offset) as *mut u32).write(value) }
}

#[inline(always)]
unsafe fn image_word(base: u32, index: u32) -> u32 {
    let address = (global::<u32>(base) as u32).wrapping_add(index.wrapping_mul(4));
    unsafe { (address as *const u32).read() }
}

export!(thiscall, rw_00e45260(this: u32) -> u32 {
    unsafe {
        const OBJ_KIND_TEXT_A: u32 = 0x00F15BB0;
        const OBJ_KIND_TEXT_B: u32 = 0x00F15BBC;
        const OBJ_KIND_TEXT_C: u32 = 0x00F15BC8;
        const OBJ_KIND_TEXT_D: u32 = 0x00F15BD4;

        let obj = this;
        write8(obj, 0x313, 2);
        write32(obj, 0x390, 0);
        write32(obj, 0x39C, 1);
        write8(obj, 0x399, 0);
        write8(obj, 0x3A0, 0);
        write16(obj, 0x396, 0);
        write8(obj, 0x394, 0);
        write32(obj, 0x3B4, 0);

        let table_index = image_word(0x018B6C94, 0);
        let style_result = callee_cdecl!(1u32, u32, table_index, 0x1Au32);
        let show_detail = if style_result == 0x7FFF_FFFF {
            1
        } else {
            u8::from(image_word(0x01160C48, style_result) != 0)
        };
        write8(obj, 0x395, show_detail);

        let mode_result = callee_cdecl!(2u32, u32, table_index, 0x1Cu32);
        let mode_slot = if mode_result == 0x7FFF_FFFF { 0x2D } else { mode_result };
        let mut mode_value = image_word(0x01160C48, mode_slot);
        write32(obj, 0x38C, mode_value);
        if (mode_value as i32) > 0 {
            mode_value = mode_value.wrapping_add(1);
            write32(obj, 0x38C, mode_value);
        }

        let query_result = callee_cdecl!(3u32, u32, table_index, 0x19u32);
        let fallback_query = if query_result == 0x7FFF_FFFF { 0x2B } else { query_result };
        let first_mode_result = callee_cdecl!(4u32, u32, table_index, 0x1Bu32);
        let query_key = if first_mode_result == 0x7FFF_FFFF {
            fallback_query
        } else {
            callee_cdecl!(5u32, u32, table_index, 0x1Bu32)
        };

        let raw_primary = callee_cdecl!(6u32, u32, table_index, 0x1Eu32);
        let primary_key = if raw_primary == 0x7FFF_FFFF { 0x2F } else { raw_primary };
        let raw_secondary = callee_cdecl!(7u32, u32, table_index, 0x1Fu32);
        let secondary_key = if raw_secondary == 0x7FFF_FFFF { 0x30 } else { raw_secondary };

        let row_slot = table_index.wrapping_mul(24);
        let row_base_address = (global::<u8>(0x019D33A0) as u32).wrapping_add(row_slot);
        let row_base = unsafe { (row_base_address as *const u32).read() };
        let row_count = unsafe { (row_base_address.wrapping_add(4) as *const u16).read() };
        let mut matched_secondary = false;

        for row_index in 0..u32::from(row_count) {
            let row = row_base.wrapping_add(row_index.wrapping_mul(0x16));
            let row_key = (read16(row, 0x12) as i16) as i32;
            let row_kind = if row_key as u32 == query_key {
                0
            } else if row_key as u32 == secondary_key {
                1
            } else if row_key as u32 == primary_key {
                2
            } else {
                continue;
            };

            let descriptor = u32::from(read8(row, 0x15));
            let mapped_index = image_word(0x01160C48, row_key as u32);
            let mut record_index = 0u32;
            while record_index < 60 {
                let key = unsafe {
                    let address = (global::<u32>(0x019D30C0) as u32)
                        .wrapping_add(record_index.wrapping_mul(12));
                    (address as *const u32).read()
                };
                if key == descriptor {
                    break;
                }
                record_index += 1;
            }

            if record_index == 60 {
                continue;
            }

            if row_kind == 0 {
                let nested_base = unsafe {
                    let address = (global::<u32>(0x019D30C4) as u32)
                        .wrapping_add(record_index.wrapping_mul(12));
                    (address as *const u32).read()
                };
                let nested_offset = mapped_index.wrapping_mul(24).wrapping_add(0x14);
                let value = read32(nested_base, nested_offset);
                write32(obj, 0x388, value);
            } else {
                if (mapped_index as i32) < 0 {
                    continue;
                }
                let limit = unsafe {
                    let address = (global::<u32>(0x019D30C8) as u32)
                        .wrapping_add(record_index.wrapping_mul(12));
                    (address as *const u16).read()
                };
                if mapped_index >= u32::from(limit) {
                    continue;
                }
                let nested_base = unsafe {
                    let address = (global::<u32>(0x019D30C4) as u32)
                        .wrapping_add(record_index.wrapping_mul(12));
                    (address as *const u32).read()
                };
                let nested_offset = mapped_index.wrapping_mul(24).wrapping_add(0x14);
                let value = read32(nested_base, nested_offset);
                write32(obj, 0x390, value);
                if row_kind == 2 {
                    matched_secondary = true;
                }
            }
        }

        let normalized_mode = callee_cdecl!(8u32, u32, read32(obj, 0x388));
        write32(obj, 0x384, normalized_mode);
        let option_present = callee_cdecl!(9u32, u32, normalized_mode);
        write8(obj, 0x394, u8::from(option_present != 0));

        if read32(obj, 0x388) == 0x22 {
            write8(obj, 0x395, 1);
            write8(obj, 0x397, 1);
            write32(obj, 0x38C, 1);
            write32(obj, 0x388, 0x22);
        }

        if matched_secondary {
            let _ = callee_thiscall!(10u32, u32, obj);
        } else {
            let _ = callee_thiscall!(11u32, u32, obj);
        }

        let item_count_raw = callee_cdecl!(12u32, u32,
            read32(obj, 0x388),
            read32(obj, 0x390),
            read32(obj, 0x38C),
            u32::from(read8(obj, 0x395))
        );
        let item_count = (item_count_raw as u8).wrapping_add(2);
        write8(obj, 0x313, item_count);
        let byte_len = u32::from(item_count).wrapping_mul(0x7C).wrapping_add(4);
        let first_array = callee_cdecl!(13u32, u32,
            byte_len,
            read32(obj, 0x388),
            read32(obj, 0x390),
            read32(obj, 0x38C),
            u32::from(read8(obj, 0x395))
        );
        let mut first_array_head = 0;
        if first_array != 0 {
            write32(first_array, 0, u32::from(item_count));
            first_array_head = first_array.wrapping_add(4);
            for item in 0..u32::from(item_count) {
                let entry = first_array_head.wrapping_add(item.wrapping_mul(0x7C));
                let _ = callee_thiscall!(14u32, u32, entry);
            }
        }

        let _ = callee_thiscall!(15u32, u32, obj,
            -2i32 as u32, 0u32, relocated(OBJ_KIND_TEXT_B), relocated(OBJ_KIND_TEXT_A)
        );
        write32(obj, 0x50, first_array_head);
        let _ = callee_thiscall!(15u32, u32, obj,
            -1i32 as u32, 1u32, relocated(OBJ_KIND_TEXT_D), relocated(OBJ_KIND_TEXT_C)
        );

        for title_row in 0..36u32 {
            let has_title = callee_cdecl!(16u32, u32,
                read32(obj, 0x388), read32(obj, 0x390), title_row,
                read32(obj, 0x38C), u32::from(read8(obj, 0x395))
            ) as u8 != 0;
            if has_title {
                let category = read32(obj, 0x388);
                let value = read32(obj, 0x390);
                let mode = read32(obj, 0x38C);
                let detail = u32::from(read8(obj, 0x395));
                let result_a = callee_cdecl!(17u32, u32, category, value, title_row, mode, detail);
                let result_b = callee_cdecl!(18u32, u32, category, value, title_row, mode, detail);
                let result_c = callee_cdecl!(19u32, u32, category, value, title_row, mode, detail);
                let _ = callee_thiscall!(15u32, u32, obj,
                    title_row, result_a, result_c, result_b
                );
            }
        }

        let second_count = u32::from(read8(obj, 0x312));
        let second_bytes = second_count.wrapping_mul(0x2B0).wrapping_add(4);
        let second_array = callee_cdecl!(20u32, u32, second_bytes);
        let mut second_array_head = 0;
        if second_array != 0 {
            write32(second_array, 0, second_count);
            second_array_head = second_array.wrapping_add(4);
            for item in 0..second_count {
                let entry = second_array_head.wrapping_add(item.wrapping_mul(0x2B0));
                let _ = callee_thiscall!(21u32, u32, entry);
            }
        }
        write32(obj, 0x54, second_array_head);

        for item in 0..second_count {
            let entry = second_array_head.wrapping_add(item.wrapping_mul(0x2B0));
            let _ = callee_thiscall!(22u32, u32, entry, u32::from(item_count));
        }

        callee_thiscall!(23u32, u32, obj)
    }
});
