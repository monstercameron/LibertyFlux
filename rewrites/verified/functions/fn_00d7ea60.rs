// original: 0x00D7EA60 pool_entry_updater
//! Updates eligible pool entries after comparing the input object's direction
//! vectors with each entry's position and orientation. It scans the pool from
//! the last entry to the first, honors the state and indexed-table gates,
//! records one of three event pointers when the scripted result object accepts
//! the transition, and sets the entry's 0x400 flag unless its existing flags
//! already contain the protected 0x0c00 pattern. Float operations retain the
//! original scalar order and comparisons, including unordered-value behavior.

//! Proven scope: a fabricated pool of zero to four entries with a fixed stride,
//! selected entry flags, result states and indexed flags, and scripted virtual
//! calls returning fixed vector objects. The input position is zero; direction
//! and entry vectors use selected finite values. Other counts, layouts, indices,
//! arbitrary float values and native callee behavior remain untested. Outgoing
//! frame pointer arguments for virtual calls 0 and 1 are skipped without snapshots;
//! their pointee effects are not observed. All other declared comparisons pass.

use core::hint::black_box;
use lf_checker_rt::{callee_cdecl, export, global};

#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    black_box(black_box(a) * black_box(b))
}

#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    black_box(black_box(a) + black_box(b))
}

#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    black_box(black_box(a) - black_box(b))
}

#[inline(always)]
fn div(a: f32, b: f32) -> f32 {
    black_box(black_box(a) / black_box(b))
}

#[inline(always)]
fn sqrt(a: f32) -> f32 {
    black_box(black_box(a).sqrt())
}

#[inline(always)]
unsafe fn rd_u32(base: u32, off: u32) -> u32 {
    unsafe { (base.wrapping_add(off) as *const u32).read() }
}

#[inline(always)]
unsafe fn rd_i16(base: u32, off: u32) -> i16 {
    unsafe { (base.wrapping_add(off) as *const i16).read() }
}

#[inline(always)]
unsafe fn rd_u8(base: u32, off: u32) -> u8 {
    unsafe { (base.wrapping_add(off) as *const u8).read() }
}

#[inline(always)]
unsafe fn rd_f32(base: u32, off: u32) -> f32 {
    unsafe { (base.wrapping_add(off) as *const f32).read() }
}

#[inline(always)]
unsafe fn wr_u32(base: u32, off: u32, value: u32) {
    unsafe { (base.wrapping_add(off) as *mut u32).write(value) }
}

#[inline(always)]
unsafe fn wr_u8(base: u32, off: u32, value: u8) {
    unsafe { (base.wrapping_add(off) as *mut u8).write(value) }
}

#[inline(always)]
unsafe fn vcall_1(object: u32, slot: u32, out_frame: u32) -> u32 {
    unsafe {
        let table = rd_u32(object, 0);
        let target = rd_u32(table, slot);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object, out_frame)
    }
}

#[inline(always)]
unsafe fn entry_pair_value(entry: u32, slot: u32, frame: &mut [u32; 4]) -> u32 {
    unsafe { vcall_1(entry, slot, frame.as_mut_ptr() as u32) }
}

