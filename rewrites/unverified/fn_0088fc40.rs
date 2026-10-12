// original: 0x0088FC40 audio_compute_voice_mix
/// Recomputes a voice's gain, timing, duration, minimum, and maximum values.
///
/// The two voice selectors use separate indexed tables. The first selector
/// supplies the initial gains and duration bounds; the second selects the
/// current voice profile. Sentinel selectors preserve the same defaults as
/// the original. Optional float pointers contribute to the gain and timing
/// accumulators, with the original truncating conversion rules.
use lf_checker_rt::{export, relocated};

const ENTITY_ROW_STRIDE: u32 = 0x6F40;
const PRIMARY_TABLE_FIELD: u32 = 0x6F10;
const SECONDARY_TABLE_FIELD: u32 = 0x6F14;
const ENTITY_BASE_VA: u32 = 0x0115D988;
const PRIMARY_STRIDE_VA: u32 = 0x0115D964;
const SECONDARY_STRIDE_VA: u32 = 0x0115D968;
const GROUP_TABLE_VA: u32 = 0x0115F80C;
const SCALE_100_VA: u32 = 0x00FE8BB0;
const DEFAULT_DURATION: u32 = 0x5D5C;

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn read_u16(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_i16(address: u32) -> i16 {
    unsafe { (address as *const i16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    unsafe { (address as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u16(address: u32, value: u16) {
    unsafe { (address as *mut u16).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_f32(address: u32, value: f32) {
    unsafe { (address as *mut f32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn descriptor_for(
    this_address: u32,
    selector: u8,
    table_field: u32,
    stride_va: u32,
) -> u32 {
    let base = unsafe { read_u32(relocated(ENTITY_BASE_VA)) };
    let row_index = unsafe { read_u8(this_address.wrapping_add(0x40)) } as u32;
    let row = base.wrapping_add(row_index.wrapping_mul(ENTITY_ROW_STRIDE));
    let entries = unsafe { read_u32(row.wrapping_add(table_field)) };
    let stride = unsafe { read_u32(relocated(stride_va)) };
    entries.wrapping_add(stride.wrapping_mul(u32::from(selector)))
}

#[inline(always)]
unsafe fn group_index(descriptor: u32) -> u8 {
    unsafe { read_u8(descriptor.wrapping_add(0xE7)) & 7 }
}

#[inline(always)]
unsafe fn table_offset(group: u8) -> u32 {
    let table_entry = GROUP_TABLE_VA.wrapping_add(u32::from(group).wrapping_mul(8));
    unsafe { read_u32(relocated(table_entry)) }
}

#[inline(always)]
fn ordered_add(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn ordered_mul(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

/// Reproduce the x86 truncating single-to-signed-word conversion.
#[inline(always)]
fn cvttss2si(value: f32) -> i32 {
    if !value.is_finite()
        || value >= 2_147_483_648.0_f32
        || value < -2_147_483_648.0_f32
    {
        i32::MIN
    } else {
        // The checks above bound the value to the signed 32-bit interval;
        // this cast therefore truncates toward zero like CVTTSS2SI.
        value as i32
    }
}

/// Return the low word from a truncating x87 conversion to a signed 64-bit
/// integer; the x87 indefinite result has a zero low word.
#[inline(always)]
fn fistp_i64_low_u32(value: f32) -> u32 {
    let widened = value as f64;
    const I64_LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if !value.is_finite() || widened >= I64_LIMIT || widened < -I64_LIMIT {
        0
    } else {
        // The checks above bound the value to the signed 64-bit interval;
        // the cast truncates toward zero, matching the x87 control word.
        (widened as i64) as u32
    }
}

#[inline(always)]
fn combine_sentinel(accumulator: &mut i32, value: i32) {
    if *accumulator == -1 {
        *accumulator = value;
    } else if value != -1 {
        *accumulator = accumulator.wrapping_add(value);
    }
}

export!(thiscall, rw_0088fc40(this_ptr: *mut u8) -> () {
    let this_address = this_ptr as u32;
    unsafe {
        let primary_selector = read_u8(this_address.wrapping_add(5));
        let (initial_gain, initial_secondary, duration_limit, secondary_limit, mut peak) =
            if primary_selector == 0xFF {
                (0.0_f32, 0.0_f32, DEFAULT_DURATION, 0_i32, 0_u16)
            } else {
                let descriptor = descriptor_for(
                    this_address,
                    primary_selector,
                    PRIMARY_TABLE_FIELD,
                    PRIMARY_STRIDE_VA,
                );
                // The original reads this signed field before replacing the
                // register with the unsigned duration field below.
                let _ = core::hint::black_box(read_i16(descriptor.wrapping_add(0x34)));
                (
                    read_f32(descriptor.wrapping_add(0x28)),
                    read_f32(descriptor.wrapping_add(0x2C)),
                    u32::from(read_u16(descriptor.wrapping_add(0x30))),
                    i32::from(read_i16(descriptor.wrapping_add(0x36))),
                    read_u16(descriptor.wrapping_add(0x32)),
                )
            };

        let flags = read_u8(this_address.wrapping_add(8));
        let selector = read_u8(this_address.wrapping_add(4));

        let mut gain = read_f32(this_address.wrapping_add(0x24));
        if flags & 1 == 0 {
            gain = ordered_add(gain, initial_gain);
        }

        let mut current_descriptor = 0_u32;
        if selector != 0xFF {
            current_descriptor = descriptor_for(
                this_address,
                selector,
                SECONDARY_TABLE_FIELD,
                SECONDARY_STRIDE_VA,
            );
            let group = group_index(current_descriptor);
            let preset = table_offset(group).wrapping_add(2).wrapping_shl(5);
            gain = ordered_add(
                gain,
                read_f32(current_descriptor.wrapping_add(preset)),
            );
        }

        let optional_gain = read_u32(this_address.wrapping_add(0x0C));
        if optional_gain != 0 {
            gain = ordered_add(gain, read_f32(optional_gain));
        }
        gain = ordered_add(gain, read_f32(this_address.wrapping_add(0x1C)));
        write_f32(this_address.wrapping_add(0x28), gain);

        let mut secondary = initial_secondary;
        if selector != 0xFF {
            let group = group_index(current_descriptor);
            let preset = table_offset(group).wrapping_shl(5);
            secondary = ordered_add(
                secondary,
                read_f32(current_descriptor.wrapping_add(preset).wrapping_add(0x44)),
            );
        }
        write_f32(this_address.wrapping_add(0x2C), secondary);

        let table_angle = if selector == 0xFF {
            -1_i32
        } else {
            let group = group_index(current_descriptor);
            let preset = table_offset(group).wrapping_shl(5);
            i32::from(read_i16(
                current_descriptor
                    .wrapping_add(preset)
                    .wrapping_add(0x52),
            ))
        };

        let mut combined = if flags & 4 == 0 {
            secondary_limit
        } else {
            -1_i32
        };
        let optional_integer = read_u32(this_address.wrapping_add(0x14));
        if optional_integer != 0 {
            combine_sentinel(
                &mut combined,
                cvttss2si(read_f32(optional_integer)),
            );
        }
        combine_sentinel(
            &mut combined,
            i32::from(read_i16(this_address.wrapping_add(0x0A))),
        );
        combine_sentinel(&mut combined, table_angle);
        combine_sentinel(
            &mut combined,
            i32::from(read_i16(this_address.wrapping_add(0x7C))),
        );
        if combined != -1 {
            combined = combined % 360;
        }
        write_u16(this_address.wrapping_add(0x34), combined as u16);

        let mut timing = i32::from(read_i16(this_address.wrapping_add(0x22)));
        if flags & 2 == 0 {
            timing = timing.wrapping_add(secondary_limit);
        }
        if selector != 0xFF {
            let group = group_index(current_descriptor);
            let preset = table_offset(group).wrapping_shl(5);
            timing = timing.wrapping_add(i32::from(read_i16(
                current_descriptor
                    .wrapping_add(preset)
                    .wrapping_add(0x50),
            )));
        }
        let optional_scale = read_u32(this_address.wrapping_add(0x10));
        if optional_scale != 0 {
            let scale = read_f32(relocated(SCALE_100_VA));
            timing = timing.wrapping_add(cvttss2si(ordered_mul(
                read_f32(optional_scale),
                scale,
            )));
        }
        let updated_timing = read_u16(this_address.wrapping_add(0x20))
            .wrapping_add(timing as u16);
        write_u16(this_address.wrapping_add(0x36), updated_timing);

        let mut duration = duration_limit;
        if selector != 0xFF {
            let group = group_index(current_descriptor);
            let preset = table_offset(group).wrapping_shl(5);
            duration = duration.min(u32::from(read_u16(
                current_descriptor
                    .wrapping_add(preset)
                    .wrapping_add(0x58),
            )));
        }
        let optional_duration = read_u32(this_address.wrapping_add(0x18));
        if optional_duration != 0 {
            duration = duration.min(fistp_i64_low_u32(read_f32(optional_duration)));
        }
        write_u16(
            this_address.wrapping_add(0x30),
            duration.min(duration_limit) as u16,
        );

        if selector != 0xFF {
            let group = group_index(current_descriptor);
            let preset = table_offset(group).wrapping_shl(5);
            peak = peak.max(read_u16(
                current_descriptor
                    .wrapping_add(preset)
                    .wrapping_add(0x5A),
            ));
        }
        write_u16(this_address.wrapping_add(0x32), peak);
    }
});
