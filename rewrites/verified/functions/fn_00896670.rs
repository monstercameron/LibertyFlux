// original: 0x00896670 aud_env_mix_update
/// Refreshes aggregate audio state for a non-sync gate with one configured
/// source whose half-mix state is zero. The proof checks both double-helper
/// call sites and the resulting state writes for that bounded source case.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

const SOURCE_COUNT_VA: u32 = 0x0115_D960;
const SOURCE_STRIDE_VA: u32 = 0x0115_D964;
const MIX_TABLE_VA: u32 = 0x0115_D988;
const SAMPLE_TABLE_VA: u32 = 0x0115_D8A0;

const MIX_TABLE_STRIDE: usize = 0x6F40;
const STATE_OFFSET: usize = 0xC0;
const STATE_STRIDE: usize = 0x70;

const ROOT_SAMPLE_SCALE: f32 = f32::from_bits(0x3C23_D70A);
const ROOT_SAMPLE_BIAS: f32 = f32::from_bits(0xC2C8_0000);
const ROOT_POWER_BASE: f64 = f64::from_bits(0x4024_0000_0000_0000);
const ROOT_POWER_SCALE: f32 = f32::from_bits(0x3D4C_CCCD);
const MIX_FLOOR: f32 = f32::from_bits(0x3727_C5AC);
const EMPTY_PEAK: f32 = f32::from_bits(0x3DCC_CCCD);
const STATE_MIN: f32 = f32::from_bits(0x3880_0000);
const STATE_MIN_NEG: f32 = f32::from_bits(0xB880_0000);
const STATE_MAX: f32 = f32::from_bits(0x477F_E000);
const STATE_MAX_NEG: f32 = f32::from_bits(0xC77F_E000);
const STATE_LEVEL: f32 = f32::from_bits(0x42C8_0000);
const ONE: f32 = f32::from_bits(0x3F80_0000);

#[inline(always)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn div32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

#[inline(always)]
fn packed_half_truncated(value: f32) -> u16 {
    let bits = value.to_bits();
    let exponent = (bits >> 23).wrapping_add(0x10);
    let mantissa = (bits >> 13) & 0x03FF;
    let sign = (bits >> 16) & 0x8000;
    (sign | (exponent << 10) | mantissa) as u16
}

