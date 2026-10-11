// original: 0x008B27F0 rage::audCompressorEffectPc::vf0
/// Initialize compressor channel state and derive its time constants.
///
/// `count` is the number of leading channel positions to inspect; the
/// positions map from the highest mask bit down. The object records that
/// count, the supplied mask byte, and the number of selected positions.
/// Two global float settings are thresholded and, when enabled, scaled into
/// double-precision helper calls whose answers become object fields. A
/// runtime rate and four object values drive the remaining reciprocal and
/// scale fields. It clears 120 state words, three following qwords are
/// cleared explicitly, and the function returns one in EAX's low byte
/// while retaining the clear operation's upper return bytes.

use lf_checker_rt::{callee_cdecl, export, global};

const COUNT_OFF: u32 = 0x294;
const SELECTED_COUNT_OFF: u32 = 0x295;
const MASK_OFF: u32 = 0x296;
const MASK_INPUT_OFF: u32 = 0x0e;
const RATE_A_OFF: u32 = 0x278;
const RATE_B_OFF: u32 = 0x27c;
const RATE_C_OFF: u32 = 0x280;
const RATE_SCALE_OFF: u32 = 0x284;
const EXP_B_OFF: u32 = 0x264;
const EXP_A_OFF: u32 = 0x268;
const RECIP_A_OFF: u32 = 0x250;
const RECIP_B_OFF: u32 = 0x254;
const RECIP_C_OFF: u32 = 0x258;
const RATE_DELTA_OFF: u32 = 0x260;
const RATE_RECIP_OFF: u32 = 0x25c;
const BUFFER_RECIP_OFF: u32 = 0x26c;
const RUNTIME_RATE_OFF: u32 = 0x80;
const CLEAR_START_OFF: u32 = 0x10;
const CLEAR_BYTES: u32 = 0x1e0;
const CLEAR_QWORDS: [u32; 3] = [0x1f0, 0x1f8, 0x200];

const CONFIG_A_VA: u32 = 0x0103_08f0;
const CONFIG_B_VA: u32 = 0x0103_08fc;
const RUNTIME_POINTER_VA: u32 = 0x0115_a448;
const RATE_SCALE_BITS: u32 = 0xc00c_cccd;
const NEGATIVE_THRESHOLD_BITS: u32 = 0xc2c8_0000;
const ONE_BITS: u32 = 0x3f80_0000;
const TEN_F64_BITS: u64 = 0x4024_0000_0000_0000;
const POW_CALLEE: u32 = 1;
const RATE_CALLEE: u32 = 2;
const BUFFER_CALLEE: u32 = 3;
const CHANNEL_SHIFTS: [u32; 8] = [7, 6, 5, 4, 3, 2, 1, 0];

#[inline(always)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
}

#[inline(always)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
}

#[inline(always)]
fn div32(left: f32, right: f32) -> f32 {
    core::hint::black_box(core::hint::black_box(left) / core::hint::black_box(right))
}

