// original: 0x0064F7D0 ptxSprite_field_update
/// Advances one sprite node against a bounded keyframe record table. It
/// normalizes the node direction, chooses an eligible record, asks the
/// geometry helpers whether that record applies, samples two animation
/// channels, applies a small random directional perturbation, updates the
/// node position and tangent, and returns whether any record was applied.
///
/// The manager, table owner, and node are separate pointers. The fourth
/// stack word is a movement scale. The final stack word is forwarded to the
/// sampling helper. The low 32 bits of XMM3 hold the comparison threshold.
lf_checker_rt::export!(thiscall, u133_sprite_update(
    manager: u32,
    table_owner: u32,
    node: u32,
    move_scale_bits: u32,
    _sample_context: u32,
) -> u32 {
    const CAL_SQRT: u32 = 1;
    const CAL_CLIP: u32 = 2;
    const CAL_SAMPLE: u32 = 3;
    const CAL_INTERSECT: u32 = 4;

    const OWNER_TABLE_PTR: u32 = 0x1FC;
    const TABLE_HEAD_DATA: u32 = 0x00;
    const TABLE_HEAD_COUNT: u32 = 0x04;
    const RECORD_STRIDE: u32 = 0x60;
    const RECORD_VECTOR: u32 = 0x40;
    const RECORD_KIND: u32 = 0x50;
    const RECORD_OPTIONAL: u32 = 0x30;

    const MANAGER_RADIUS: u32 = 0x118;
    const MANAGER_FLAGS: u32 = 0x11C;
    const MANAGER_SAMPLER_A: u32 = 0x3D4;
    const MANAGER_REVERSE_A: u32 = 0x405;
    const MANAGER_SAMPLER_B: u32 = 0x408;
    const MANAGER_REVERSE_B: u32 = 0x439;
    const MANAGER_RANDOM_CHANCE: u32 = 0x133;

    const NODE_TANGENT: u32 = 0xB0;
    const NODE_POSITION: u32 = 0xD0;
    const NODE_INDEX_B: u32 = 0x86;
    const NODE_INDEX_A: u32 = 0x87;
    const NODE_KEY: u32 = 0xF0;
    const NODE_NEAR_FLAG: u32 = 0x81;

    const RNG_POINTER_VA: u32 = 0x018B7444;
    const NOISE_TABLE_VA: u32 = 0x018B7588;
    const ONE_VA: u32 = 0x00FE88E8;
    const THRESHOLD_SCALE_VA: u32 = 0x00FE8874;
    const SIGN_MASK_VA: u32 = 0x00FE8FA0;
    const RANGE_SCALE_VA: u32 = 0x00FE870C;
    const NOISE_SCALE_VA: u32 = 0x00FE864C;
    const TWO_VA: u32 = 0x00FE8A24;

    const RNG_MULTIPLIER: u64 = 0x5CDCFaa7;
    const RNG_FRACTION_MASK: u32 = 0x007F_FFFF;
    const RNG_CHANCE_MODULUS: u32 = 101;
    const FULL_RECORD_KIND: u32 = 4;
    const CLIP_REJECTED: u32 = 3;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    #[inline(always)]
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { write_u32(address, value.to_bits()) }
    }

    #[inline(always)]
    unsafe fn read_image_word(file_va: u32) -> u32 {
        unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_image_f32(file_va: u32) -> f32 {
        f32::from_bits(unsafe { read_image_word(file_va) })
    }

    #[inline(never)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(never)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    #[inline(never)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(never)]
    fn div_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }

    #[inline(always)]
    fn local_address<T>(value: &mut T) -> u32 {
        value as *mut T as usize as u32
    }

    #[inline(always)]
    fn image_table_value(index: u8) -> f32 {
        let address = NOISE_TABLE_VA.wrapping_add(u32::from(index).wrapping_mul(4));
        unsafe { read_image_f32(address) }
    }

    #[inline(always)]
    fn scripted_sqrt(value: f32) -> f32 {
        let bits = lf_checker_rt::callee_cdecl!(CAL_SQRT, u32, value.to_bits());
        f32::from_bits(bits)
    }

    fn write_sample_pair(output: &mut [u32; 4], channel: u32, key: f32, node_field: u32) {
        let output_address = local_address(output);
        lf_checker_rt::callee_stdcall!(
            CAL_SAMPLE,
            u32,
            output_address,
            channel,
            key.to_bits(),
            node_field,
        );
    }

    fn next_random_words(rng: u32) -> (u32, u32, u32) {
        let low = unsafe { read_u32(rng) };
        let high = unsafe { read_u32(rng.wrapping_add(4)) };
        let first_product = u64::from(low).wrapping_mul(RNG_MULTIPLIER);
        let first_low = first_product as u32;
        let mixed_low = first_low.wrapping_add(high);
        let carry = u32::from(mixed_low < first_low);
        let upper_mix = ((first_product >> 32) as u32).wrapping_add(carry);
        let second_product = u64::from(mixed_low)
            .wrapping_mul(RNG_MULTIPLIER)
            .wrapping_add(u64::from(upper_mix));
        let third_product = u64::from(second_product as u32)
            .wrapping_mul(RNG_MULTIPLIER)
            .wrapping_add(second_product >> 32);
        let state_low = third_product as u32;
        let state_high = (third_product >> 32) as u32;
        unsafe {
            write_u32(rng, state_low);
            write_u32(rng.wrapping_add(4), state_high);
        }
        (mixed_low, second_product as u32, state_low)
    }

    fn advance_random_state(rng: u32) -> u32 {
        let low = unsafe { read_u32(rng) };
        let high = unsafe { read_u32(rng.wrapping_add(4)) };
        let product = u64::from(low).wrapping_mul(RNG_MULTIPLIER);
        let next_low = (product as u32).wrapping_add(high);
        let carry = u32::from(next_low < product as u32);
        let next_high = ((product >> 32) as u32).wrapping_add(carry);
        unsafe {
            write_u32(rng, next_low);
            write_u32(rng.wrapping_add(4), next_high);
        }
        next_low
    }

    let movement_scale = f32::from_bits(move_scale_bits);
    let input_threshold = f32::from_bits(lf_checker_rt::xmm_word(3, 0));
    let one = unsafe { read_image_f32(ONE_VA) };
    let twice = unsafe { read_image_f32(TWO_VA) };
    let threshold_scale = unsafe { read_image_f32(THRESHOLD_SCALE_VA) };
    let range_scale = unsafe { read_image_f32(RANGE_SCALE_VA) };
    let noise_scale = unsafe { read_image_f32(NOISE_SCALE_VA) };
    let sign_mask = unsafe { read_image_word(SIGN_MASK_VA) };

    let radius = unsafe { read_f32(manager.wrapping_add(MANAGER_RADIUS)) };
    let minimum_distance_sq = mul_f32(radius, radius);
    let position_x = unsafe { read_f32(node.wrapping_add(NODE_POSITION)) };
    let position_y = unsafe { read_f32(node.wrapping_add(NODE_POSITION + 4)) };
    let position_z = unsafe { read_f32(node.wrapping_add(NODE_POSITION + 8)) };
    let table_head = unsafe { read_u32(table_owner.wrapping_add(OWNER_TABLE_PTR)) };
    let records = unsafe { read_u32(table_head.wrapping_add(TABLE_HEAD_DATA)) };
    let record_count = unsafe { read_u16(table_head.wrapping_add(TABLE_HEAD_COUNT)) };

    let squared_length_a = add_f32(
        add_f32(mul_f32(position_x, position_x), mul_f32(position_y, position_y)),
        mul_f32(position_z, position_z),
    );
    let initial_length = scripted_sqrt(squared_length_a);
    let scaled_threshold = mul_f32(input_threshold, threshold_scale);

    let squared_length_b = add_f32(
        add_f32(mul_f32(position_y, position_y), mul_f32(position_x, position_x)),
        mul_f32(position_z, position_z),
    );
    let direction_scale = if squared_length_b == 0.0 {
        0.0
    } else {
        div_f32(one, scripted_sqrt(squared_length_b))
    };
    let direction_x = mul_f32(position_x, direction_scale);
    let direction_y = mul_f32(position_y, direction_scale);
    let direction_z = mul_f32(position_z, direction_scale);
    let mut applied_any = false;

    for index in 0..u32::from(record_count) {
        let record = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        let key_x = unsafe { read_f32(record.wrapping_add(RECORD_VECTOR)) };
        let key_y = unsafe { read_f32(record.wrapping_add(RECORD_VECTOR + 4)) };
        let key_z = unsafe { read_f32(record.wrapping_add(RECORD_VECTOR + 8)) };

        let dot_y = mul_f32(mul_f32(key_y, position_y), direction_scale);
        let dot_x = mul_f32(mul_f32(key_x, position_x), direction_scale);
        let dot_z = mul_f32(mul_f32(key_z, position_z), direction_scale);
        let facing_dot = add_f32(add_f32(dot_y, dot_x), dot_z);
        let faces_node = !(facing_dot >= 0.0);
        if !faces_node {
            continue;
        }

        let mut update_time = 0.0f32;
        let scaled_length = mul_f32(initial_length, movement_scale);
        if !(scaled_length > scaled_threshold) {
            let mut key_vector = [key_x, key_y, key_z];
            let mut workspace = [0u32; 7];
            let mut scratch_vector = [0u32; 4];
            let mut returned_distance = 0u32;
            let mut returned_direction = [0.0f32; 3];
            let mut returned_time = 0.0f32;
            let item_kind = unsafe { read_u32(record.wrapping_add(RECORD_KIND)) };
            let clip_result = lf_checker_rt::callee_fastcall!(
                CAL_CLIP,
                u32,
                node.wrapping_add(NODE_TANGENT),
                record,
                move_scale_bits,
                local_address(&mut key_vector),
                local_address(&mut workspace),
                local_address(&mut scratch_vector),
                local_address(&mut returned_distance),
                local_address(&mut returned_direction),
                local_address(&mut returned_time),
            );
            if clip_result == CLIP_REJECTED {
                continue;
            }
            update_time = returned_time;
        } else {
            let mut base_vector = [
                unsafe { read_f32(node.wrapping_add(NODE_TANGENT)) },
                unsafe { read_f32(node.wrapping_add(NODE_TANGENT + 4)) },
                unsafe { read_f32(node.wrapping_add(NODE_TANGENT + 8)) },
            ];
            let mut key_vector = [key_x, key_y, key_z];
            let mut face_workspace = [0u32; 3];
            let mut hit_time = 0.0f32;
            let mut auxiliary_workspace = [0u32; 3];
            let item_kind = unsafe { read_u32(record.wrapping_add(RECORD_KIND)) };
            let optional_record = if item_kind == FULL_RECORD_KIND {
                record.wrapping_add(RECORD_OPTIONAL)
            } else {
                0
            };
            let intersection = lf_checker_rt::callee_fastcall!(
                CAL_INTERSECT,
                u32,
                local_address(&mut base_vector),
                local_address(&mut key_vector),
                record,
                record.wrapping_add(0x10),
                record.wrapping_add(0x20),
                optional_record,
                local_address(&mut face_workspace),
                local_address(&mut hit_time),
                local_address(&mut auxiliary_workspace),
            );
            if intersection != 0 {
                continue;
            }
            update_time = sub_f32(input_threshold, hit_time);
        }

        applied_any = true;

        let mut sample_a = [0u32; 4];
        let threshold_value = unsafe { read_f32(node.wrapping_add(NODE_KEY)) };
        write_sample_pair(
            &mut sample_a,
            manager.wrapping_add(MANAGER_SAMPLER_A),
            threshold_value,
            node.wrapping_add(0x5C),
        );
        let sample_a0 = f32::from_bits(sample_a[0]);
        let sample_a1 = f32::from_bits(sample_a[1]);
        let sample_index_a = unsafe { read_u8(node.wrapping_add(NODE_INDEX_A)) };
        let mut blend_a = image_table_value(sample_index_a);
        if unsafe { read_u8(manager.wrapping_add(MANAGER_REVERSE_A)) } != 0 {
            blend_a = sub_f32(one, blend_a);
        }
        let elapsed_scale = sub_f32(
            one,
            mul_f32(
                add_f32(mul_f32(sub_f32(sample_a1, sample_a0), blend_a), sample_a0),
                range_scale,
            ),
        );

        let mut sample_b = [0u32; 4];
        write_sample_pair(
            &mut sample_b,
            manager.wrapping_add(MANAGER_SAMPLER_B),
            threshold_value,
            node.wrapping_add(0x5C),
        );
        let sample_b0 = f32::from_bits(sample_b[0]);
        let sample_b1 = f32::from_bits(sample_b[1]);
        let sample_index_b = unsafe { read_u8(node.wrapping_add(NODE_INDEX_B)) };
        let mut noise_range = image_table_value(sample_index_b);
        if unsafe { read_u8(manager.wrapping_add(MANAGER_REVERSE_B)) } != 0 {
            noise_range = sub_f32(one, noise_range);
        }
        noise_range = add_f32(mul_f32(sub_f32(sample_b1, sample_b0), noise_range), sample_b0);

        if noise_range != 0.0 {
            let rng = unsafe { read_u32(RNG_POINTER_VA) };
            let (random_mid, random_low, random_high) = next_random_words(rng);
            let negative_range = f32::from_bits(noise_range.to_bits() ^ sign_mask);
            let span = sub_f32(noise_range, negative_range);
            let random_x = add_f32(
                mul_f32(
                    mul_f32((random_low & RNG_FRACTION_MASK) as f32, noise_scale),
                    span,
                ),
                negative_range,
            );
            let random_y = add_f32(
                mul_f32(
                    mul_f32((random_mid & RNG_FRACTION_MASK) as f32, noise_scale),
                    span,
                ),
                negative_range,
            );
            let random_z = add_f32(
                mul_f32(
                    mul_f32((random_high & RNG_FRACTION_MASK) as f32, noise_scale),
                    span,
                ),
                negative_range,
            );

            let reflect_dot = mul_f32(
                add_f32(
                    add_f32(mul_f32(key_y, random_x), mul_f32(key_x, random_y)),
                    mul_f32(random_z, key_z),
                ),
                twice,
            );
            let reflected_y = add_f32(sub_f32(random_x, mul_f32(key_y, reflect_dot)), key_y);
            let reflected_x = add_f32(sub_f32(random_y, mul_f32(key_x, reflect_dot)), key_x);
            let reflected_z = add_f32(sub_f32(random_z, mul_f32(key_z, reflect_dot)), key_z);
            let reflected_length = add_f32(
                add_f32(
                    mul_f32(reflected_y, reflected_y),
                    mul_f32(reflected_x, reflected_x),
                ),
                mul_f32(reflected_z, reflected_z),
            );
            let reflected_scale = if reflected_length == 0.0 {
                0.0
            } else {
                div_f32(one, scripted_sqrt(reflected_length))
            };
            let axis_x = mul_f32(reflected_x, reflected_scale);
            let axis_y = mul_f32(reflected_y, reflected_scale);
            let axis_z = mul_f32(reflected_z, reflected_scale);

            let current_x = unsafe { read_f32(node.wrapping_add(NODE_POSITION)) };
            let current_y = unsafe { read_f32(node.wrapping_add(NODE_POSITION + 4)) };
            let current_z = unsafe { read_f32(node.wrapping_add(NODE_POSITION + 8)) };
            let position_dot = mul_f32(
                add_f32(
                    add_f32(mul_f32(current_y, axis_y), mul_f32(current_x, axis_x)),
                    mul_f32(current_z, axis_z),
                ),
                twice,
            );
            let projected_x = sub_f32(current_x, mul_f32(axis_x, position_dot));
            let projected_y = sub_f32(current_y, mul_f32(axis_y, position_dot));
            let projected_z = sub_f32(current_z, mul_f32(axis_z, position_dot));
            let moved_x = mul_f32(projected_x, elapsed_scale);
            let moved_y = mul_f32(projected_y, elapsed_scale);
            let moved_z = mul_f32(projected_z, elapsed_scale);
            let moved_length = add_f32(
                add_f32(mul_f32(moved_y, moved_y), mul_f32(moved_x, moved_x)),
                mul_f32(moved_z, moved_z),
            );
            if moved_length < minimum_distance_sq {
                unsafe {
                    write_u32(node.wrapping_add(NODE_POSITION), 0);
                    write_u32(node.wrapping_add(NODE_POSITION + 4), 0);
                    write_u32(node.wrapping_add(NODE_POSITION + 8), 0);
                    (node.wrapping_add(NODE_NEAR_FLAG) as *mut u8).write_unaligned(1);
                }
            } else {
                unsafe {
                    write_f32(node.wrapping_add(NODE_POSITION), moved_x);
                    write_f32(node.wrapping_add(NODE_POSITION + 4), moved_y);
                    write_f32(node.wrapping_add(NODE_POSITION + 8), moved_z);
                }
            }
        }

        if unsafe { read_u32(manager.wrapping_add(MANAGER_FLAGS)) } & 1 != 0 {
            let rng = unsafe { read_u32(RNG_POINTER_VA) };
            let random_value = advance_random_state(rng) & 0x7FFF_FFFF;
            let chance = random_value % RNG_CHANCE_MODULUS;
            if chance <= u32::from(unsafe { read_u8(manager.wrapping_add(MANAGER_RANDOM_CHANCE)) }) {
                unsafe { write_u32(node.wrapping_add(NODE_KEY), one.to_bits()) };
            }
        }

        let tangent_x = unsafe { read_f32(node.wrapping_add(NODE_TANGENT)) };
        let tangent_y = unsafe { read_f32(node.wrapping_add(NODE_TANGENT + 4)) };
        let tangent_z = unsafe { read_f32(node.wrapping_add(NODE_TANGENT + 8)) };
        unsafe {
            write_f32(
                node.wrapping_add(NODE_TANGENT),
                add_f32(mul_f32(key_x, update_time), tangent_x),
            );
            write_f32(
                node.wrapping_add(NODE_TANGENT + 4),
                add_f32(mul_f32(key_y, update_time), tangent_y),
            );
            write_f32(
                node.wrapping_add(NODE_TANGENT + 8),
                add_f32(mul_f32(key_z, update_time), tangent_z),
            );
        }
    }

    u32::from(applied_any)
});
