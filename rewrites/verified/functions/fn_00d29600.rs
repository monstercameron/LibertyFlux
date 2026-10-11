// original: 0x00D29600 task_manager_update_task_progress (proposed)

/// Update eight 0x40-byte task records. Empty pointers are skipped. For each
/// present task, a COMISS comparison of zero against the limit at slot offset
/// +8 takes the helper path when the limit is positive or unordered (NaN).
/// That path calls the receiver's +0x24 virtual helper, multiplies its result
/// by the first direct helper's result in that order, optionally multiplies
/// by the receiver's +8 virtual helper when the signed saved index equals the
/// slot, then adds the final helper result to the accumulator. A nonpositive
/// limit stores 0.1, except that the selected slot stores the product of two
/// relocated global scales. The thiscall method consumes three stack words.
/// The original uses its second incoming word as scratch for helper results;
/// those values are reproduced locally and flow to the receiver accumulator.
lf_checker_rt::export!(thiscall, rw_00D29600(
    this: u32,
    first_argument: u32,
    second_argument: u32,
    third_float_bits: u32,
) -> () {
    const SLOT_POINTERS: u32 = 0x34;
    const SLOT_ACCUMULATOR: u32 = 0x3c;
    const SLOT_LIMIT: u32 = 0x44;
    const SLOT_STRIDE: u32 = 0x40;
    const SLOT_COUNT: u32 = 8;
    const SAVED_SLOT_INDEX: u32 = 0x228;
    const VTABLE_POINTER: u32 = 0;
    const VIRTUAL_PROGRESS_SLOT: u32 = 0x24;
    const VIRTUAL_SCALE_SLOT: u32 = 0x08;
    const DIRECT_WEIGHT_CALLEE: u32 = 2;
    const DIRECT_DELTA_CALLEE: u32 = 4;
    const GLOBAL_PROGRESS_SCALE: u32 = 0x01054628;
    const GLOBAL_TIME_SCALE: u32 = 0x00fe879c;

    #[inline(always)]
    fn ordered_mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn ordered_add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let signed_selected_slot = crate::read_u16(this.wrapping_add(SAVED_SLOT_INDEX)) as i16 as i32;
        let vtable = crate::read_u32(this.wrapping_add(VTABLE_POINTER));
        let progress_function = crate::read_u32(vtable.wrapping_add(VIRTUAL_PROGRESS_SLOT));
        let scale_function = crate::read_u32(vtable.wrapping_add(VIRTUAL_SCALE_SLOT));
        let progress_call: extern "thiscall" fn(u32, u32) -> f32 =
            core::mem::transmute(progress_function as usize);
        let scale_call: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(scale_function as usize);

        for slot in 0..SLOT_COUNT {
            let slot_displacement = slot.wrapping_mul(SLOT_STRIDE);
            let task = crate::read_u32(this.wrapping_add(SLOT_POINTERS).wrapping_add(slot_displacement));
            if task == 0 {
                continue;
            }

            let accumulator_address = this.wrapping_add(SLOT_ACCUMULATOR).wrapping_add(slot_displacement);
            let limit_bits = crate::read_u32(this.wrapping_add(SLOT_LIMIT).wrapping_add(slot_displacement));
            let limit = f32::from_bits(limit_bits);
            let selected = slot as i32 == signed_selected_slot;

            if !(limit.is_nan() || limit > 0.0) {
                let mut accumulator = f32::from_bits(0x3dcc_cccd);
                if selected {
                    let progress_scale = f32::from_bits(core::ptr::read(
                        lf_checker_rt::global::<u32>(GLOBAL_PROGRESS_SCALE),
                    ));
                    let time_scale = f32::from_bits(core::ptr::read(
                        lf_checker_rt::global::<u32>(GLOBAL_TIME_SCALE),
                    ));
                    accumulator = ordered_mul(progress_scale, time_scale);
                }
                crate::write_u32(accumulator_address, accumulator.to_bits());
                continue;
            }

            let progress = progress_call(this, task);
            crate::write_u32(accumulator_address, progress.to_bits());

            let weight = lf_checker_rt::callee_thiscall!(
                DIRECT_WEIGHT_CALLEE,
                f32,
                this,
                first_argument,
                second_argument,
                task,
            );
            let mut accumulator = ordered_mul(weight, progress);
            crate::write_u32(accumulator_address, accumulator.to_bits());

            if selected {
                let scale = scale_call(this);
                accumulator = ordered_mul(scale, accumulator);
                crate::write_u32(accumulator_address, accumulator.to_bits());
            }

            let detail_pointer = accumulator_address.wrapping_sub(0x1c);
            let previous_pointer = accumulator_address.wrapping_sub(4);
            let delta = lf_checker_rt::callee_thiscall!(
                DIRECT_DELTA_CALLEE,
                f32,
                this,
                previous_pointer,
                second_argument,
                third_float_bits,
                detail_pointer,
            );
            accumulator = ordered_add(delta, accumulator);
            crate::write_u32(accumulator_address, accumulator.to_bits());
        }
    }
});
