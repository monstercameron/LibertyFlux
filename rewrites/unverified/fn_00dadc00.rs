// original: 0x00DADC00 task_planar_frame_angle

/// Selects a planar direction against a reference direction and returns the
/// angle produced by the game helpers.
///
/// `this` points to two object references at `+0x14` and `+0x18`, a
/// normalization flag at `+0x1c`, and a planar point at `+0x20/+0x24`. The
/// second argument points to a frame whose entity reference is at `+0x20`.
/// The vector-frame helper fills two four-word output views that overlap by
/// one word. This method computes both dot products in scalar float order,
/// chooses one pair, multiplies it by the stored negative-one constant only
/// when its signed dot product is negative, then returns the final helper's
/// ST(0) result.
///
/// The helper calls remain outgoing calls so their arguments, return bits,
/// and the output words are observed by the contract.
lf_checker_rt::export!(thiscall, rw_00DADC00(this: u32, frame: u32) -> f32 {
    const NORMALIZE_FLAG: u32 = 0x1C;
    const POINT_X: u32 = 0x20;
    const POINT_Y: u32 = 0x24;
    const ENTITY_POSITION_X: u32 = 0x30;
    const ENTITY_POSITION_Y: u32 = 0x34;
    const ENTITY_FORWARD_X: u32 = 0x10;
    const ENTITY_FORWARD_Y: u32 = 0x14;
    const ENTITY_FORWARD_Z: u32 = 0x18;
    const FRAME_ENTITY: u32 = 0x20;
    const VECTOR_FRAME_HELPER: u32 = 1;
    const PLANAR_ANGLE_HELPER: u32 = 2;
    const FINAL_ANGLE_HELPER: u32 = 3;
    const NORMALIZATION_BIAS_VA: u32 = 0x00FE_88E8;
    const NEGATE_SCALE_VA: u32 = 0x00FE_8D94;
    const ABS_MASK_VA: u32 = 0x00FE_8F80;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }

    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
    }

    #[inline(always)]
    fn subtract(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) - core::hint::black_box(right))
    }

    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
    }

    #[inline(always)]
    fn divide(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) / core::hint::black_box(right))
    }

    unsafe {
        let mut overlapping_outputs = [0u32; 7];
        let first_output = overlapping_outputs.as_mut_ptr() as u32;
        let second_output = first_output.wrapping_add(3 * 4);
        let _ = lf_checker_rt::callee_thiscall!(
            VECTOR_FRAME_HELPER,
            u32,
            this,
            first_output,
            second_output
        );

        let frame_entity = read_word(frame.wrapping_add(FRAME_ENTITY));
        let (direction_x, direction_y, direction_z) =
            if read_word(this.wrapping_add(NORMALIZE_FLAG)) & 0xFF != 0 {
                let point_x = read_float(this.wrapping_add(POINT_X));
                let point_y = read_float(this.wrapping_add(POINT_Y));
                let entity_x = read_float(frame_entity.wrapping_add(ENTITY_POSITION_X));
                let entity_y = read_float(frame_entity.wrapping_add(ENTITY_POSITION_Y));
                let delta_x = subtract(point_x, entity_x);
                let delta_y = subtract(point_y, entity_y);
                let squared_length = add(
                    multiply(delta_y, delta_y),
                    multiply(delta_x, delta_x),
                );
                let inverse_length = if squared_length == 0.0 {
                    0.0
                } else {
                    let bias = lf_checker_rt::global::<f32>(NORMALIZATION_BIAS_VA).read_unaligned();
                    divide(bias, squared_length.sqrt())
                };
                (
                    multiply(inverse_length, delta_x),
                    multiply(delta_y, inverse_length),
                    multiply(inverse_length, 0.0),
                )
            } else {
                (
                    read_float(frame_entity.wrapping_add(ENTITY_FORWARD_X)),
                    read_float(frame_entity.wrapping_add(ENTITY_FORWARD_Y)),
                    read_float(frame_entity.wrapping_add(ENTITY_FORWARD_Z)),
                )
            };

        let first_x = f32::from_bits(overlapping_outputs[0]);
        let first_y = f32::from_bits(overlapping_outputs[1]);
        let first_z = f32::from_bits(overlapping_outputs[2]);
        let second_y = f32::from_bits(overlapping_outputs[4]);
        let second_z = f32::from_bits(overlapping_outputs[5]);
        let second_scratch = f32::from_bits(overlapping_outputs[6]);

        let first_dot = add(
            add(multiply(first_y, direction_y), multiply(first_x, direction_x)),
            multiply(first_z, direction_z),
        );
        let second_dot = add(
            add(multiply(second_z, direction_y), multiply(second_y, direction_x)),
            multiply(second_scratch, direction_z),
        );
        let abs_mask = lf_checker_rt::global::<u32>(ABS_MASK_VA).read_unaligned();
        let first_magnitude = f32::from_bits(first_dot.to_bits() & abs_mask);
        let second_magnitude = f32::from_bits(second_dot.to_bits() & abs_mask);
        let negate_scale = lf_checker_rt::global::<f32>(NEGATE_SCALE_VA).read_unaligned();

        let (mut selected_x, mut selected_y, selected_dot) =
            if second_magnitude > first_magnitude {
                (second_y, second_z, second_dot)
            } else {
                (first_x, first_y, first_dot)
            };
        if selected_dot < 0.0 {
            selected_x = multiply(selected_x, negate_scale);
            selected_y = multiply(selected_y, negate_scale);
        }

        let intermediate = lf_checker_rt::callee_cdecl!(
            PLANAR_ANGLE_HELPER,
            f32,
            selected_x.to_bits(),
            selected_y.to_bits(),
            0,
            0
        );
        lf_checker_rt::callee_cdecl!(FINAL_ANGLE_HELPER, f32, intermediate.to_bits())
    }
});
