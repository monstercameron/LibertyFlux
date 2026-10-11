// original: 0x00D74750 CReplayToolOverlay::vf1
//! Updates replay tool overlay state for one frame.
//!
//! The routine prepares the child and layout, derives scaled geometry, emits
//! visible rows while applying bounds and selection checks, and forwards item
//! activation or navigation updates when requested. Its early exits return
//! the child or layout result; otherwise it returns the navigation result or
//! the final position accumulator with its low byte cleared.
//!
//! Scalar floating-point operations preserve operand order and exact bits.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Scripted callees and their call shapes are declared in the contract.
const C_PREPARE: u32 = 1;
const C_LAYOUT: u32 = 2;
const C_MANAGER: u32 = 3;
const C_UPDATE: u32 = 4;
const C_X87_PRIMARY: u32 = 5;
const C_X87_SECONDARY: u32 = 6;
const C_SCALE_MODE: u32 = 7;
const C_SHOULD_REFRESH: u32 = 8;
const C_MANAGER_WITH_ARGS: u32 = 9;
const C_MANAGER_NOTIFY: u32 = 10;
const C_PROBE: u32 = 11;
const C_EMIT: u32 = 12;
const C_SELECT_ITEM: u32 = 13;
const C_ITEM_ACTIVE: u32 = 14;
const C_ITEM_UPDATE: u32 = 15;
const C_NAVIGATE: u32 = 16;

const UI_MANAGER: u32 = 0x0103_E498;
const UI_SELECTION_NAME: u32 = 0x00EE_BBD4;

