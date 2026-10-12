// original: 0x00E45260 K_NICO_GT
//! Builds the online leaderboard state for the selected table row. It
//! resolves several configured identifiers, derives the display mode and
//! row selections, allocates and initializes two arrays, performs the
//! per-entry update calls, and finishes through the object's tail routine.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

const UNRESOLVED_ID: u32 = 0x7FFF_FFFF;
const ID_RESOLVE_1A: u32 = 1;
const ID_RESOLVE_1C: u32 = 2;
const ID_RESOLVE_19: u32 = 3;
const ID_RESOLVE_1B_FIRST: u32 = 4;
const ID_RESOLVE_1B_SECOND: u32 = 5;
const ID_RESOLVE_1E: u32 = 6;
const ID_RESOLVE_1F: u32 = 7;
const ID_BUTTON_CODE_SCAN: u32 = 8;
const ID_MENU_COLUMN: u32 = 9;
const ID_COUNT_ROWS: u32 = 10;
const ID_ALLOC_SMALL: u32 = 11;
const ID_INIT_SMALL: u32 = 12;
const ID_LOG_UPDATE: u32 = 13;
const ID_BUILD_RANKED: u32 = 14;
const ID_BUILD_TWIN: u32 = 15;
const ID_CHECK_ROW: u32 = 16;
const ID_SELECT_FIRST: u32 = 17;
const ID_SELECT_SECOND: u32 = 18;
const ID_SELECT_THIRD: u32 = 19;
const ID_ALLOC_LARGE: u32 = 20;
const ID_INIT_LARGE: u32 = 21;
const ID_INIT_FINAL: u32 = 22;
const ID_TAIL: u32 = 23;

#[inline(always)]
unsafe fn load_u8(base: *const u8, offset: usize) -> u8 {
    unsafe { core::ptr::read(base.add(offset)) }
}

#[inline(always)]
unsafe fn load_u16(base: *const u8, offset: usize) -> u16 {
    unsafe { core::ptr::read_unaligned(base.add(offset).cast::<u16>()) }
}

#[inline(always)]
unsafe fn load_i16(base: *const u8, offset: usize) -> i16 {
    unsafe { core::ptr::read_unaligned(base.add(offset).cast::<i16>()) }
}

#[inline(always)]
unsafe fn load_u32(base: *const u8, offset: usize) -> u32 {
    unsafe { core::ptr::read_unaligned(base.add(offset).cast::<u32>()) }
}

#[inline(always)]
unsafe fn store_u8(base: *mut u8, offset: usize, value: u8) {
    unsafe { core::ptr::write(base.add(offset), value) }
}

#[inline(always)]
unsafe fn store_u16(base: *mut u8, offset: usize, value: u16) {
    unsafe { core::ptr::write_unaligned(base.add(offset).cast::<u16>(), value) }
}

#[inline(always)]
unsafe fn store_u32(base: *mut u8, offset: usize, value: u32) {
    unsafe { core::ptr::write_unaligned(base.add(offset).cast::<u32>(), value) }
}

#[inline(always)]
unsafe fn lookup_value_table(index: u32) -> u32 {
    let table = relocated(0x01160C48) as *const u8;
    unsafe { load_u32(table, index.wrapping_mul(4) as usize) }
}

#[inline(always)]
unsafe fn descriptor_index(row_kind: u8) -> Option<u32> {
    let table = relocated(0x019D30C0) as *const u8;
    let mut index = 0u32;
    while index < 60 {
        if unsafe { load_u32(table, index.wrapping_mul(12) as usize) } == row_kind as u32 {
            return Some(index);
        }
        index += 1;
    }
    None
}

#[inline(always)]
unsafe fn descriptor_value(index: u32, item_index: u32) -> u32 {
    let table = relocated(0x019D30C0) as *const u8;
    let descriptor = unsafe { load_u32(table, index.wrapping_mul(12).wrapping_add(4) as usize) };
    let data = descriptor as *const u8;
    unsafe { load_u32(data, item_index.wrapping_mul(24).wrapping_add(0x14) as usize) }
}

#[inline(always)]
unsafe fn descriptor_count(index: u32) -> u16 {
    let table = relocated(0x019D30C0) as *const u8;
    unsafe { load_u16(table, index.wrapping_mul(12).wrapping_add(8) as usize) }
}

