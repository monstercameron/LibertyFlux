// original: 0x0088FC40 audio_compute_voice_mix
//! Recomputes the mix and integer limits stored in the voice state. The two
//! slot bytes select entity records through row tables and global strides.
//! The function accumulates selected floating values, combines signed angle
//! terms with -1 as the unset marker, wraps the angle by 360, and stores
//! unsigned minimum and maximum limits.
use lf_checker_rt::{export, relocated};

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
unsafe fn load_f32(base: *const u8, offset: usize) -> f32 {
    unsafe { core::ptr::read_unaligned(base.add(offset).cast::<f32>()) }
}

#[inline(always)]
unsafe fn store_u16(base: *mut u8, offset: usize, value: u16) {
    unsafe { core::ptr::write_unaligned(base.add(offset).cast::<u16>(), value) }
}

#[inline(always)]
unsafe fn entity_for_slot(
    row_table: *const u8,
    row_selector: u8,
    row_field: u32,
    slot: u8,
    stride_global: u32,
) -> *const u8 {
    let row_offset = (row_selector as u32)
        .wrapping_mul(0x6F40)
        .wrapping_add(row_field);
    let row = unsafe { row_table.add(row_offset as usize) };
    let entities = unsafe { load_u32(row, 0) };
    let stride = unsafe { load_u32(relocated(stride_global) as *const u8, 0) };
    entities
        .wrapping_add((slot as u32).wrapping_mul(stride))
        as *const u8
}

#[inline(always)]
fn merge_unset(sum: i32, value: i32) -> i32 {
    if sum == -1 {
        value
    } else if value == -1 {
        sum
    } else {
        sum.wrapping_add(value)
    }
}

#[inline(always)]
fn cvttss2si(value: f32) -> i32 {
    // The source conversion returns the indefinite integer for NaN and for
    // values outside [-2147483648, 2147483648). The cast is safe only after
    // those instruction-specific bounds have been checked.
    if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
        i32::MIN
    } else {
        value as i32
    }
}