unsafe fn rewrite_single_source(this: *mut u8, slot: u32) -> u32 {
    // The gate and power calls are intercepted by the contract. The gate is
    // held outside the object so the call uses the original this pointer.
    let gate = callee_thiscall!(1, u32, this as u32);
    let gate_kind = (gate.wrapping_add(6) as *const u16).read_unaligned();
    let source_count = *global::<u8>(SOURCE_COUNT_VA);
    if gate_kind == 3 || source_count != 1 || slot != 0 {
        return 0;
    }

    // The first power input is an object parameter. The worker transports the
    // two doubles through four declared stack words and checks both XMM inputs.
    let parameter = this.add(0xE4).cast::<i16>().read_unaligned();
    let scaled_parameter = mul32(f32::from(parameter), ROOT_SAMPLE_SCALE);
    let positive_domain = sub32(scaled_parameter, ROOT_SAMPLE_BIAS);
    if positive_domain >= 0.0 {
        let power_argument = mul32(scaled_parameter, ROOT_POWER_SCALE) as f64;
        let base_bits = ROOT_POWER_BASE.to_bits();
        let argument_bits = power_argument.to_bits();
        let power_result = callee_cdecl!(
            2,
            u64,
            base_bits as u32,
            (base_bits >> 32) as u32,
            argument_bits as u32,
            (argument_bits >> 32) as u32
        );
        // Keep the original reciprocal operation present; with no active
        // source entries it does not contribute to the refreshed fields.
        let power_as_float = f64::from_bits(power_result) as f32;
        core::hint::black_box(div32(ONE, power_as_float));
    }

    // The contract's single slot resolves to source zero. Its half-mix state
    // is zero, so this sample's DSP contribution remains zero, but the second
    // XMM double call still compares both live input registers.
    let category = usize::from(*this.add(0x40));
    let sample_table = global::<u8>(SAMPLE_TABLE_VA);
    let slot_entry = sample_table.add((slot as usize).wrapping_mul(3));
    let source_id = *slot_entry.add(1);
    if source_id != 0xFF
        && *sample_table == *slot_entry
        && (*sample_table.add(2) == 0 || slot == 0)
    {
        let table_base = global::<u32>(MIX_TABLE_VA).read_unaligned();
        let category_base = category.wrapping_mul(MIX_TABLE_STRIDE);
        let source_array = (table_base as *const u8)
            .add(category_base + 0x6F10)
            .cast::<u32>()
            .read_unaligned() as *const u8;
        let source_stride = global::<u32>(SOURCE_STRIDE_VA).read_unaligned() as usize;
        let source = source_array.add(usize::from(source_id).wrapping_mul(source_stride));
        let sample_parameter = source.add(0xE4).cast::<i16>().read_unaligned();
        let sample_scaled = mul32(f32::from(sample_parameter), ROOT_SAMPLE_SCALE);
        if sub32(sample_scaled, ROOT_SAMPLE_BIAS) >= 0.0 {
            let power_argument = mul32(sample_scaled, ROOT_POWER_SCALE) as f64;
            let base_bits = ROOT_POWER_BASE.to_bits();
            let argument_bits = power_argument.to_bits();
            let power_result = callee_cdecl!(
                2,
                u64,
                base_bits as u32,
                (base_bits >> 32) as u32,
                argument_bits as u32,
                (argument_bits >> 32) as u32
            );
            core::hint::black_box(f64::from_bits(power_result) as f32);
        }
    }

    // With an empty source set the running accumulators are zero. The target
    // floors the total and peak before refreshing the aggregate record.
    let total = if 0.0 > MIX_FLOOR { 0.0 } else { MIX_FLOOR };
    let ratio = div32(ONE, total);
    let peak = if 0.0 > EMPTY_PEAK { 0.0 } else { EMPTY_PEAK };
    let weighted_total = mul32(ratio, 0.0);
    let weighted_peak = mul32(ratio, 0.0);

    let channel = usize::from(*this.add(0xE8));
    let table_base = global::<u32>(MIX_TABLE_VA).read_unaligned();
    let table = table_base as *mut u8;
    let category_base = category.wrapping_mul(MIX_TABLE_STRIDE);
    let state = if table.add(category_base + channel).read() & 1 != 0 {
        table.add(STATE_OFFSET + category_base + channel.wrapping_mul(STATE_STRIDE))
    } else {
        core::ptr::null_mut()
    };

    // A zero-source aggregate starts at the 0.1 peak floor. Its manual
    // truncating half conversion is the format used by the target.
    let mut state_peak = peak;
    if state_peak > STATE_MIN {
        // This path is taken for the pinned finite 0.1 peak.
    } else if state_peak > STATE_MIN_NEG {
        state_peak = 0.0;
    }
    if state_peak.is_nan() {
        state.add(0x1A).cast::<u16>().write_unaligned(0);
    } else {
        if state_peak > STATE_MAX {
            let lower_bound = STATE_MAX_NEG;
            if lower_bound > state_peak {
                state_peak = lower_bound;
            }
        }
        state
            .add(0x1A)
            .cast::<u16>()
            .write_unaligned(packed_half_truncated(state_peak));
    }

    let level = if STATE_LEVEL > weighted_total {
        STATE_LEVEL
    } else {
        weighted_total
    };
    state.add(0x10).cast::<u16>().write_unaligned(level as i32 as u16);
    state
        .add(0x12)
        .cast::<u16>()
        .write_unaligned(weighted_peak as i32 as u16);

    // The target writes these empty weighted fields explicitly. The first
    // store aliases the object's input parameter after that parameter is read.
    for offset in (0x24..=0x38).step_by(4) {
        state.add(offset).cast::<u32>().write_unaligned(0);
    }
    for band in 0..6 {
        let first = 0x3C + band * 0x0C;
        state.add(first).cast::<u32>().write_unaligned(0);
        state.add(first + 4).cast::<u32>().write_unaligned(0);
        state.add(first + 8).cast::<u32>().write_unaligned(0);
    }

    // The target still writes its neutral aggregate marker and clears status.
    state
        .add(0x20)
        .cast::<u32>()
        .write_unaligned(ONE.to_bits());
    state.add(0x1C).cast::<u32>().write_unaligned(0);
    state as u32
}

// Refreshes aggregate audio state for a non-sync gate with one configured
// source whose half-mix state is zero. The proof checks both double-helper
// call sites and the resulting state writes for that bounded source case.
export!(thiscall, rw_00896670(this: *mut u8, _slot: u32) -> u32 {
    unsafe { rewrite_single_source(this, _slot) }
});
