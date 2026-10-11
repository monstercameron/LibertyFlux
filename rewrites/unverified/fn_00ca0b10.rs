// original: 0x00CA0B10 ik_two_bone_solver_angles
/// Replays the three sentinel exits and the selector-zero call sequence for
/// the two-bone solve. This proof pins vector inputs and scripted helper
/// answers to zero, compares the three double calls and the low float lane,
/// and checks the four-word state transfer at the end.
///
/// The general nonzero vector arithmetic and selector values outside 0..=3
/// are outside this deliberately narrow proof.

const BONE_TABLE_VA: u32 = 0x1295_CD8;
const BONE_METHOD_OFFSET: u32 = 0x38;
const RESULT_WORDS_OFFSET: u32 = 0x30;
const RESULT_WORD_COUNT: usize = 4;
const ZERO_F64_BITS: u64 = 0;
const SIGNED_ZERO_F32_BITS: u32 = 0x8000_0000;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_i16(address: u32) -> i16 {
    unsafe { (address as *const i16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_global_u32(file_address: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(file_address).read_unaligned() }
}

#[inline(always)]
unsafe fn bone_method(task: u32, argument: u32) -> u32 {
    let signed_index = unsafe { read_i16(task + 0x2E) } as i32;
    let table_address = BONE_TABLE_VA.wrapping_add(signed_index.wrapping_mul(4) as u32);
    let bone = unsafe { read_global_u32(table_address) };
    let vtable = unsafe { read_u32(bone) };
    let method = unsafe { read_u32(vtable + BONE_METHOD_OFFSET) };
    let call: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(method as usize) };
    call(bone, argument)
}

#[inline(always)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn call_double_helper() -> u64 {
    lf_checker_rt::callee_cdecl!(8, u64, 0u32, 0u32, 0u32, 0u32)
}

/// Execute the contract's early exits and its zero-vector full path.
pub unsafe fn rw_00ca0b10(
    task: u32,
    first_selector_arg: u32,
    second_selector_arg: u32,
    third_selector_arg: u32,
    final_scale_bits: u32,
) -> u32 {
    let first = unsafe { bone_method(task, first_selector_arg) };
    if first == 0xFF {
        return 0xFF;
    }
    let second = unsafe { bone_method(task, second_selector_arg) };
    if second == 0xFF {
        return 0xFF;
    }
    let third = unsafe { bone_method(task, third_selector_arg) };
    if third == 0xFF {
        return 0xFF;
    }

    let selected_source = unsafe { bone_method(task, first_selector_arg) };
    let first_provider = lf_checker_rt::callee_thiscall!(5, u32, task, selected_source);
    let first_manager = lf_checker_rt::callee_thiscall!(6, u32, task);
    let first_table = unsafe { read_u32(first_manager + 4) };
    let first_table_entry = unsafe {
        read_u32(first_table.wrapping_add(selected_source.wrapping_mul(4)))
    };
    let first_vector = lf_checker_rt::callee_thiscall!(5, u32, task, first_table_entry);

    let mut first_frame = [0u32; 16];
    let first_frame_pointer = first_frame.as_mut_ptr() as usize as u32;
    let _ = lf_checker_rt::callee_thiscall!(7, u32, first_frame_pointer, first_provider);

    let angle_a_bits = call_double_helper();
    let _ = lf_checker_rt::callee_cdecl!(9, f32, SIGNED_ZERO_F32_BITS);
    let angle_b_bits = call_double_helper();

    let fourth = unsafe { bone_method(task, second_selector_arg) };
    let second_provider = lf_checker_rt::callee_thiscall!(5, u32, task, fourth);
    let second_manager = lf_checker_rt::callee_thiscall!(6, u32, task);
    let second_table = unsafe { read_u32(second_manager + 4) };
    let second_table_entry = unsafe {
        read_u32(second_table.wrapping_add(fourth.wrapping_mul(4)))
    };
    let second_vector = lf_checker_rt::callee_thiscall!(5, u32, task, second_table_entry);

    let mut second_frame = [0u32; 16];
    let second_frame_pointer = second_frame.as_mut_ptr() as usize as u32;
    let _ = lf_checker_rt::callee_thiscall!(7, u32, second_frame_pointer, second_provider);

    let final_manager = lf_checker_rt::callee_thiscall!(6, u32, task);
    let base_vector = unsafe { read_u32(final_manager) };
    let final_index_bytes = fourth.wrapping_mul(0xE0);
    let final_angle_bits = call_double_helper();
    let base_angle_bits = unsafe { read_u32(base_vector.wrapping_add(final_index_bytes + 0x30)) };
    let final_angle = f32::from_bits(final_angle_bits as u32);
    let base_angle = f32::from_bits(base_angle_bits);
    let first_x87_input = sub_f32(final_angle, base_angle);
    let first_x87 = lf_checker_rt::callee_cdecl!(10, f32, first_x87_input.to_bits());
    let scale = f32::from_bits(final_scale_bits);
    let second_x87_input = add_f32(mul_f32(first_x87, scale), 0.0);
    let second_x87 = lf_checker_rt::callee_cdecl!(10, f32, second_x87_input.to_bits());

    let final_selector = unsafe { bone_method(task, third_selector_arg) };
    let final_destination = lf_checker_rt::callee_thiscall!(5, u32, task, final_selector);
    let angle_output = [
        0,
        second_x87.to_bits(),
        f32::from_bits(angle_b_bits as u32).to_bits(),
        0,
    ];
    let angle_output_pointer = angle_output.as_ptr() as usize as u32;
    let _ = lf_checker_rt::callee_thiscall!(11, u32, final_destination, angle_output_pointer);
    let _ = lf_checker_rt::callee_thiscall!(12, u32, final_destination, first_vector);

    let mut word = 0;
    while word < RESULT_WORD_COUNT {
        let offset = RESULT_WORDS_OFFSET + (word as u32) * 4;
        let value = unsafe { read_u32(first_provider + offset) };
        unsafe { write_u32(final_destination + offset, value) };
        word += 1;
    }

    let _ = (first_vector, second_vector, angle_a_bits, angle_b_bits, ZERO_F64_BITS);
    0
}
