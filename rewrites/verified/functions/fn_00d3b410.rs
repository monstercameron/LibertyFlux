// original: 0x00D3B410 ped_task_release_motion_with_fixed_duration

/// Forward `this` to the motion cleanup helper with the fixed duration -8.0f. The original passes the float as one stack word; this wrapper's return value is unused.
lf_checker_rt::export!(thiscall, rw_00d3b410(this: u32) -> u32 {
    unsafe {
        const MOTION_CLEANUP: u32 = 1;
        let _ = lf_checker_rt::callee_thiscall!(MOTION_CLEANUP, u32, this, 0xc1000000);
        0
    }
});