unsafe fn update_pool(input: u32, flag_to_set: u32) {
    unsafe {
        // The first four virtual calls form the initial squared-length value;
        // the last two provide the normalized input direction.
        let mut frame = [0u32; 4];
        let first = vcall_1(input, 0xec, frame.as_mut_ptr() as u32);
        let second = vcall_1(input, 0xec, frame.as_mut_ptr() as u32);
        let first_product = mul(rd_f32(second, 4), rd_f32(first, 4));
        let third = vcall_1(input, 0xec, frame.as_mut_ptr() as u32);
        let fourth = vcall_1(input, 0xec, frame.as_mut_ptr() as u32);
        let second_product = mul(rd_f32(fourth, 0), rd_f32(third, 4));
        let squared_length = add(second_product, first_product);
        let length = sqrt(squared_length);
        if 0.1f32 > length {
            return;
        }

        let fifth = vcall_1(input, 0xec, frame.as_mut_ptr() as u32);
        let inverse_length = div(1.0f32, length);
        let input_unit_x = mul(inverse_length, rd_f32(fifth, 0));
        let sixth = vcall_1(input, 0xec, frame.as_mut_ptr() as u32);
        let input_unit_y = mul(rd_f32(sixth, 4), inverse_length);

        let pool = global::<u32>(0x12e22a4).read();
        let mut remaining = rd_u32(pool, 8);
        if remaining == 0 {
            return;
        }
        let base = rd_u32(pool, 0);
        let active_flags = rd_u32(pool, 4);
        let stride = rd_u32(pool, 0x0c) as i32;
        let input_matrix = rd_u32(input, 0x20);
        let input_x = rd_f32(input_matrix, 0x30);
        let input_y = rd_f32(input_matrix, 0x34);
        let input_z = rd_f32(input_matrix, 0x38);
        let event_table = global::<u32>(0x11735b4).read();

        while remaining != 0 {
            remaining = remaining.wrapping_sub(1);
            if rd_u8(active_flags.wrapping_add(remaining), 0) & 0x80 != 0 {
                continue;
            }
            let entry = base.wrapping_add(stride.wrapping_mul(remaining as i32) as u32);
            if entry == 0 {
                continue;
            }

            let entry_state = rd_u32(entry, 0x1300);
            if entry_state != 0 && entry_state != 1 {
                continue;
            }
            if rd_u8(entry, 0xf1f) & 0x40 != 0 {
                continue;
            }
            if rd_u8(entry, 0xf1d) & 0x10 != 0 {
                continue;
            }
            if rd_u8(entry, 0x10b8) != 1 {
                continue;
            }
            let linked = rd_u32(entry, 0xf50);
            if linked != 0 && rd_u8(linked, 0xa60) != 1 {
                continue;
            }
            if entry == input {
                continue;
            }
            if rd_u8(entry, 0xf14) & 3 != 0 {
                continue;
            }
            if rd_u8(entry, 0xf1a) & 0x20 != 0 {
                continue;
            }
            let index = rd_i16(entry, 0x2e) as i32;
            let index_table = global::<u32>(0x1295cd8) as u32;
            let indexed = rd_u32(index_table, (index as u32).wrapping_mul(4));
            let index_flags = rd_u32(indexed, 0x94) >> 1;
            if index_flags & 1 != 0 {
                continue;
            }

            let entry_matrix = rd_u32(entry, 0x20);
            let mut z_delta = sub(input_z, rd_f32(entry_matrix, 0x38));
            if 0.0f32 > z_delta {
                z_delta = f32::from_bits(z_delta.to_bits() ^ 0x8000_0000);
            }
            if !(4.0f32 > z_delta) {
                continue;
            }

            let delta_y = sub(rd_f32(entry_matrix, 0x34), input_y);
            let delta_x = sub(rd_f32(entry_matrix, 0x30), input_x);
            let dy_squared = mul(delta_y, delta_y);
            let dx_squared = mul(delta_x, delta_x);
            let horizontal_length = sqrt(add(dy_squared, dx_squared));
            let distance_limit = add(length, 40.0f32);
            if !(distance_limit > horizontal_length) {
                continue;
            }

            let entry_first = entry_pair_value(entry, 0xec, &mut frame);
            let entry_second = entry_pair_value(entry, 0xec, &mut frame);
            let initial_x = rd_f32(entry_first, 0);
            let initial_y = rd_f32(entry_second, 4);
            let initial_y_sq = mul(initial_y, initial_y);
            let initial_x_sq = mul(initial_x, initial_x);
            let gate_length = sqrt(add(initial_y_sq, initial_x_sq));
            if !(gate_length > 0.05f32) {
                continue;
            }

            let result = callee_cdecl!(2, u32, entry);
            if result == 0 {
                continue;
            }

            let dot_y = mul(delta_y, input_unit_y);
            let dot_x = mul(delta_x, input_unit_x);
            let direction_dot = div(add(dot_y, dot_x), horizontal_length);

            if !(direction_dot > 0.8f32) {
                let first_value = entry_pair_value(entry, 0xec, &mut frame);
                let temp = mul(rd_f32(first_value, 4), delta_y);
                let second_value = entry_pair_value(entry, 0xec, &mut frame);
                let projection = add(mul(rd_f32(second_value, 0), temp), horizontal_length);
                if 0.0f32 > projection {
                    let result_state = rd_u8(result, 0x2a);
                    if result_state != 1 && result_state != 0x18 {
                        wr_u8(result, 0x2a, 1);
                        wr_u32(result, 0x10, event_table.wrapping_add(0x7d0));
                    }
                }
                continue;
            }

            let orient_y = mul(rd_f32(entry_matrix, 0x14), input_unit_y);
            let orient_x = mul(rd_f32(entry_matrix, 0x10), input_unit_x);
            let orientation_dot = add(orient_y, orient_x);
            if orientation_dot > 0.7f32 || -0.9f32 > orientation_dot {
                let mut result_state = rd_u8(result, 0x2a);
                if result_state != 0x14 && result_state != 0x15 {
                    let cross_y = mul(delta_x, input_unit_y);
                    let cross_x = mul(delta_y, input_unit_x);
                    let cross = sub(cross_y, cross_x);
                    result_state = if !(cross > 0.0f32) { 0x15 } else { 0x14 };
                    if direction_dot < 0.0f32 {
                        result_state = if result_state == 0x14 { 0x15 } else { 0x14 };
                    }
                    if result_state == 0x15 {
                        result_state = 1;
                    }
                    wr_u8(result, 0x2a, result_state);
                    wr_u32(result, 0x10, event_table.wrapping_add(0x9c4));
                }

                let entry_flags = rd_u32(entry, 0x28);
                if entry_flags & 0x7c00 != 0x0c00 {
                    let updated = (entry_flags & 0xffff_87ff) | flag_to_set;
                    wr_u32(entry, 0x28, updated);
                }
                continue;
            }

            let first_value = entry_pair_value(entry, 0xec, &mut frame);
            let temp = mul(rd_f32(first_value, 4), delta_y);
            let second_value = entry_pair_value(entry, 0xec, &mut frame);
            let projection = add(mul(rd_f32(second_value, 0), temp), horizontal_length);
            if 0.0f32 > projection {
                let result_state = rd_u8(result, 0x2a);
                if result_state != 1 && result_state != 0x18 {
                    wr_u8(result, 0x2a, 1);
                    wr_u32(result, 0x10, event_table.wrapping_add(0xfa0));
                }
            }
        }
    }
}

export!(cdecl, rw_00d7ea60(input: u32) -> () {
    unsafe { update_pool(input, 0x400) }
});

/// The negative control differs only in the pool-entry flag selected by the
/// state-update path; the same contract must detect that observable heap word.
export!(cdecl, mut_00d7ea60(input: u32) -> () {
    unsafe { update_pool(input, 0x800) }
});