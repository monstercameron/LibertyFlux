// original: 0x00895A00 audio_voice_render
//! Render one active audio voice into its selected parameter slot.
//!
//! The active path resolves a voice row, gathers its input block, updates the
//! slot's rendered fields, and returns whether the voice remains active.

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global};

const VOICE_STRIDE: u32 = 0x0115_D968;
const VOICE_TABLE: u32 = 0x0115_D988;
const INDEX_TABLE: u32 = 0x0115_F80C;
const PARAM_TABLE: u32 = 0x0115_F814;

#[inline(always)]
unsafe fn r8(base: *const u8, offset: usize) -> u8 {
    unsafe { base.add(offset).read_volatile() }
}

#[inline(always)]
unsafe fn r16(base: *const u8, offset: usize) -> u16 {
    unsafe { base.add(offset).cast::<u16>().read_unaligned() }
}

#[inline(always)]
unsafe fn r32(base: *const u8, offset: usize) -> u32 {
    unsafe { base.add(offset).cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn w16(base: *mut u8, offset: usize, value: u16) {
    unsafe { base.add(offset).cast::<u16>().write_unaligned(value) }
}

#[inline(always)]
unsafe fn w32(base: *mut u8, offset: usize, value: u32) {
    unsafe { base.add(offset).cast::<u32>().write_unaligned(value) }
}

#[inline(always)]
unsafe fn rf32(base: *const u8, offset: usize) -> f32 {
    f32::from_bits(unsafe { r32(base, offset) })
}

#[inline(always)]
unsafe fn gf32(file_va: u32) -> f32 {
    f32::from_bits(unsafe { global::<u32>(file_va).read_unaligned() })
}

#[inline(always)]
fn double_words(bits: u64) -> (u32, u32) {
    (bits as u32, (bits >> 32) as u32)
}

export!(thiscall, rw_00895a00(this: *mut u8, _arg: u32) -> u32 {
    unsafe {
        if r16(this, 6) != 2 {
            return 0;
        }

        let voice_base = global::<u32>(VOICE_TABLE).read_unaligned();
        let stride = global::<u32>(VOICE_STRIDE).read_unaligned();
        let voice = r8(this, 0x40) as u32;
        let slot = r8(this, 0xe8) as u32;
        let bank = voice_base.wrapping_add(voice.wrapping_mul(0x6f40));
        let row_anchor = r32(bank as *const u8, 0x6f14);
        let row_a = row_anchor.wrapping_add((r8(this, 0xeb) as u32).wrapping_mul(stride));
        let row_b = row_anchor.wrapping_add((r8(this, 0xea) as u32).wrapping_mul(stride));
        let selected = if r8(row_a as *const u8, 0xe7) & 0x10 != 0 { row_a } else { row_b };
        let mut scratch = [0u8; 0x180];

        let _slot_measure = callee_thiscall!(1, u32, row_a);
        let _ = callee_thiscall!(2, u32, scratch.as_mut_ptr().add(0x9c) as u32);

        let index = (r8(row_a as *const u8, 0xe7) & 7) as usize;
        let index_offset = global::<u32>(INDEX_TABLE + (index as u32) * 8).read_unaligned();
        let row_values = row_a as *const u8;
        let _position = [
            rf32(row_values, (index_offset as usize + 1) * 16),
            rf32(row_values, (index_offset as usize + 1) * 16 + 4),
            rf32(row_values, (index_offset as usize + 1) * 16 + 8),
        ];

        let _ = callee_stdcall!(3, u32, scratch.as_mut_ptr().add(0x9c) as u32);

        let (a0_lo, a0_hi) = double_words(global::<u64>(0x00fe_8a28).read_unaligned());
        let (a1_lo, a1_hi) = double_words(0);
        let d0 = callee_cdecl!(4, u64, 0x100, 0xff, 3, 0, a0_lo, a0_hi, a1_lo, a1_hi);
        let first_result = f64::from_bits(d0) as f32;

        let (b0_lo, b0_hi) = double_words(global::<u64>(0x00fe_8a78).read_unaligned());
        let param_index = r16(this, 0x3e) as i16;
        let param_table = global::<u32>(PARAM_TABLE).read_unaligned() as *const u8;
        let indexed_speed = if param_index != -1 {
            rf32(param_table, param_index as usize * 0x24)
        } else {
            0.0
        };
        let object_speed = rf32(this, 0x2c);
        let speed = core::hint::black_box(indexed_speed) + core::hint::black_box(object_speed);
        let speed_scaled = core::hint::black_box(speed) * core::hint::black_box(gf32(0x00fe_876c));
        let speed_double = (speed_scaled as f64).to_bits();
        let (b1_lo, b1_hi) = double_words(speed_double);
        let second_result = callee_cdecl!(4, u64, 0x100, 0xff, 3, 0, b0_lo, b0_hi, b1_lo, b1_hi);

        let interpolation = core::hint::black_box(rf32(row_a as *const u8, 0xc8))
            * core::hint::black_box(gf32(0x00fe_86b4));
        let interpolation_bits = interpolation.to_bits();
        let _ = callee_thiscall!(
            5,
            u32,
            (this as u32).wrapping_add(0xb0),
            interpolation_bits,
            interpolation_bits
        );
        let measurement_input = core::hint::black_box(first_result)
            * core::hint::black_box(rf32(this, 0xdc));
        let _measurement = callee_thiscall!(
            6,
            f32,
            (this as u32).wrapping_add(0xb0),
            measurement_input.to_bits()
        );

        let local_start = rf32(this, 0x28)
            - core::hint::black_box(r16(this, 0xe4) as i16 as f32)
                * core::hint::black_box(gf32(0x00fe_870c));
        let phase = local_start - gf32(0x00fe_8a24);
        let phase_scaled = core::hint::black_box(phase) * core::hint::black_box(gf32(0x00fe_876c));
        let phase_double = (phase_scaled as f64).to_bits();
        let (c0_lo, c0_hi) = double_words(global::<u64>(0x00fe_8a78).read_unaligned());
        let (c1_lo, c1_hi) = double_words(phase_double);
        let _third_result = callee_cdecl!(4, u64, 0x100, 0xff, 3, 0, c0_lo, c0_hi, c1_lo, c1_hi);

        let _gate = callee_thiscall!(7, u8, this as u32, this as u32);

        let table = global::<u32>(VOICE_TABLE).read_unaligned();
        let slot_flags = (table.wrapping_add(voice.wrapping_mul(0x6f40)).wrapping_add(slot)) as *const u8;
        if r8(slot_flags, 0) & 1 != 0 {
            let record = (table
                .wrapping_add(0xc0)
                .wrapping_add(voice.wrapping_mul(0x6f40))
                .wrapping_add(slot.wrapping_mul(0x70))) as *mut u8;
            w32(record, 0, _measurement.to_bits());

            let param_offset = (param_index.max(0) as usize) * 0x24;
            let table_min = r16(param_table, param_offset + 0x1e);
            let table_max = r16(param_table, param_offset + 0x20);
            let object_min = r16(this, 0x30);
            let object_max = r16(this, 0x32);
            let low = table_min.min(object_min);
            let high = table_max.max(object_max);
            let stored_low = if low < 100 { 100 } else { low };
            w16(record, 0x10, stored_low);
            w16(record, 0x12, high);

            const HALF_ZERO_EDGE: f32 = f32::from_bits(0x3880_0000);
            let raw_half_value = f64::from_bits(second_result) as f32;
            let half_value = if raw_half_value.is_nan()
                || (raw_half_value > -HALF_ZERO_EDGE && raw_half_value < HALF_ZERO_EDGE)
            {
                0.0
            } else if raw_half_value > 65504.0 {
                65504.0
            } else if raw_half_value < -65504.0 {
                -65504.0
            } else {
                raw_half_value
            };
            let half_bits = half_value.to_bits();
            let half = (((half_bits >> 23).wrapping_add(0x10) << 10)
                | ((half_bits >> 13) & 0x3ff)
                | ((half_bits >> 16) & 0x8000)) as u16;
            w32(record, 0x18, ((half as u32) << 16) | 0x10);

            let scalar_scale = if phase < -100.0 {
                0.0
            } else {
                let value = f64::from_bits(_third_result) as f32;
                if value < 0.0 {
                    0.0
                } else if value > 1.0 {
                    1.0
                } else {
                    value
                }
            };
            let indexed_values = row_a as *const u8;
            let vector_offset = index_offset as usize * 0x20;
            let channel_a = rf32(indexed_values, vector_offset + 0x48);
            let channel_b = rf32(indexed_values, vector_offset + 0x4c);
            w32(
                record,
                0x1c,
                (core::hint::black_box(channel_a) * core::hint::black_box(scalar_scale)).to_bits(),
            );
            w32(
                record,
                0x20,
                (core::hint::black_box(channel_b) * core::hint::black_box(scalar_scale)).to_bits(),
            );

            let transform_state = scratch.as_ptr().add(0x9c);
            let mut lane = 0usize;
            while lane < 6 {
                w32(
                    record,
                    0x24 + lane * 4,
                    r32(transform_state, 0x18 + lane * 4),
                );
                lane += 1;
            }
            for (destination, source) in [(0x3c, 0x30), (0x48, 0x3c), (0x54, 0x48), (0x60, 0x54)] {
                lane = 0;
                while lane < 3 {
                    w32(
                        record,
                        destination + lane * 4,
                        r32(transform_state, source + lane * 4),
                    );
                    lane += 1;
                }
            }
        }
        let _ = selected;
        let _ = gf32(PARAM_TABLE);
        1
    }
});
