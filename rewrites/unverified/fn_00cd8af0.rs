// original: 0x00CD8AF0 CTaskComplexFollowPedFootsteps::vf19

/// Call the task's virtual callback with the fixed code when the override
/// pointer is null. Otherwise compute the squared distance between the stored
/// and supplied XYZ positions, then call the same callback with the near code
/// when the value is at most the global threshold (including unordered float
/// comparisons), or the far code when it is greater.
///
/// The override pointer is at `this + 0x14`; the position object is reached
/// through its `0x20` pointer and contains three floats at `0x30`, `0x34` and
/// `0x38`. The supplied position pointer is at argument offset `0x20`. The
/// squared terms and additions follow the original's y, x, z order.
///
/// Calling convention: thiscall with one 32-bit stack pointer argument. The
/// selected vtable callback is thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_00cd8af0(this: u32, input: u32) -> u32 {
    const OVERRIDE_POINTER: u32 = 0x14;
    const POSITION_POINTER: u32 = 0x20;
    const POSITION_X: u32 = 0x30;
    const POSITION_Y: u32 = 0x34;
    const POSITION_Z: u32 = 0x38;
    const VTABLE_SLOT: u32 = 0x54;
    const DISTANCE_THRESHOLD: u32 = 0x00FE_88E8;
    const NEAR_CODE: u32 = 0xCB;
    const FAR_CODE: u32 = 0x38B;
    const OVERRIDE_CODE: u32 = 0x516;

    unsafe {
        let vtable = (this as *const u32).read_unaligned();
        let callback_address = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned();
        let callback: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callback_address as usize);

        let override_object = ((this + OVERRIDE_POINTER) as *const u32).read_unaligned();
        if override_object == 0 {
            return callback(this, OVERRIDE_CODE, input);
        }

        let position_object = ((override_object + POSITION_POINTER) as *const u32).read_unaligned();
        let input_object = ((input + POSITION_POINTER) as *const u32).read_unaligned();
        let stored_x = ((position_object + POSITION_X) as *const u32).read_unaligned();
        let stored_y = ((position_object + POSITION_Y) as *const u32).read_unaligned();
        let stored_z = ((position_object + POSITION_Z) as *const u32).read_unaligned();
        let input_x = ((input_object + POSITION_X) as *const u32).read_unaligned();
        let input_y = ((input_object + POSITION_Y) as *const u32).read_unaligned();
        let input_z = ((input_object + POSITION_Z) as *const u32).read_unaligned();

        let subtract = |left_bits: u32, right_bits: u32| {
            let left = core::hint::black_box(f32::from_bits(left_bits));
            let right = core::hint::black_box(f32::from_bits(right_bits));
            core::hint::black_box(left - right).to_bits()
        };
        let multiply = |left_bits: u32, right_bits: u32| {
            let left = core::hint::black_box(f32::from_bits(left_bits));
            let right = core::hint::black_box(f32::from_bits(right_bits));
            core::hint::black_box(left * right).to_bits()
        };
        let add = |left_bits: u32, right_bits: u32| {
            let left = core::hint::black_box(f32::from_bits(left_bits));
            let right = core::hint::black_box(f32::from_bits(right_bits));
            core::hint::black_box(left + right).to_bits()
        };

        let dy = subtract(stored_y, input_y);
        let dx = subtract(stored_x, input_x);
        let dz = subtract(stored_z, input_z);
        let dy_squared = multiply(dy, dy);
        let dx_squared = multiply(dx, dx);
        let dz_squared = multiply(dz, dz);
        let xy_squared = add(dy_squared, dx_squared);
        let distance_squared = add(xy_squared, dz_squared);
        let threshold_bits = lf_checker_rt::global::<u32>(DISTANCE_THRESHOLD).read();
        let distance = core::hint::black_box(f32::from_bits(distance_squared));
        let threshold = core::hint::black_box(f32::from_bits(threshold_bits));
        let comparison = distance.partial_cmp(&threshold);
        let code = if comparison != Some(core::cmp::Ordering::Greater) {
            NEAR_CODE
        } else {
            FAR_CODE
        };
        callback(this, code, input)
    }
});