const G_INITIAL_LIMIT: u32 = 0x018B_7A8C;
const G_INITIAL_SCALE: u32 = 0x017A_CCF0;
const G_SECOND_LIMIT: u32 = 0x018B_7A80;
const G_SECOND_SCALE: u32 = 0x017A_CCE8;
const G_TOP_LIMIT: u32 = 0x00FE_88E8;
const G_BASE_ADJUST: u32 = 0x00FE_86EC;
const G_ORIGIN_ADJUST: u32 = 0x00E8_5F54;
const G_COORD_ADJUST: u32 = 0x00EE_B830;
const G_ROW_ADJUST: u32 = 0x00EE_B828;
const G_ROW_STEP: u32 = 0x00EE_B82C;
const G_ACCUMULATOR_ADJUST: u32 = 0x00FE_870C;
const G_SCALE_A0: u32 = 0x0105_C884;
const G_SCALE_A1: u32 = 0x0105_C888;
const G_SCALE_B0: u32 = 0x0105_C880;
const G_SCALE_B1: u32 = 0x0105_C87C;
const G_REBUILD_FLAGS: u32 = 0x0103_775A;
const G_HOTKEYS: u32 = 0x018B_7A88;
const G_HOTKEYS_MASK: u32 = 0x018B_7A84;
const G_ACTIVE_COUNT: u32 = 0x018B_7A70;
const G_MANAGER_OBJECT: u32 = 0x0118_E868;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_byte(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_byte(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

#[inline(always)]
unsafe fn read_global_word(file_va: u32) -> u32 {
    unsafe { global::<u32>(file_va).read_unaligned() }
}

#[inline(always)]
unsafe fn read_global_byte(file_va: u32) -> u8 {
    unsafe { global::<u8>(file_va).read() }
}

#[inline(always)]
unsafe fn read_global_float(file_va: u32) -> f32 {
    f32::from_bits(unsafe { read_global_word(file_va) })
}

/// Pins scalar SSE operand order so NaN payloads follow the destination used
/// by the original sequence. Both inputs remain live through the operation.
#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    let result = core::hint::black_box(left) + core::hint::black_box(right);
    core::hint::black_box(left);
    core::hint::black_box(right);
    result
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    left - right
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    let result = core::hint::black_box(left) * core::hint::black_box(right);
    core::hint::black_box(left);
    core::hint::black_box(right);
    result
}

#[inline(always)]
unsafe fn selected_scale(use_second: u32, first_address: u32, second_address: u32) -> f32 {
    let integer_scale = if use_second != 0 {
        unsafe { read_global_word(second_address) as i32 }
    } else {
        unsafe { read_global_word(first_address) as i32 }
    };
    integer_scale as f32
}

unsafe fn run_overlay(this_ptr: u32, mutate_row_scale: bool) -> u32 {
    let child = read_word(this_ptr.wrapping_add(4));
    let _: u32 = callee_thiscall!(C_PREPARE, u32, child, 0);
    if read_byte(child.wrapping_add(0x1B)) != 0 {
        return child;
    }

    let layout_result = callee_thiscall!(C_LAYOUT, u32, this_ptr);
    if read_byte(this_ptr.wrapping_add(0x48)) == 0 {
        return layout_result;
    }

    let manager = relocated(UI_MANAGER);
    let manager_state = callee_thiscall!(C_MANAGER, u32, manager);
    write_byte(
        manager_state.wrapping_add(0x39B),
        read_byte(manager_state.wrapping_add(0x39B)) | 4,
    );

    let manager_object = read_global_word(G_MANAGER_OBJECT);
    write_word(manager_object.wrapping_add(0x44), 0x3E);

    let mut upper_bound = mul_f32(
        (read_global_word(G_INITIAL_LIMIT) as i32) as f32,
        read_global_float(G_INITIAL_SCALE),
    );
    let top_limit = read_global_float(G_TOP_LIMIT);
    if 0.0f32 > upper_bound {
        upper_bound = 0.0;
    } else if upper_bound > top_limit {
        upper_bound = top_limit;
    }

    let second_limit = mul_f32(
        (read_global_word(G_SECOND_LIMIT) as i32) as f32,
        read_global_float(G_SECOND_SCALE),
    );
    let lower_bound = if !(0.0f32 > second_limit) && second_limit > top_limit {
        top_limit
    } else {
        second_limit
    };

    write_word(this_ptr.wrapping_add(0x40), u32::MAX);
    let horizontal_seed = add_f32(
        f32::from_bits(read_word(this_ptr.wrapping_add(8))),
        read_global_float(G_BASE_ADJUST),
    );
    let vertical_seed = add_f32(
        f32::from_bits(read_word(this_ptr.wrapping_add(0x0C))),
        read_global_float(G_BASE_ADJUST),
    );
    let mut local_vertical = add_f32(vertical_seed, read_global_float(G_ORIGIN_ADJUST));

    let initial_index = if read_global_byte(G_REBUILD_FLAGS) != 0 {
        read_word(this_ptr.wrapping_add(0x100))
    } else {
        0x0B
    };

    let _: u32 = callee_thiscall!(C_UPDATE, u32, this_ptr, 0);
    let first_x87: f32 = callee_cdecl!(C_X87_PRIMARY, f32,);
    let _: u32 = callee_thiscall!(C_UPDATE, u32, this_ptr, 1);
    let second_x87: f32 = callee_cdecl!(C_X87_SECONDARY, f32,);
    let initial_span = initial_index.wrapping_sub(2) as i32 as f32;
    let menu_span = add_f32(mul_f32(second_x87, initial_span), first_x87);

    let positioned_x87: f32 = callee_cdecl!(C_X87_PRIMARY, f32,);
    let menu_vertical = add_f32(
        add_f32(positioned_x87, menu_span),
        read_global_float(G_COORD_ADJUST),
    );
    write_word(this_ptr.wrapping_add(0x14), menu_vertical.to_bits());

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_A0, G_SCALE_A1);
    let field = f32::from_bits(read_word(this_ptr.wrapping_add(8)));
    write_word(this_ptr.wrapping_add(0x1C), mul_f32(scale, field).to_bits());

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_B0, G_SCALE_B1);
    let field = f32::from_bits(read_word(this_ptr.wrapping_add(0x0C)));
    write_word(this_ptr.wrapping_add(0x20), mul_f32(scale, field).to_bits());

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_A0, G_SCALE_A1);
    let field = add_f32(
        f32::from_bits(read_word(this_ptr.wrapping_add(8))),
        f32::from_bits(read_word(this_ptr.wrapping_add(0x10))),
    );
    write_word(this_ptr.wrapping_add(0x24), mul_f32(field, scale).to_bits());

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_B0, G_SCALE_B1);
    let field = add_f32(
        f32::from_bits(read_word(this_ptr.wrapping_add(0x14))),
        f32::from_bits(read_word(this_ptr.wrapping_add(0x0C))),
    );
    write_word(this_ptr.wrapping_add(0x28), mul_f32(field, scale).to_bits());

    let should_refresh = callee_thiscall!(C_SHOULD_REFRESH, u32, this_ptr, 0, 0, 0, 0) & 0xFF;
    if should_refresh != 0 {
        let manager_state = callee_thiscall!(C_MANAGER_WITH_ARGS, u32, manager, 0, 4);
        let _: u32 = callee_thiscall!(C_MANAGER_NOTIFY, u32, manager_state);
    }

    let _: u32 = callee_thiscall!(C_UPDATE, u32, this_ptr, 0);

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_A0, G_SCALE_A1);
    write_word(this_ptr.wrapping_add(0x2C), mul_f32(scale, horizontal_seed).to_bits());

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_B0, G_SCALE_B1);
    let field = add_f32(
        f32::from_bits(read_word(this_ptr.wrapping_add(0x0C))),
        read_global_float(G_ROW_ADJUST),
    );
    write_word(this_ptr.wrapping_add(0x30), mul_f32(field, scale).to_bits());

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_A0, G_SCALE_A1);
    let field = sub_f32(
        add_f32(
            f32::from_bits(read_word(this_ptr.wrapping_add(8))),
            f32::from_bits(read_word(this_ptr.wrapping_add(0x10))),
        ),
        read_global_float(G_BASE_ADJUST),
    );
    let row_scale = mul_f32(field, scale).to_bits();
    write_word(
        this_ptr.wrapping_add(0x34),
        if mutate_row_scale { row_scale ^ 1 } else { row_scale },
    );

    let choose = callee_cdecl!(C_SCALE_MODE, u32,) & 0xFF;
    let scale = selected_scale(choose, G_SCALE_B0, G_SCALE_B1);
    let end_x87: f32 = callee_cdecl!(C_X87_PRIMARY, f32,);
    let field = add_f32(
        end_x87,
        add_f32(
            f32::from_bits(read_word(this_ptr.wrapping_add(0x0C))),
            read_global_float(G_ROW_ADJUST),
        ),
    );
    write_word(this_ptr.wrapping_add(0x38), mul_f32(field, scale).to_bits());

    let initial_horizontal_output = sub_f32(
        add_f32(horizontal_seed, read_global_float(G_BASE_ADJUST)),
        read_global_float(G_ORIGIN_ADJUST),
    );
    let initial_vertical_output = sub_f32(
        add_f32(local_vertical, read_global_float(G_BASE_ADJUST)),
        read_global_float(G_ORIGIN_ADJUST),
    );
    write_word(this_ptr.wrapping_add(0x104), initial_horizontal_output.to_bits());
    write_word(this_ptr.wrapping_add(0x108), initial_vertical_output.to_bits());

    let accumulator_adjust: f32 = callee_cdecl!(C_X87_PRIMARY, f32,);
    let accumulated_offset = sub_f32(
        add_f32(accumulator_adjust, read_global_float(G_ACCUMULATOR_ADJUST)),
        read_global_float(G_ORIGIN_ADJUST),
    );
    local_vertical = add_f32(accumulator_adjust, local_vertical);
    upper_bound = add_f32(accumulated_offset, upper_bound);
    let _: u32 = callee_thiscall!(C_UPDATE, u32, this_ptr, 1);

    let mut selection_changed = false;
    let mut navigation_direction = 1u32;
    let mut last_x87 = end_x87;
    let mut index = 1u32;
    while index < 0x0B {
        if read_global_byte(G_REBUILD_FLAGS) != 0 {
            let probe = callee_thiscall!(C_PROBE, u32, this_ptr, index) & 0xFF;
            if probe == 0 {
                index += 1;
                continue;
            }
        }

        write_word(this_ptr.wrapping_add(0x104 + index * 8), horizontal_seed.to_bits());
        write_word(this_ptr.wrapping_add(0x108 + index * 8), local_vertical.to_bits());

        let emit = callee_thiscall!(C_EMIT, u32, this_ptr, index) & 0xFF;
        if emit != 0 {
            let shifted = add_f32(horizontal_seed, read_global_float(G_ROW_STEP));
            write_word(this_ptr.wrapping_add(0x104 + index * 8), shifted.to_bits());
        }

        if !(lower_bound > horizontal_seed) {
            last_x87 = callee_cdecl!(C_X87_SECONDARY, f32,);
            local_vertical = add_f32(last_x87, local_vertical);
            index += 1;
            continue;
        }

        let width = sub_f32(
            add_f32(
                f32::from_bits(read_word(this_ptr.wrapping_add(8))),
                f32::from_bits(read_word(this_ptr.wrapping_add(0x10))),
            ),
            read_global_float(G_BASE_ADJUST),
        );
        if !(width > lower_bound) || !(upper_bound > local_vertical) {
            last_x87 = callee_cdecl!(C_X87_SECONDARY, f32,);
            local_vertical = add_f32(last_x87, local_vertical);
            index += 1;
            continue;
        }

        let extra_x87: f32 = callee_cdecl!(C_X87_SECONDARY, f32,);
        if !(add_f32(extra_x87, local_vertical) > upper_bound) {
            last_x87 = callee_cdecl!(C_X87_SECONDARY, f32,);
            local_vertical = add_f32(last_x87, local_vertical);
            index += 1;
            continue;
        }

        if read_word(this_ptr.wrapping_add(0x3C)) != index {
            if read_word(this_ptr.wrapping_add(0x44)) != index {
                let _ = callee_thiscall!(
                    C_SELECT_ITEM,
                    u32,
                    relocated(0x0117_6888),
                    relocated(UI_SELECTION_NAME)
                );
                write_word(this_ptr.wrapping_add(0x44), index);
            }
            write_word(this_ptr.wrapping_add(0x40), index);
        }

        if index == 5 {
            let child_active = read_word(this_ptr.wrapping_add(4));
            let active = callee_thiscall!(C_ITEM_ACTIVE, u32, child_active) & 0xFF;
            if active != 0 {
                let _: u32 = callee_thiscall!(C_ITEM_UPDATE, u32, child_active, 1, 1);
                let _: u32 = callee_thiscall!(C_PREPARE, u32, child_active, 1);
            }
        }

        let hotkeys = read_global_word(G_HOTKEYS);
        let hotkey_mask = read_global_word(G_HOTKEYS_MASK);
        if (hotkeys ^ hotkey_mask) & hotkey_mask & 1 != 0 {
            selection_changed = true;
            navigation_direction = 1;
            write_word(this_ptr.wrapping_add(0x3C), index);
            last_x87 = callee_cdecl!(C_X87_SECONDARY, f32,);
            local_vertical = add_f32(last_x87, local_vertical);
            index += 1;
            continue;
        }

        if read_word(this_ptr.wrapping_add(0x3C)) == index {
            let active_count = read_global_word(G_ACTIVE_COUNT) as i32;
            if active_count != 0 {
                selection_changed = true;
                navigation_direction = if active_count > 0 { 1 } else { u32::MAX };
                let manager_state = callee_thiscall!(C_MANAGER, u32, manager);
                write_byte(
                    manager_state.wrapping_add(0x39B),
                    read_byte(manager_state.wrapping_add(0x39B)) & !4,
                );
            }
        }

        last_x87 = callee_cdecl!(C_X87_SECONDARY, f32,);
        local_vertical = add_f32(last_x87, local_vertical);
        index += 1;
    }

    if selection_changed {
        callee_thiscall!(C_NAVIGATE, u32, this_ptr, navigation_direction)
    } else {
        last_x87.to_bits() & 0xFFFF_FF00
    }
}

export!(thiscall, rw_d74750(this_ptr: u32) -> u32 {
    unsafe { run_overlay(this_ptr, false) }
});

export!(thiscall, mut_d74750(this_ptr: u32) -> u32 {
    unsafe { run_overlay(this_ptr, true) }
});
