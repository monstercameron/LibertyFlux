// original: 0x00D3B3E0 ped_task_clear_motion_reference

/// If the motion reference at `this`+8 is nonnull, pass the duration bit pattern and reference to the stop helper, notify the owner, then clear the reference. The duration is transported as one exact 32-bit float word; no arithmetic changes its bits.
lf_checker_rt::export!(thiscall, rw_00d3b3e0(this: u32, duration_bits: u32) -> u32 {
    unsafe {
        const MOTION_REFERENCE: u32 = 0x08;
        const STOP_MOTION: u32 = 1;
        const NOTIFY_OWNER: u32 = 2;
        let slot = this.wrapping_add(MOTION_REFERENCE);
        let motion = (slot as *const u32).read_unaligned();
        if motion != 0 {
            let _ = lf_checker_rt::callee_thiscall!(STOP_MOTION, u32, motion, duration_bits);
            let _ = lf_checker_rt::callee_thiscall!(NOTIFY_OWNER, u32, motion, this);
            (slot as *mut u32).write_unaligned(0);
        }
        0
    }
});
