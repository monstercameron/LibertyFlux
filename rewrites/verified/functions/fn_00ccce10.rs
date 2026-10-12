// original: 0x00CCCE10 ped_task_update
/// Updates a pedestrian task's pose and animation state. It obtains the
/// current pose, optionally applies the early pose adjustment, prepares the
/// task's motion context, updates the associated animation matrix, and clears
/// the transient receiver flag before returning the receiver.

#[inline(never)]
fn sub_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(never)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(never)]
fn mul_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(never)]
fn jbe_f32(left: f32, right: f32) -> bool {
    match core::hint::black_box(left).partial_cmp(&core::hint::black_box(right)) {
        Some(core::cmp::Ordering::Greater) => false,
        Some(_) | None => true,
    }
}

#[inline(never)]
fn ja_f32(left: f32, right: f32) -> bool {
    matches!(
        core::hint::black_box(left).partial_cmp(&core::hint::black_box(right)),
        Some(core::cmp::Ordering::Greater)
    )
}

#[inline(never)]
fn ordered_equal_f32(left: f32, right: f32) -> bool {
    matches!(
        core::hint::black_box(left).partial_cmp(&core::hint::black_box(right)),
        Some(core::cmp::Ordering::Equal)
    )
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    (address as usize as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    (address as usize as *mut u32).write_unaligned(value);
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(read_u32(address))
}

#[inline(always)]
unsafe fn write_f32(address: u32, value: f32) {
    write_u32(address, value.to_bits());
}

lf_checker_rt::export!(thiscall, ped_task_update(this: u32, ped: u32, mode: u32) -> u32 {
    unsafe {
        let mut pose_words = [0u32; 16];
        let pose_address = pose_words.as_mut_ptr() as usize as u32;
        let _pose_status = lf_checker_rt::callee_thiscall!(1, u32, ped, pose_address, 0);

        let sign_words = lf_checker_rt::global::<[u32; 4]>(0x00FE8FA0).read_unaligned();
        let first_angle = f32::from_bits(pose_words[4] ^ sign_words[0]) as f64;
        let second_angle = f32::from_bits(pose_words[5]) as f64;
        let first_angle_bits = first_angle.to_bits();
        let second_angle_bits = second_angle.to_bits();
        let angle_answer: u64 = lf_checker_rt::callee_cdecl!(
            2,
            u64,
            first_angle_bits as u32,
            (first_angle_bits >> 32) as u32,
            second_angle_bits as u32,
            (second_angle_bits >> 32) as u32
        );
        let half_pi = lf_checker_rt::global::<f32>(0x00FE8978).read_unaligned();
        let adjusted_angle = sub_f32(f64::from_bits(angle_answer) as f32, half_pi);

        let receiver_flags = (this.wrapping_add(0x18)) as *mut u8;
        receiver_flags.write(receiver_flags.read() & !0x20);

        let mut stack_flag = false;
        let mut motion_words = [pose_words[12], pose_words[13], pose_words[14], pose_words[15]];
        let pose_enabled = ((ped.wrapping_add(0x118)) as *const u8).read() & 1 != 0;

        if pose_enabled && (mode as u8) == 0 {
            let _pose_status = lf_checker_rt::callee_thiscall!(
                1,
                u32,
                ped,
                pose_address,
                0x36A0u32
            );
            motion_words.copy_from_slice(&pose_words[12..16]);

            let motion_address = motion_words.as_mut_ptr() as usize as u32;
            let _motion_status: u32 = lf_checker_rt::callee_cdecl!(
                3,
                u32,
                motion_address,
                0u32,
                0u32,
                0u32,
                0x42C80000u32,
                0x0Eu32,
                0u32
            );

            let mut adjusted_pose_bits = 0u32;
            let adjusted_pose_address = core::ptr::addr_of_mut!(adjusted_pose_bits) as usize as u32;
            let emit_result: u32 = lf_checker_rt::callee_cdecl!(
                4,
                u32,
                motion_words[0],
                motion_words[1],
                motion_words[2],
                adjusted_pose_address,
                1u32,
                0u32,
                0u32,
                0x40C00000u32,
                0x41A00000u32,
                0u32
            );

            if (emit_result as u8) != 0 {
                let difference = sub_f32(
                    f32::from_bits(adjusted_pose_bits),
                    f32::from_bits(motion_words[2]),
                );
                let upper_bound = lf_checker_rt::global::<f32>(0x00FE892C).read_unaligned();
                if jbe_f32(difference, upper_bound) {
                    let lower_bound = lf_checker_rt::global::<f32>(0x00FE881C).read_unaligned();
                    if ja_f32(difference, lower_bound) {
                        return emit_result;
                    }
                } else {
                    stack_flag = true;
                    receiver_flags.write(receiver_flags.read() | 0x20);
                    let bias = lf_checker_rt::global::<f32>(0x00FE87E8).read_unaligned();
                    motion_words[2] = sub_f32(f32::from_bits(motion_words[2]), bias).to_bits();
                }
            }
        }

        let mut handle_slot = motion_words[0];
        let mut chosen_object = 0u32;
        if receiver_flags.read() & 0x20 == 0 {
            let global_328 = lf_checker_rt::global::<f32>(0x01B4B328).read_unaligned();
            let global_320 = lf_checker_rt::global::<f32>(0x01B4B320).read_unaligned();
            let global_324 = lf_checker_rt::global::<f32>(0x01B4B324).read_unaligned();
            let mut handle_metadata = [global_328.to_bits(), 0u32];
            let mut request_block = [0u32; 102];
            request_block[0] = this;
            request_block[4] = global_320.to_bits();
            request_block[5] = global_324.to_bits();
            request_block[6] = global_328.to_bits();
            request_block[8] = global_320.to_bits();
            request_block[9] = global_324.to_bits();
            request_block[10] = global_328.to_bits();
            request_block[12] = global_320.to_bits();
            request_block[13] = global_324.to_bits();
            request_block[14] = global_328.to_bits();
            request_block[16] = this;
            request_block[17] = this;
            request_block[19] = 0xFFFF;
            request_block[20] = 0;
            let request_address = request_block.as_mut_ptr() as usize as u32;
            let handle_address = core::ptr::addr_of_mut!(handle_slot) as usize as u32;
            let metadata_address = handle_metadata.as_mut_ptr() as usize as u32;
            let _created = lf_checker_rt::callee_cdecl!(
                10,
                u32,
                handle_address,
                metadata_address,
                0x3DCCCCCDu32,
                0u32,
                request_address,
                0x19Eu32,
                0x20u32,
                1u32,
                0u32
            );

            if (_created as u8) != 0 {
                let candidate: u32 = lf_checker_rt::callee_cdecl!(11, u32, 0u32);
                if candidate != 0 {
                    let class_tag = (read_u32(candidate.wrapping_add(0x28)) >> 6) & 0xF;
                    if class_tag > 1 && class_tag < 5 {
                        stack_flag = true;
                        if class_tag == 2 && read_u32(candidate.wrapping_add(0x1304)) == 2 {
                            chosen_object = candidate;
                        }
                    }
                }
            }
            core::hint::black_box(global_328);
        }

        let mut animation_context = [0u32; 20];
            animation_context[0] = adjusted_angle.to_bits();
            let animation_context_address = animation_context.as_mut_ptr() as usize as u32;
            let _animation: u32 = lf_checker_rt::callee_thiscall!(
                5,
                u32,
                ped,
                animation_context_address,
                1u32,
                0u32
            );

            let angle_bits = adjusted_angle.to_bits();
            let _set_angle_a: u32 = lf_checker_rt::callee_thiscall!(12, u32, ped, angle_bits);
            let _set_angle_b: u32 = lf_checker_rt::callee_thiscall!(13, u32, ped, angle_bits);
            let sine_bits: u32 = lf_checker_rt::callee_cdecl!(14, u32, angle_bits);
            let cosine_bits: u32 = lf_checker_rt::callee_cdecl!(15, u32, angle_bits);

            let mut vector_context = [0u32; 20];
            vector_context[0] = sine_bits;
            vector_context[4] = adjusted_angle.to_bits() ^ sign_words[0];
            vector_context[5] = sine_bits;
            vector_context[9] = 0;
            vector_context[10] = 0;
            vector_context[11] = 0x3F800000;
            vector_context[13..17].copy_from_slice(&motion_words);
            let vector_context_address = vector_context.as_mut_ptr() as usize as u32;
            let _set_transform: u32 = lf_checker_rt::callee_thiscall!(
                6,
                u32,
                ped,
                vector_context_address,
                1u32,
                0u32
            );
            let transform_tail = f32::from_bits(vector_context[10]);

            let matrix = read_u32(ped.wrapping_add(0x7B4));
            let matrix_values = [
                0.0,
                0.0,
                0.0,
                f32::from_bits(cosine_bits ^ sign_words[0]),
                0.0,
                0.0,
                0.0,
                0.0,
                transform_tail,
                f32::from_bits(handle_slot),
                f32::from_bits(motion_words[1]),
                f32::from_bits(vector_context[14]),
            ];
            for (index, value) in matrix_values.into_iter().enumerate() {
                let destination = match index {
                    0..=2 => 0x10 + index as u32 * 4,
                    3..=5 => 0x20 + (index as u32 - 3) * 4,
                    6..=8 => 0x30 + (index as u32 - 6) * 4,
                    _ => 0x40 + (index as u32 - 9) * 4,
                };
                write_f32(matrix.wrapping_add(destination), value);
            }
            let transform_values = [
                f32::from_bits(motion_words[0]),
                f32::from_bits(motion_words[1]),
                f32::from_bits(motion_words[2]),
            ];
            let transform_address = transform_values.as_ptr() as usize as u32;
            let _matrix_update: u32 = lf_checker_rt::callee_thiscall!(7, u32, matrix, transform_address);
            let _ped_update: u32 = lf_checker_rt::callee_thiscall!(16, u32, ped);

            let nested_matrix = read_u32(matrix.wrapping_add(4));
            let nested_leaf = read_u32(nested_matrix.wrapping_add(0xC));
            let previous_component = read_f32(nested_leaf.wrapping_add(8));
            let _matrix_probe_a: u32 = lf_checker_rt::callee_thiscall!(8, u32, matrix);
            let _matrix_probe_b: u32 = lf_checker_rt::callee_thiscall!(8, u32, matrix);
            let nested_matrix = read_u32(matrix.wrapping_add(4));
            let nested_leaf = read_u32(nested_matrix.wrapping_add(0xC));
            let current_component = read_f32(nested_leaf.wrapping_add(8));
            let compare_tag = (read_u32(matrix.wrapping_add(8)) & 0xFFFF) as u32;
            let manager = lf_checker_rt::global::<u32>(0x012B9C78).read_unaligned();
            let compare_args = [compare_tag, previous_component.to_bits(), 0u32];
            if ordered_equal_f32(previous_component, current_component) {
                let _notify: u32 = lf_checker_rt::callee_thiscall!(
                    18,
                    u32,
                    manager,
                    compare_args[0],
                    0u32
                );
            } else {
                let _notify: u32 = lf_checker_rt::callee_thiscall!(
                    17,
                    u32,
                    manager,
                    compare_args[0],
                    compare_args[1],
                    compare_args[2]
                );
            }

            let mut selected_task = read_u32(ped.wrapping_add(0xB94));
            let task_kind: u32 = lf_checker_rt::callee_cdecl!(19, u32, selected_task, 0xFu32);
            if task_kind == 0 {
                let fallback_task: u32 = lf_checker_rt::callee_thiscall!(20, u32, ped);
                let fallback_kind: u32 = lf_checker_rt::callee_cdecl!(19, u32, fallback_task, 0xFu32);
                if fallback_kind == 0 {
                    selected_task = 0x31;
                } else {
                    selected_task = fallback_task;
                }
            }
            let _schedule: u32 = lf_checker_rt::callee_thiscall!(
                21,
                u32,
                read_u32(ped.wrapping_add(0x78)),
                selected_task,
                0xFu32,
                0x447A0000u32,
                0xFFFFFFFFu32
            );

            let mut selected_motion = read_u32(ped.wrapping_add(0x100));
            let active_motion: u32 = lf_checker_rt::callee_thiscall!(9, u32, ped);
            if active_motion != 0 {
                let _active_subtask: u32 = lf_checker_rt::callee_thiscall!(9, u32, ped);
                selected_motion = lf_checker_rt::callee_thiscall!(
                    8,
                    u32,
                    active_motion
                );
            }
            let _apply_motion: u32 = lf_checker_rt::callee_thiscall!(
                22,
                u32,
                read_u32(ped.wrapping_add(0x78)),
                selected_motion,
                0x3F800000u32,
                0u32
            );
            let ped_anim_flags = (ped.wrapping_add(0xF4)) as *mut u8;
            if ped_anim_flags.read() & 0x20 == 0 {
                ped_anim_flags.write(ped_anim_flags.read() | 0x20);
            }
            ped_anim_flags.write(ped_anim_flags.read() | 3);

            if stack_flag {
                let _reset_state: u32 = lf_checker_rt::callee_thiscall!(23, u32, ped, 1u32, 1u32, 0u32);
                let task_flags = (ped.wrapping_add(0x26C)) as *mut u32;
                task_flags.write(task_flags.read() | 3);
                write_u32(ped.wrapping_add(0xB14), 0xBF800000);
            } else {
                let _reset_state: u32 = lf_checker_rt::callee_thiscall!(23, u32, ped, 0u32, 1u32, 0u32);
                let matrix = read_u32(ped.wrapping_add(0x7B0));
                for (index, value) in matrix_values.into_iter().enumerate() {
                    let destination = match index {
                        0..=2 => 0x10 + index as u32 * 4,
                        3..=5 => 0x20 + (index as u32 - 3) * 4,
                        6..=8 => 0x30 + (index as u32 - 6) * 4,
                        _ => 0x40 + (index as u32 - 9) * 4,
                    };
                    write_f32(matrix.wrapping_add(destination), value);
                }
            }

            if chosen_object != 0 {
                let mut object_matrix = read_u32(chosen_object.wrapping_add(0x20));
                if object_matrix == 0 {
                    let _ensure: u32 = lf_checker_rt::callee_thiscall!(24, u32, chosen_object);
                    let _attach: u32 = lf_checker_rt::callee_thiscall!(
                        25,
                        u32,
                        chosen_object.wrapping_add(0x10),
                        read_u32(chosen_object.wrapping_add(0x20))
                    );
                    object_matrix = read_u32(chosen_object.wrapping_add(0x20));
                }
                let ped_matrix = read_u32(ped.wrapping_add(0x20));
                let mut delta = [0.0f32; 3];
                for i in 0..3 {
                    delta[i] = sub_f32(
                        read_f32(ped_matrix.wrapping_add(0x30 + i as u32 * 4)),
                        read_f32(object_matrix.wrapping_add(0x30 + i as u32 * 4)),
                    );
                }

                let mut transformed = [0.0f32; 3];
                for row in 0usize..3 {
                    let base = object_matrix.wrapping_add(row as u32 * 0x10);
                    let x = mul_f32(read_f32(base), delta[0]);
                    let y = mul_f32(read_f32(base.wrapping_add(4)), delta[1]);
                    let z = mul_f32(read_f32(base.wrapping_add(8)), delta[2]);
                    transformed[row] = add_f32(add_f32(y, x), z);
                }

                if read_u32(chosen_object.wrapping_add(0x20)) == 0 {
                    let _ensure: u32 = lf_checker_rt::callee_thiscall!(24, u32, chosen_object);
                    let _attach: u32 = lf_checker_rt::callee_thiscall!(
                        25,
                        u32,
                        chosen_object.wrapping_add(0x10),
                        read_u32(chosen_object.wrapping_add(0x20))
                    );
                }
                object_matrix = read_u32(chosen_object.wrapping_add(0x20));
                if object_matrix == 0 {
                    let _ensure: u32 = lf_checker_rt::callee_thiscall!(24, u32, chosen_object);
                    let _attach: u32 = lf_checker_rt::callee_thiscall!(
                        25,
                        u32,
                        chosen_object.wrapping_add(0x10),
                        read_u32(chosen_object.wrapping_add(0x20))
                    );
                }
                object_matrix = read_u32(chosen_object.wrapping_add(0x20));

                let first_axis = read_f32(object_matrix.wrapping_add(4)) as f64;
                let second_axis = read_f32(object_matrix) as f64;
                let first_axis_bits = first_axis.to_bits();
                let second_axis_bits = second_axis.to_bits();
                let orientation_bits: u64 = lf_checker_rt::callee_cdecl!(
                    2,
                    u64,
                    first_axis_bits as u32,
                    (first_axis_bits >> 32) as u32,
                    second_axis_bits as u32,
                    (second_axis_bits >> 32) as u32
                );
                let orientation = f64::from_bits(orientation_bits) as f32;
                let orientation_delta = sub_f32(adjusted_angle, orientation);
                let transformed_address = transformed.as_ptr() as usize as u32;
                let _apply_parameters: u32 = lf_checker_rt::callee_thiscall!(
                    26,
                    u32,
                    ped,
                    chosen_object,
                    0u32,
                    0x1105u32,
                    transformed_address,
                    orientation_delta.to_bits(),
                    0u32
                );
            }
        let receiver_flags = (this.wrapping_add(0x18)) as *mut u8;
        receiver_flags.write(receiver_flags.read() & !0x10);
        this
    }
});
