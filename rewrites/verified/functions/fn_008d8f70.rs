// original: 0x008D8F70 FADE_LOAD
//! Implements the active-object service-return path: it reads the configured
//! numeric parameters, records the later tick value, passes a local color
//! word by pointer, and returns the final service result. The proof covers
//! only the nonzero-active path with scripted helper results. Other load and
//! scheduling branches and switch-table cases remain unproven; this rewrite
//! currently returns zero outside the tested path.

#![allow(unsafe_code)]

use lf_checker_rt::global;

const GLOBAL_ELAPSED_SECONDS: u32 = 0x117_37A0;
const GLOBAL_PREVIOUS_TICK_LOW: u32 = 0x117_5768;
const GLOBAL_PREVIOUS_TICK_HIGH: u32 = 0x117_576C;
const GLOBAL_TICK_SCALE: u32 = 0x17A_CD18;
const GLOBAL_CONFIG: u32 = 0x17F_5838;
const GLOBAL_PARAMETER: u32 = 0x106_B310;
const OBJECT_ACTIVE_FLAG: u32 = 0xFCC;
const CONFIG_FIRST_COUNT: u32 = 0x2B4;
const CONFIG_FIRST_SCALE: u32 = 0x28C;
const CONFIG_SECOND_COUNT: u32 = 0x2B0;
const CONFIG_SECOND_SCALE: u32 = 0x288;
const UINT32_FLOAT_ONE: u32 = 0x3F80_0000;
const INITIAL_COLOR: u32 = 0xFF00_0000;

#[inline(always)]
unsafe fn read_global_word(file_va: u32) -> u32 {
    global::<u32>(file_va).read()
}

#[inline(always)]
unsafe fn write_global_word(file_va: u32, value: u32) {
    global::<u32>(file_va).write(value);
}

#[inline(always)]
unsafe fn read_heap_word(base: u32, offset: u32) -> u32 {
    (base.wrapping_add(offset) as *const u32).read()
}

#[inline(always)]
fn cvttss2si(value: f32) -> i32 {
    if !value.is_finite() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}

/// Advances the shared elapsed clock and submits the current streaming
/// configuration when the active object follows the direct service-return
/// path. The proof deliberately covers only that path; other load and
/// scheduling states remain outside this narrow rewrite.
fn fade_load_service_return(this_object: u32, wrong_tick_update: bool) -> u32 {
    let active = unsafe {
        (this_object.wrapping_add(OBJECT_ACTIVE_FLAG) as *const u8).read()
    };
    if active == 0 {
        return 0;
    }

    let current_tick_before = lf_checker_rt::callee_cdecl!(0, u64,);
    let previous_low = unsafe { read_global_word(GLOBAL_PREVIOUS_TICK_LOW) } as u64;
    let previous_high = unsafe { read_global_word(GLOBAL_PREVIOUS_TICK_HIGH) } as u64;
    let previous_tick = (previous_high << 32) | previous_low;
    let elapsed_ticks = current_tick_before.wrapping_sub(previous_tick) as i64;
    let elapsed_as_float = elapsed_ticks as f32;
    let tick_scale =
        f32::from_bits(unsafe { read_global_word(GLOBAL_TICK_SCALE) });
    let scaled_elapsed = core::hint::black_box(elapsed_as_float)
        * core::hint::black_box(tick_scale);
    let elapsed_before =
        f32::from_bits(unsafe { read_global_word(GLOBAL_ELAPSED_SECONDS) });
    let elapsed_after =
        core::hint::black_box(elapsed_before) + core::hint::black_box(scaled_elapsed);
    unsafe { write_global_word(GLOBAL_ELAPSED_SECONDS, elapsed_after.to_bits()) };

    let current_tick_after = lf_checker_rt::callee_cdecl!(0, u64,);
    let previous_tick_low_after = current_tick_after as u32;
    let stored_tick_low = if wrong_tick_update {
        previous_tick_low_after.wrapping_add(1)
    } else {
        previous_tick_low_after
    };
    unsafe {
        write_global_word(GLOBAL_PREVIOUS_TICK_LOW, stored_tick_low);
        write_global_word(GLOBAL_PREVIOUS_TICK_HIGH, (current_tick_after >> 32) as u32);
    }

    let _ = lf_checker_rt::callee_cdecl!(1, u32,);

    let config = unsafe { read_global_word(GLOBAL_CONFIG) };
    let first_count = unsafe { read_heap_word(config, CONFIG_FIRST_COUNT) } as i32 as f32;
    let first_scale = f32::from_bits(unsafe { read_heap_word(config, CONFIG_FIRST_SCALE) });
    let first_scaled = core::hint::black_box(first_count) * core::hint::black_box(first_scale);
    let first_units = cvttss2si(first_scaled) as u32;

    let second_count = unsafe { read_heap_word(config, CONFIG_SECOND_COUNT) } as i32 as f32;
    let second_scale = f32::from_bits(unsafe { read_heap_word(config, CONFIG_SECOND_SCALE) });
    let second_scaled =
        core::hint::black_box(second_count) * core::hint::black_box(second_scale);
    let second_units = cvttss2si(second_scaled) as u32;

    let parameter = unsafe { read_global_word(GLOBAL_PARAMETER) };
    let service_parameter = if (parameter as i32) < 0 { parameter } else { 0 };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, config, 1);
    let _ = lf_checker_rt::callee_cdecl!(3, u32,);
    let _ = lf_checker_rt::callee_cdecl!(4, u32,);
    let _ = lf_checker_rt::callee_cdecl!(5, u32, 10, 0);
    let _ = lf_checker_rt::callee_cdecl!(6, u32, UINT32_FLOAT_ONE);
    let _ = lf_checker_rt::callee_cdecl!(5, u32, 1, service_parameter);
    let _ = lf_checker_rt::callee_cdecl!(7, u32,);

    let mut color_slot = INITIAL_COLOR;
    let color_slot_ptr = (&mut color_slot as *mut u32) as u32;
    let _ = lf_checker_rt::callee_cdecl!(
        8,
        u32,
        0,
        0,
        second_units,
        first_units,
        0,
        color_slot_ptr
    );

    lf_checker_rt::callee_cdecl!(9, u32,)
}

pub fn fade_load_early_service_path(this_object: u32) -> u32 {
    fade_load_service_return(this_object, false)
}

pub fn fade_load_wrong_clock_mutant(this_object: u32) -> u32 {
    fade_load_service_return(this_object, true)
}

lf_checker_rt::export!(thiscall, rw_008D8F70(this_object: u32) -> u32 {
    fade_load_early_service_path(this_object)
});

lf_checker_rt::export!(thiscall, mut_008D8F70(this_object: u32) -> u32 {
    fade_load_wrong_clock_mutant(this_object)
});