#[inline(always)]
fn split_f64(bits: u64) -> (u32, u32) {
    (bits as u32, (bits >> 32) as u32)
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_u64(address: u32, value: u64) {
    unsafe { (address as *mut u64).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    unsafe { f32::from_bits(global::<u32>(file_va).read_unaligned()) }
}

export!(thiscall, rw_8b27f0(this: u32, count: u32, state: u32) -> u32 {
    let mask = unsafe { read_u8(state.wrapping_add(MASK_INPUT_OFF)) };
    unsafe {
        write_u8(this.wrapping_add(COUNT_OFF), count as u8);
        write_u8(this.wrapping_add(MASK_OFF), mask);
        write_u8(this.wrapping_add(SELECTED_COUNT_OFF), 0);
    }

    let mut selected = 0u8;
    let mut index = 0u32;
    while index < count {
        let shift = CHANNEL_SHIFTS.get(index as usize).copied().unwrap_or(0);
        if (mask as u32) & (1u32 << shift) != 0 {
            selected = selected.wrapping_add(1);
        }
        index = index.wrapping_add(1);
    }
    unsafe { write_u8(this.wrapping_add(SELECTED_COUNT_OFF), selected) };

    let threshold = f32::from_bits(NEGATIVE_THRESHOLD_BITS);
    let one = f32::from_bits(ONE_BITS);
    let rate_scale = f32::from_bits(RATE_SCALE_BITS);
    let config_a = unsafe { read_global_f32(CONFIG_A_VA) };
    let config_a_delta = sub32(config_a, threshold);
    let exp_a = if config_a_delta >= 0.0 {
        let scaled = mul32(config_a, f32::from_bits(0x3d4c_cccd));
        let (ten_low, ten_high) = split_f64(TEN_F64_BITS);
        let (scaled_low, scaled_high) = split_f64((scaled as f64).to_bits());
        let answer: u64 = callee_cdecl!(
            POW_CALLEE, u64, ten_low, ten_high, scaled_low, scaled_high,
        );
        core::hint::black_box(f64::from_bits(answer)) as f32
    } else {
        0.0
    };
    unsafe { write_u32(this.wrapping_add(EXP_A_OFF), exp_a.to_bits()) };

    let config_b = unsafe { read_global_f32(CONFIG_B_VA) };
    let config_b_delta = sub32(config_b, threshold);
    let exp_b = if config_b_delta >= 0.0 {
        let scaled = mul32(config_b, f32::from_bits(0x3d4c_cccd));
        let (ten_low, ten_high) = split_f64(TEN_F64_BITS);
        let (scaled_low, scaled_high) = split_f64((scaled as f64).to_bits());
        let answer: u64 = callee_cdecl!(
            POW_CALLEE, u64, ten_low, ten_high, scaled_low, scaled_high,
        );
        core::hint::black_box(f64::from_bits(answer)) as f32
    } else {
        0.0
    };
    unsafe { write_u32(this.wrapping_add(EXP_B_OFF), exp_b.to_bits()) };

    let runtime_object = unsafe { global::<u32>(RUNTIME_POINTER_VA).read_unaligned() };
    let raw_rate = unsafe { read_u32(runtime_object.wrapping_add(RUNTIME_RATE_OFF)) };
    let rate = core::hint::black_box(raw_rate as f64) as f32;

    let recip_input_a = div32(one, rate);
    let recip_scaled_a = mul32(recip_input_a, rate_scale);
    let recip_arg_a = div32(recip_scaled_a, unsafe { f32::from_bits(read_u32(this.wrapping_add(RATE_B_OFF))) });
    let answer_a_bits: u32 = callee_cdecl!(RATE_CALLEE, u32, recip_arg_a.to_bits());
    let recip_a = sub32(one, f32::from_bits(answer_a_bits));
    unsafe { write_u32(this.wrapping_add(RECIP_A_OFF), recip_a.to_bits()) };

    let recip_input_b = div32(one, rate);
    let recip_scaled_b = mul32(recip_input_b, rate_scale);
    let recip_arg_b = div32(recip_scaled_b, unsafe { f32::from_bits(read_u32(this.wrapping_add(RATE_C_OFF))) });
    let answer_b_bits: u32 = callee_cdecl!(RATE_CALLEE, u32, recip_arg_b.to_bits());
    let recip_b = sub32(one, f32::from_bits(answer_b_bits));
    unsafe { write_u32(this.wrapping_add(RECIP_B_OFF), recip_b.to_bits()) };

    let reciprocal_c = div32(one, unsafe { f32::from_bits(read_u32(this.wrapping_add(RATE_A_OFF))) });
    let rate_c = sub32(one, reciprocal_c);
    unsafe { write_u32(this.wrapping_add(RECIP_C_OFF), rate_c.to_bits()) };

    let scaled_rate = mul32(rate, unsafe { f32::from_bits(read_u32(this.wrapping_add(RATE_SCALE_OFF))) });
    let rate_delta = sub32(scaled_rate, one);
    unsafe { write_u32(this.wrapping_add(RATE_DELTA_OFF), rate_delta.to_bits()) };
    let rate_recip = div32(one, scaled_rate);
    unsafe { write_u32(this.wrapping_add(RATE_RECIP_OFF), rate_recip.to_bits()) };

    let buffer_rate_bits: u32 = callee_cdecl!(
        BUFFER_CALLEE, u32, exp_b.to_bits(), rate_c.to_bits(),
    );
    let buffer_recip = div32(one, f32::from_bits(buffer_rate_bits));
    unsafe { write_u32(this.wrapping_add(BUFFER_RECIP_OFF), buffer_recip.to_bits()) };

    let clear_pointer = this.wrapping_add(CLEAR_START_OFF);
    unsafe {
        core::ptr::write_bytes(clear_pointer as *mut u8, 0, CLEAR_BYTES as usize);
        for offset in CLEAR_QWORDS {
            write_u64(this.wrapping_add(offset), 0);
        }
    }
    (clear_pointer & 0xffff_ff00) | 1
});
