// original: 0x00CBA090 CTaskSimpleMoveAchieveHeading::vf17

/// Refresh a heading task from its ped's matrix and movement state.
///
/// `this` is the task object. `ped` is the sole stack argument. The method
/// copies the four-word world transform, asks the movement component and
/// distance helpers for their scripted values, stores the scaled distance,
/// and returns one when the ped is already flagged as reached or the measured
/// distance is within the task's arrival limit. The final comparison follows
/// COMISS unordered semantics: a NaN takes the not-reached path.
///
/// The source class layout identifies this as slot 17 of
/// `CTaskSimpleMoveAchieveHeading`; its inherited field names are not
/// established, so offsets remain named constants. The checker substitutes
/// each outgoing method with its contract stand-in.
lf_checker_rt::export!(thiscall, rw_00cba090(this: u32, ped: u32) -> u8 {
    const TASK_VALUE: u32 = 0x20;
    const TASK_SCALE: u32 = 0x24;
    const ARRIVAL_LIMIT: u32 = 0x28;
    const TASK_POSITION: u32 = 0x30;
    const PED_MATRIX: u32 = 0x20;
    const PED_FLAGS: u32 = 0x26c;
    const PED_DISTANCE: u32 = 0xaa0;
    const PED_SCALED_DISTANCE: u32 = 0xaa8;
    const PED_MOVEMENT_COMPONENT: u32 = 0xa80;
    const REACHED_FLAG: u8 = 0x04;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
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
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    unsafe {
        // The movement component accepts three zero-valued stack words and
        // clears its two cached vector components through the stand-in.
        let movement_component = read_u32(ped + PED_MOVEMENT_COMPONENT);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, movement_component, 0, 0, 0);

        // Refresh the task's world transform from the ped's matrix.
        let matrix = read_u32(ped + PED_MATRIX);
        for offset in [0u32, 4, 8, 12] {
            write_u32(this + TASK_POSITION + offset, read_u32(matrix + 0x30 + offset));
        }

        if read_u8(ped + PED_FLAGS) & REACHED_FLAG != 0 {
            return 1;
        }

        // The movement update helper seeds the ped's scaled-distance field.
        let _ = lf_checker_rt::callee_thiscall!(2, u32, ped);

        // Scale the helper's distance by the task's configured speed.
        let base_distance: f32 = lf_checker_rt::callee_thiscall!(3, f32, ped);
        let scaled_distance = mul_f32(base_distance, read_f32(this + TASK_SCALE));
        write_f32(ped + PED_SCALED_DISTANCE, scaled_distance);

        // Preserve the task value through the original setter call.
        let task_value = read_u32(this + TASK_VALUE);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, ped, task_value);

        // Compare the helper's distance from the ped's current value against
        // the configured arrival limit. COMISS treats unordered values as
        // below, so NaNs take the not-reached path.
        let delta = sub_f32(read_f32(this + TASK_VALUE), read_f32(ped + PED_DISTANCE));
        let absolute_delta = f32::from_bits(delta.to_bits() & 0x7fff_ffff);
        let measured_distance: f32 = lf_checker_rt::callee_cdecl!(5, f32, absolute_delta.to_bits());
        let absolute_distance = f32::from_bits(measured_distance.to_bits() & 0x7fff_ffff);
        let arrival_limit = read_f32(this + ARRIVAL_LIMIT);
        if arrival_limit >= absolute_distance {
            let current_distance = read_u32(ped + PED_DISTANCE);
            let _ = lf_checker_rt::callee_thiscall!(6, u32, ped, current_distance);
            let _ = lf_checker_rt::callee_thiscall!(7, u32, ped);
            let _ = lf_checker_rt::callee_thiscall!(8, u32, this, ped);
            return 1;
        }

        let _ = lf_checker_rt::callee_thiscall!(9, u32, this, ped);
        0
    }
});