#[inline(always)]
fn fistp_qword_low_u32(value: f32) -> u32 {
    // The source sets x87 rounding to truncate, stores a signed qword, then
    // reads its low dword. Invalid conversions store i64::MIN, whose low
    // dword is zero. The cast below is bounded to the qword conversion range.
    let wide = value as f64;
    if value.is_nan() || wide >= 9_223_372_036_854_775_808.0 || wide < -9_223_372_036_854_775_808.0 {
        0
    } else {
        (value as i64) as u32
    }
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

export!(thiscall, rw_0088FC40(this_ptr: *mut u8) -> u32 {
    unsafe {
        let this = this_ptr.cast_const();
        let slot_a = load_u8(this, 5);
        let row_selector = load_u8(this, 0x40);
        let flags = load_u8(this, 8);
        let row_table = load_u32(relocated(0x0115D988) as *const u8, 0) as *const u8;

        let entity_a = if slot_a == 0xFF {
            None
        } else {
            Some(entity_for_slot(
                row_table,
                row_selector,
                0x6F10,
                slot_a,
                0x0115D964,
            ))
        };

        let (mix_a, mut mix_b, min_seed, max_seed, local_34, local_36) =
            if let Some(entity) = entity_a {
                (
                    load_f32(entity, 0x28),
                    load_f32(entity, 0x2C),
                    load_u16(entity, 0x30) as u32,
                    load_u16(entity, 0x32) as u32,
                    load_i16(entity, 0x34) as i32,
                    load_i16(entity, 0x36) as i32,
                )
            } else {
                (0.0, 0.0, 0x5D5C, 0, -1, 0)
            };

        let mut angle_total = if flags & 4 == 0 { local_34 } else { -1 };
        let mut first_mix = load_f32(this, 0x24);
        if flags & 1 == 0 {
            first_mix = add_f32(first_mix, mix_a);
        }

        let slot_b = load_u8(this, 4);
        let entity_b = if slot_b == 0xFF {
            None
        } else {
            Some(entity_for_slot(
                row_table,
                row_selector,
                0x6F14,
                slot_b,
                0x0115D968,
            ))
        };

        let metadata_base = if let Some(entity) = entity_b {
            let kind = (load_u8(entity, 0xE7) & 7) as usize;
            let table = relocated(0x0115F80C) as *const u8;
            load_u32(table, kind * 8).wrapping_shl(5)
        } else {
            0
        };

        if let Some(entity) = entity_b {
            let table_mix = load_f32(entity, metadata_base.wrapping_add(0x40) as usize);
            first_mix = add_f32(table_mix, first_mix);
            mix_b = add_f32(
                mix_b,
                load_f32(entity, metadata_base.wrapping_add(0x44) as usize),
            );
        }

        let pointer_c = load_u32(this, 0x0C);
        if pointer_c != 0 {
            first_mix = add_f32(first_mix, load_f32(pointer_c as *const u8, 0));
        }
        let final_mix = add_f32(load_f32(this, 0x1C), first_mix);
        core::ptr::write_unaligned(this_ptr.add(0x28).cast::<f32>(), final_mix);
        core::ptr::write_unaligned(this_ptr.add(0x2C).cast::<f32>(), mix_b);

        let pointer_14 = load_u32(this, 0x14);
        let angle_from_float = if pointer_14 == 0 {
            -1
        } else {
            cvttss2si(load_f32(pointer_14 as *const u8, 0))
        };
        angle_total = merge_unset(angle_total, angle_from_float);
        angle_total = merge_unset(angle_total, load_i16(this, 0x0A) as i32);
        if let Some(entity) = entity_b {
            angle_total = merge_unset(
                angle_total,
                load_i16(entity, metadata_base.wrapping_add(0x52) as usize) as i32,
            );
        }
        angle_total = merge_unset(angle_total, load_i16(this, 0x7C) as i32);
        if angle_total != -1 {
            angle_total %= 360;
        }
        store_u16(this_ptr, 0x34, angle_total as u16);

        let mut angle_b = load_i16(this, 0x22) as i32;
        if flags & 2 == 0 {
            angle_b = angle_b.wrapping_add(local_36);
        }
        if let Some(entity) = entity_b {
            angle_b = angle_b.wrapping_add(
                load_i16(entity, metadata_base.wrapping_add(0x50) as usize) as i32,
            );
        }
        let pointer_10 = load_u32(this, 0x10);
        if pointer_10 != 0 {
            let scale = load_f32(relocated(0x00FE8BB0) as *const u8, 0);
            let converted = cvttss2si(mul_f32(load_f32(pointer_10 as *const u8, 0), scale));
            angle_b = angle_b.wrapping_add(converted);
        }
        let angle_out = load_u16(this, 0x20).wrapping_add(angle_b as u16);
        store_u16(this_ptr, 0x36, angle_out);

        let mut min_candidate = if entity_b.is_some() { min_seed } else { 0x5D5C };
        if let Some(entity) = entity_b {
            min_candidate = min_candidate.min(
                load_u16(entity, metadata_base.wrapping_add(0x58) as usize) as u32,
            );
        }
        let pointer_18 = load_u32(this, 0x18);
        if pointer_18 != 0 {
            min_candidate = min_candidate.min(fistp_qword_low_u32(load_f32(pointer_18 as *const u8, 0)));
        }
        let min_value = min_seed.min(min_candidate);
        store_u16(this_ptr, 0x30, min_value as u16);

        let mut max_value = max_seed;
        if let Some(entity) = entity_b {
            max_value = max_value.max(
                load_u16(entity, metadata_base.wrapping_add(0x5A) as usize) as u32,
            );
        }
        store_u16(this_ptr, 0x32, max_value as u16);

        let return_value = if let Some(entity) = entity_b {
            load_u16(entity, metadata_base.wrapping_add(0x5A) as usize) as u32
        } else {
            ((min_candidate as u16 as u32) & 0xFFFF_FF00) | 0xFF
        };
        return_value
    }
});
