// original: 0x00A21440 ped_task_move_blend
/// Updates the task's blend value from its stored rate and a manager-provided
/// float record. It returns early for manager states that disable blending.
/// Otherwise it runs two interpolation kernels, clamps the blend to the
/// unit interval, records the latest positive time delta, and either clears
/// the transient timer state or writes the sampled blend and its normalized
/// progress to the shared task state.
const TASK_BLEND_OFFSET: u32 = 0x74;
const TASK_MANAGER_OFFSET: u32 = 0x384;
const TASK_TARGET_OFFSET: u32 = 0x31c;
const TASK_CURRENT_OFFSET: u32 = 0x320;
const TASK_BLEND_OUTPUT_OFFSET: u32 = 0x330;
const TASK_TIME_OFFSET: u32 = 0x2dc;
const TASK_DELTA_OFFSET: u32 = 0x2e0;
const TASK_STAMP_OFFSET: u32 = 0x364;
const MANAGER_DISABLED_FLAG_OFFSET: u32 = 0x210;
const MANAGER_CENTER_OFFSET: u32 = 0x214;
const MANAGER_DISABLED_STATE_OFFSET: u32 = 0xa70;
const MANAGER_VTABLE_OFFSET: u32 = 0;
const VTABLE_BLEND_SLOT_OFFSET: u32 = 0xec;
const RECORD_VALUE_OFFSET: u32 = 8;
const FRAME_VALUE_WORD: usize = 2;

const FIRST_KERNEL_A: u32 = 0x0103_bffc;
const FIRST_KERNEL_B: u32 = 0x0103_c000;
const FIRST_KERNEL_C: u32 = 0x0103_c004;
const FIRST_KERNEL_D: u32 = 0x0103_c008;
const SECOND_KERNEL_A: u32 = 0x0103_c020;
const SECOND_KERNEL_B: u32 = 0x0103_c024;
const SECOND_KERNEL_C: u32 = 0x0103_c028;
const SECOND_KERNEL_D: u32 = 0x0103_c02c;
const THIRD_KERNEL_A: u32 = 0x0103_c010;
const THIRD_KERNEL_B: u32 = 0x0103_c014;
const THIRD_KERNEL_C: u32 = 0x0103_c018;
const THIRD_KERNEL_D: u32 = 0x0103_c01c;
const BLEND_LIMIT: u32 = 0x0103_c048;
const TIMER_INTERVAL: u32 = 0x0103_c00c;
const TIMER_VALUE: u32 = 0x0117_35c4;
const UNSIGNED_DOUBLE_TABLE: u32 = 0x00fe_8f50;
const ABS_MASK: u32 = 0x00fe_8f80;

const GLOBAL_BLEND_SAMPLE: u32 = 0x12dd_2a0;
const GLOBAL_BLEND_STATE: u32 = 0x12dd_298;
const GLOBAL_BLEND_PROGRESS: u32 = 0x12dd_29c;

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_u32(file_address: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(file_address).read_unaligned() }
}

#[inline(always)]
unsafe fn read_global_f32(file_address: u32) -> f32 {
    f32::from_bits(unsafe { read_global_u32(file_address) })
}

#[inline(always)]
unsafe fn read_global_f64(file_address: u32) -> f64 {
    f64::from_bits(unsafe {
        lf_checker_rt::global::<u64>(file_address).read_unaligned()
    })
}

