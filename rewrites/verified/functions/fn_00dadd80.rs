// original: 0x00DADD80 task_planar_reference_frame_vectors

/// Builds a normalized planar direction and its perpendicular frame vector.
///
/// `this` supplies the base entity through `+0x14/+0x20` and the reference
/// entity through `+0x18/+0x20`. A null reference entity instead uses the
/// reference object's two words at `+0x10/+0x14`. The method subtracts the
/// base position from the reference position, scales that difference by the
/// stored `1.0001` normalization bias, writes it to the first four-word
/// output view, rotates it by ninety degrees for the second view, and
/// normalizes that view with the same bias. The output pointers may overlap;
/// the second view is written after the first. The original reads an
/// uninitialized local word into each output's fourth slot; under the
/// contract's zero stack fill that word is zero.
lf_checker_rt::export!(thiscall, rw_00DADD80(this: u32, first: u32, second: u32) -> () {
    const BASE_OBJECT: u32 = 0x14;
    const REFERENCE_OBJECT: u32 = 0x18;
    const OBJECT_ENTITY: u32 = 0x20;
    const OBJECT_FALLBACK_X: u32 = 0x10;
    const ENTITY_POSITION_X: u32 = 0x30;
    const ENTITY_POSITION_Y: u32 = 0x34;
    const REFERENCE_ENTITY_POSITION: u32 = 0x30;
    const NORMALIZATION_BIAS_VA: u32 = 0x00FE_88E8;
    const PERPENDICULAR_SIGN_VA: u32 = 0x00FE_8FA0;
    const LOCAL_SCRATCH: u32 = 0;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
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

    #[inline(always)]
    fn normalize_scale(x: f32, y: f32, bias: f32) -> f32 {
        let x_squared = multiply(x, x);
        let y_squared = multiply(y, y);
        let squared_length = add(x_squared, y_squared);
        if squared_length == 0.0 {
            0.0
        } else {
            divide(bias, squared_length.sqrt())
        }
    }

    unsafe {
        let base_object = read_word(this.wrapping_add(BASE_OBJECT));
        let reference_object = read_word(this.wrapping_add(REFERENCE_OBJECT));
        let base_entity = read_word(base_object.wrapping_add(OBJECT_ENTITY));
        let reference_entity = read_word(reference_object.wrapping_add(OBJECT_ENTITY));
        let (reference_x, reference_y) = if reference_entity != 0 {
            (
                read_float(reference_entity.wrapping_add(REFERENCE_ENTITY_POSITION)),
                read_float(reference_entity.wrapping_add(REFERENCE_ENTITY_POSITION + 4)),
            )
        } else {
            (
                read_float(reference_object.wrapping_add(OBJECT_FALLBACK_X)),
                read_float(reference_object.wrapping_add(OBJECT_FALLBACK_X + 4)),
            )
        };
        let base_x = read_float(base_entity.wrapping_add(ENTITY_POSITION_X));
        let base_y = read_float(base_entity.wrapping_add(ENTITY_POSITION_Y));
        let delta_x = subtract(reference_x, base_x);
        let delta_y = subtract(reference_y, base_y);
        let bias = lf_checker_rt::global::<f32>(NORMALIZATION_BIAS_VA).read_unaligned();
        let first_scale = normalize_scale(delta_y, delta_x, bias);
        let first_x = multiply(first_scale, delta_x);
        let first_y = multiply(delta_y, first_scale);
        let first_z = multiply(first_scale, 0.0);
        let perpendicular_x = f32::from_bits(
            multiply(delta_y, first_scale).to_bits()
                ^ lf_checker_rt::global::<u32>(PERPENDICULAR_SIGN_VA).read_unaligned(),
        );
        let perpendicular_y = multiply(first_scale, delta_x);
        let second_scale = normalize_scale(perpendicular_x, perpendicular_y, bias);
        let second_x = multiply(perpendicular_x, second_scale);
        let second_y = multiply(perpendicular_y, second_scale);
        let second_z = multiply(second_scale, 0.0);

        write_word(first, first_x.to_bits());
        write_word(first.wrapping_add(4), first_y.to_bits());
        write_word(first.wrapping_add(8), first_z.to_bits());
        write_word(first.wrapping_add(12), LOCAL_SCRATCH);
        write_word(second, second_x.to_bits());
        write_word(second.wrapping_add(4), second_y.to_bits());
        write_word(second.wrapping_add(8), second_z.to_bits());
        write_word(second.wrapping_add(12), LOCAL_SCRATCH);
    }
});