export!(thiscall, rw_00E45260(this_ptr: *mut u8) -> u32 {
    unsafe {
        let this = this_ptr.cast_const();
        store_u8(this_ptr, 0x313, 2);
        store_u32(this_ptr, 0x390, 0);
        store_u32(this_ptr, 0x39C, 1);
        store_u8(this_ptr, 0x399, 0);
        store_u8(this_ptr, 0x3A0, 0);
        store_u16(this_ptr, 0x396, 0);
        store_u8(this_ptr, 0x394, 0);
        store_u32(this_ptr, 0x3B4, 0);

        let query_index = load_u32(relocated(0x018B6C94) as *const u8, 0);
        let resolved_1a = callee_cdecl!(ID_RESOLVE_1A, u32, query_index, 0x1A);
        let show_entry = if resolved_1a == UNRESOLVED_ID {
            1
        } else {
            u8::from(lookup_value_table(resolved_1a) != 0)
        };
        store_u8(this_ptr, 0x395, show_entry);

        let resolved_1c = callee_cdecl!(ID_RESOLVE_1C, u32, query_index, 0x1C);
        let mode_index = if resolved_1c == UNRESOLVED_ID { 0x2D } else { resolved_1c };
        let table_mode = lookup_value_table(mode_index);
        let mode = if (table_mode as i32) > 0 {
            table_mode.wrapping_add(1)
        } else {
            table_mode
        };
        store_u32(this_ptr, 0x38C, mode);

        let resolved_19 = callee_cdecl!(ID_RESOLVE_19, u32, query_index, 0x19);
        let key_for_second_match = if resolved_19 == UNRESOLVED_ID { 0x2B } else { resolved_19 };
        let resolved_1b_first = callee_cdecl!(ID_RESOLVE_1B_FIRST, u32, query_index, 0x1B);
        let key_for_first_match = if resolved_1b_first == UNRESOLVED_ID {
            // This branch is narrowed by the contract: the original then
            // consumes a frame slot it did not initialize on this path.
            0
        } else {
            callee_cdecl!(ID_RESOLVE_1B_SECOND, u32, query_index, 0x1B)
        };
        let resolved_1e = callee_cdecl!(ID_RESOLVE_1E, u32, query_index, 0x1E);
        let _ = resolved_1e;
        let resolved_1f = callee_cdecl!(ID_RESOLVE_1F, u32, query_index, 0x1F);
        let key_for_third_match = if resolved_1f == UNRESOLVED_ID { 0x30 } else { resolved_1f };

        let row_table = relocated(0x019D33A0) as *const u8;
        let row_offset = query_index.wrapping_mul(24) as usize;
        let row_count = load_u16(row_table, row_offset + 4) as u32;
        let mut found_third_match = false;
        let mut row_number = 0u32;
        while row_number < row_count {
            let row_pointer = load_u32(row_table, row_offset) as *const u8;
            let entry_offset = row_number.wrapping_mul(0x16) as usize;
            let row_key = load_i16(row_pointer, entry_offset + 0x12) as i32 as u32;
            let row_kind = load_u8(row_pointer, entry_offset + 0x15);

            if row_key == key_for_first_match {
                let item_index = lookup_value_table(row_key);
                if let Some(desc) = descriptor_index(row_kind) {
                    store_u32(this_ptr, 0x388, descriptor_value(desc, item_index));
                }
            } else if row_key == key_for_third_match {
                let item_index = lookup_value_table(row_key) as i32;
                if item_index >= 0 {
                    if let Some(desc) = descriptor_index(row_kind) {
                        if item_index < descriptor_count(desc) as i32 {
                            found_third_match = true;
                            store_u32(this_ptr, 0x390, descriptor_value(desc, item_index as u32));
                        }
                    }
                }
            } else if row_key == key_for_second_match {
                let item_index = lookup_value_table(row_key) as i32;
                if item_index >= 0 {
                    if let Some(desc) = descriptor_index(row_kind) {
                        if item_index < descriptor_count(desc) as i32 {
                            store_u32(this_ptr, 0x390, descriptor_value(desc, item_index as u32));
                        }
                    }
                }
            }
            row_number += 1;
        }

        let selected = callee_cdecl!(ID_BUTTON_CODE_SCAN, u32, load_u32(this, 0x388));
        store_u32(this_ptr, 0x384, selected);
        let menu_column = callee_cdecl!(ID_MENU_COLUMN, u32, selected);
        store_u8(this_ptr, 0x394, u8::from(menu_column != 0));

        if load_u32(this, 0x388) == 0x22 {
            store_u8(this_ptr, 0x395, 1);
            store_u8(this_ptr, 0x397, 1);
            store_u32(this_ptr, 0x38C, 1);
            store_u32(this_ptr, 0x388, 0x22);
        }

        if found_third_match {
            let _ = callee_thiscall!(ID_BUILD_RANKED, u32, this_ptr as u32);
        } else {
            let _ = callee_thiscall!(ID_BUILD_TWIN, u32, this_ptr as u32);
        }

        let count_answer = callee_cdecl!(
            ID_COUNT_ROWS,
            u8,
            load_u32(this, 0x388),
            load_u32(this, 0x390),
            load_u32(this, 0x38C),
            load_u8(this, 0x395) as u32,
        );
        let count_small = count_answer.wrapping_add(2);
        store_u8(this_ptr, 0x313, count_small);
        let size_small = (count_small as u32).wrapping_mul(0x7C).wrapping_add(4);
        let small_array = callee_cdecl!(ID_ALLOC_SMALL, u32, size_small);
        let small_array_body = if small_array == 0 {
            0
        } else {
            store_u32(small_array as *mut u8, 0, count_small as u32);
            let mut index = 0u32;
            if count_small <= 128 {
                while index < count_small as u32 {
                    let element = small_array.wrapping_add(4).wrapping_add(index.wrapping_mul(0x7C));
                    let _ = callee_thiscall!(ID_INIT_SMALL, u32, element);
                    index += 1;
                }
            }
            small_array.wrapping_add(4)
        };
        store_u32(this_ptr, 0x50, small_array_body);

        let _ = callee_thiscall!(
            ID_LOG_UPDATE,
            u32,
            this_ptr as u32,
            0xFFFF_FFFE,
            0,
            relocated(0x00F15BBC),
            relocated(0x00F15BB0),
        );
        let _ = callee_thiscall!(
            ID_LOG_UPDATE,
            u32,
            this_ptr as u32,
            0xFFFF_FFFF,
            1,
            relocated(0x00F15BD4),
            relocated(0x00F15BC8),
        );

        let mut update_index = 0u32;
        while update_index < 0x24 {
            let flags = load_u8(this, 0x395) as u32;
            let mode = load_u32(this, 0x38C);
            let category = load_u32(this, 0x390);
            let table_mode = load_u32(this, 0x388);
            let ready = callee_cdecl!(
                ID_CHECK_ROW,
                u8,
                table_mode,
                category,
                update_index,
                mode,
                flags,
            );
            if ready != 0 {
                let first = callee_cdecl!(ID_SELECT_FIRST, u32, table_mode, category, update_index, mode, flags);
                let second = callee_cdecl!(ID_SELECT_SECOND, u32, table_mode, category, update_index, mode, flags);
                let third = callee_cdecl!(ID_SELECT_THIRD, u32, table_mode, category, update_index, mode, flags);
                let _ = callee_thiscall!(
                ID_CHECK_ROW,
                    u32,
                    this_ptr as u32,
                    update_index,
                    first,
                    second,
                    third,
                );
            }
            update_index += 1;
        }

        let count_large = load_u8(this, 0x312);
        let size_large = (count_large as u32).wrapping_mul(0x2B0).wrapping_add(4);
        let large_array = callee_cdecl!(ID_ALLOC_LARGE, u32, size_large);
        let large_array_body = if large_array == 0 {
            0
        } else {
            store_u32(large_array as *mut u8, 0, count_large as u32);
            let mut index = 0u32;
            if count_large <= 128 {
                while index < count_large as u32 {
                    let element = large_array.wrapping_add(4).wrapping_add(index.wrapping_mul(0x2B0));
                    let _ = callee_thiscall!(ID_INIT_LARGE, u32, element);
                    index += 1;
                }
            }
            large_array.wrapping_add(4)
        };
        store_u32(this_ptr, 0x54, large_array_body);

        let mut index = 0u32;
        let final_count = if count_large > 127 { 256 } else { count_large as u32 };
        while index < final_count {
            let element = large_array_body.wrapping_add(index.wrapping_mul(0x2B0));
            let _ = callee_thiscall!(ID_INIT_FINAL, u32, element, count_small as u32);
            index += 1;
        }

        callee_thiscall!(ID_TAIL, u32, this_ptr as u32)
    }
});