#[inline(always)]
unsafe fn write_global_u8(file_address: u32, value: u8) {
    unsafe { lf_checker_rt::global::<u8>(file_address).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_global_u32(file_address: u32, value: u32) {
    unsafe { lf_checker_rt::global::<u32>(file_address).write_unaligned(value) }
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn div_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

#[inline(always)]
fn add_f64(left: f64, right: f64) -> f64 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn clamp_seed(value: f32, maximum: f32) -> f32 {
    let upper = if maximum > value { value } else { maximum };
    let zero = 0.0f32;
    let lower = if zero <= upper { upper } else { zero };
    if lower <= maximum { lower } else { maximum }
}

#[inline(always)]
fn clamp_rate_update(value: f32, previous: f32, maximum: f32) -> f32 {
    if !(value >= 0.0) || !(previous >= 0.0) {
        return previous;
    }
    clamp_seed(add_f32(value, previous), maximum)
}

#[inline(always)]
unsafe fn unsigned_integer_as_f32(value: u32) -> f32 {
    let signed = (value as i32) as f64;
    let correction_address = if value >> 31 == 0 {
        UNSIGNED_DOUBLE_TABLE
    } else {
        UNSIGNED_DOUBLE_TABLE + 8
    };
    let correction = unsafe { read_global_f64(correction_address) };
    add_f64(signed, correction) as f32
}

#[inline(always)]
unsafe fn run_blend_kernel(
    callee: u32,
    value: f32,
    constants: [u32; 4],
) -> f32 {
    lf_checker_rt::callee_cdecl!(
        callee,
        f32,
        value.to_bits(),
        constants[0],
        constants[1],
        constants[2],
        constants[3],
    )
}

lf_checker_rt::export!(thiscall, rw_00A21440(this: u32) -> u32 {
    unsafe {
        let manager = read_u32(this + TASK_MANAGER_OFFSET);
        if manager != 0
            && (read_u8(manager + MANAGER_DISABLED_FLAG_OFFSET) != 0
                || read_u32(manager + MANAGER_DISABLED_STATE_OFFSET) == 1)
        {
            return 0;
        }

        let current_rate = f32::from_bits(read_u32(this + TASK_BLEND_OFFSET));
        let target = f32::from_bits(read_u32(this + TASK_TARGET_OFFSET));
        let current = f32::from_bits(read_u32(this + TASK_CURRENT_OFFSET));
        let difference = sub_f32(target, current);
        let absolute_mask = read_global_u32(ABS_MASK);
        let absolute_difference = f32::from_bits(difference.to_bits() & absolute_mask);
        let first_result = run_blend_kernel(
            1,
            absolute_difference,
            [
                read_global_u32(FIRST_KERNEL_A),
                read_global_u32(FIRST_KERNEL_B),
                read_global_u32(FIRST_KERNEL_C),
                read_global_u32(FIRST_KERNEL_D),
            ],
        );
        let maximum = read_global_f32(BLEND_LIMIT);
        let first_rate = clamp_seed(add_f32(first_result, current_rate), maximum);
        write_u32(this + TASK_BLEND_OFFSET, first_rate.to_bits());

        let mut frame_values = [0u32; 8];
        let vtable = read_u32(manager + MANAGER_VTABLE_OFFSET);
        let vslot = read_u32(vtable + VTABLE_BLEND_SLOT_OFFSET);
        let get_blend: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(vslot as usize);
        let record = get_blend(manager, frame_values.as_mut_ptr() as u32);
        let record_value = f32::from_bits(read_u32(record + RECORD_VALUE_OFFSET));
        let record_magnitude = f32::from_bits(record_value.to_bits() & absolute_mask);
        let second_result = run_blend_kernel(
            2,
            record_magnitude,
            [
                read_global_u32(SECOND_KERNEL_A),
                read_global_u32(SECOND_KERNEL_B),
                read_global_u32(SECOND_KERNEL_C),
                read_global_u32(SECOND_KERNEL_D),
            ],
        );
        let previous_second_rate = f32::from_bits(read_u32(this + TASK_BLEND_OUTPUT_OFFSET));
        let updated_rate = clamp_rate_update(second_result, previous_second_rate, maximum);
        if second_result >= 0.0 && previous_second_rate >= 0.0 {
            write_u32(this + TASK_BLEND_OFFSET, updated_rate.to_bits());
        }
        write_u32(this + TASK_BLEND_OUTPUT_OFFSET, second_result.to_bits());

        let manager_center = f32::from_bits(read_u32(manager + MANAGER_CENTER_OFFSET));
        let previous_time = f32::from_bits(read_u32(this + TASK_TIME_OFFSET));
        let time_delta = sub_f32(previous_time, manager_center);
        let timer = read_global_u32(TIMER_VALUE);
        let previous_delta = f32::from_bits(read_u32(this + TASK_DELTA_OFFSET));
        if time_delta > 0.0 && time_delta > previous_delta {
            write_u32(this + TASK_STAMP_OFFSET, timer);
            write_u32(this + TASK_DELTA_OFFSET, time_delta.to_bits());
        }

        let elapsed = timer.wrapping_sub(read_u32(this + TASK_STAMP_OFFSET));
        let interval = read_global_u32(TIMER_INTERVAL);
        let active_delta = f32::from_bits(read_u32(this + TASK_DELTA_OFFSET));
        if elapsed >= interval || !(active_delta > 0.0) {
            write_global_u8(GLOBAL_BLEND_STATE, 0);
            write_u32(this + TASK_DELTA_OFFSET, 0);
            write_u32(this + TASK_TIME_OFFSET, manager_center.to_bits());
            return 0;
        }

        let elapsed_as_f32 = unsigned_integer_as_f32(elapsed);
        let interval_as_f32 = unsigned_integer_as_f32(interval);
        let time_ratio = div_f32(elapsed_as_f32, interval_as_f32);
        let sampled_blend = lf_checker_rt::callee_cdecl!(4, f32, time_ratio.to_bits());
        write_global_u32(GLOBAL_BLEND_SAMPLE, sampled_blend.to_bits());

        let final_result = run_blend_kernel(
            5,
            active_delta,
            [
                read_global_u32(THIRD_KERNEL_A),
                read_global_u32(THIRD_KERNEL_B),
                read_global_u32(THIRD_KERNEL_C),
                read_global_u32(THIRD_KERNEL_D),
            ],
        );
        let prior_frame_value = f32::from_bits(frame_values[FRAME_VALUE_WORD]);
        let final_rate = clamp_seed(add_f32(final_result, prior_frame_value), maximum);
        write_u32(this + TASK_BLEND_OFFSET, final_rate.to_bits());

        let progress_span = sub_f32(
            read_global_f32(THIRD_KERNEL_D),
            read_global_f32(THIRD_KERNEL_C),
        );
        let progress = div_f32(final_result, progress_span);
        write_global_u8(GLOBAL_BLEND_STATE, 1);
        write_global_u32(GLOBAL_BLEND_PROGRESS, progress.to_bits());
        write_u32(this + TASK_TIME_OFFSET, manager_center.to_bits());
        0
    }
});
