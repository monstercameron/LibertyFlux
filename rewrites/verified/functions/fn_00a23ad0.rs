// original: 0x00a23ad0 camera_record_advance_scalar_toward_target

/// Move the stored scalar at 0x320 toward the target at 0x31c by the input
/// step scaled by 0.12, clamping each ordered step at the target. Mode values
/// zero and one snap immediately; mode three also snaps when the target is
/// zero. Unordered comparisons follow the original COMISS branch outcomes.
lf_checker_rt::export!(thiscall, rw_00a23ad0(this: u32, step_bits: u32) -> u32 {
    const TARGET: u32 = 0x31c;
    const CURRENT: u32 = 0x320;
    const MODE: u32 = 0x130;
    const SNAP_MODE: u32 = 0x360;
    const STEP_SCALE: f32 = f32::from_bits(0x3df5_c28f);
    const ZERO_THRESHOLD: f32 = 0.0;
    unsafe {
        let target = (this.wrapping_add(TARGET) as *const f32).read_unaligned();
        let current = (this.wrapping_add(CURRENT) as *const f32).read_unaligned();
        let scaled_step = core::hint::black_box(f32::from_bits(step_bits))
            * core::hint::black_box(STEP_SCALE);
        let next = if target > current {
            let candidate = core::hint::black_box(current) + core::hint::black_box(scaled_step);
            if candidate > target { target } else { candidate }
        } else {
            let candidate = core::hint::black_box(current) - core::hint::black_box(scaled_step);
            if target > candidate { target } else { candidate }
        };
        (this.wrapping_add(CURRENT) as *mut f32).write_unaligned(next);
        let mode = (this.wrapping_add(MODE) as *const u32).read_unaligned();
        let snap_mode = (this.wrapping_add(SNAP_MODE) as *const u32).read_unaligned();
        if mode <= 1 || (snap_mode == 3 && target == ZERO_THRESHOLD) {
            (this.wrapping_add(CURRENT) as *mut f32).write_unaligned(target);
        }
    }
    0
});
