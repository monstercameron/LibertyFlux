// original: 0x00BEE110 update_task_visual_targets

/// Refresh the target record for this task. The stack arguments are a target
/// record, a source object and an interpolation factor; the target kind is a
/// signed 16-bit field at `+0x2e`. The function first calls a thiscall helper,
/// then only updates one of three registered target kinds. A flag at target
/// `+0x24` or a factor equal to the zero threshold copies the task's current
/// two float words from `this + 0x70/+0x74`. When the factor differs from the
/// threshold, two status queries gate an interpolation helper. The method is
/// thiscall and cleans three arguments.
lf_checker_rt::export!(thiscall, rw_00BEE110(this: u32, target: u32, source: u32, factor_bits: u32) -> u32 {
    const TARGET_KIND: u32 = 0x2E;
    const TARGET_FLAGS: u32 = 0x24;
    const TARGET_FLAG_COPY: u32 = 0x400;
    const CURRENT_X: u32 = 0x70;
    const CURRENT_Y: u32 = 0x74;
    const OUTPUT_X: u32 = 0x149C;
    const OUTPUT_Y: u32 = 0x1498;
    const KIND_A: u32 = 0x012FA47C;
    const KIND_B: u32 = 0x012FA008;
    const KIND_C: u32 = 0x012FA248;
    const ZERO_THRESHOLD: u32 = 0x00FE8628;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, target, source, factor_bits);
        let signed_kind = read_u16(target + TARGET_KIND) as i16 as i32;
        let matches_registered_kind = signed_kind == lf_checker_rt::global::<i32>(KIND_A).read_unaligned()
            || signed_kind == lf_checker_rt::global::<i32>(KIND_B).read_unaligned()
            || signed_kind == lf_checker_rt::global::<i32>(KIND_C).read_unaligned();
        if !matches_registered_kind {
            return signed_kind as u32;
        }

        let threshold = f32::from_bits(read_u32(lf_checker_rt::relocated(ZERO_THRESHOLD)));
        let factor = f32::from_bits(factor_bits);
        let must_copy_current = read_u32(target + TARGET_FLAGS) & TARGET_FLAG_COPY != 0
            || factor == threshold;
        if must_copy_current {
            let current_x = read_u32(this + CURRENT_X);
            let current_y = read_u32(this + CURRENT_Y);
            write_u32(target + OUTPUT_X, current_x);
            write_u32(target + OUTPUT_Y, current_y);
            return current_y;
        }

        let first_status = lf_checker_rt::callee_thiscall!(2, u32, this);
        if first_status == 0x00FF_FFFF {
            let current_x = read_u32(this + CURRENT_X);
            let current_y = read_u32(this + CURRENT_Y);
            write_u32(target + OUTPUT_X, current_x);
            write_u32(target + OUTPUT_Y, current_y);
            return current_y;
        }

        let second_status = lf_checker_rt::callee_thiscall!(3, u32, this);
        if second_status == 0 || source == 0 {
            return second_status;
        }
        lf_checker_rt::callee_thiscall!(4, u32, this, target, source, factor_bits)
    }
});
