// original: 0x00d56100 ccam_init_timing_params

/// Write the camera's default timing parameter block, then run the shared
/// follow-up with argument 0.
///
/// `this` points to the object. Fifteen words of tuning constants (integer
/// counts and float bit patterns) are stored at their offsets, then the
/// follow-up routine runs with `this` in ECX and one stack argument of 0.
/// Returns the follow-up's answer.
///
/// Original: 0x00d56100 (thiscall, no stack arguments, one call).
lf_checker_rt::export!(thiscall, rw_00d56100(this: u32) -> u32 {
    unsafe {
        /// Follow-up routine (intercepted; thiscall, one stack argument).
        const FOLLOW_UP: u32 = 1;
        ((this + 0x224) as *mut u32).write_unaligned(0x7d0);
        ((this + 0x238) as *mut u32).write_unaligned(0xbb8);
        ((this + 0x244) as *mut u32).write_unaligned(0x40800000);
        ((this + 0x248) as *mut u32).write_unaligned(0x41700000);
        ((this + 0x24c) as *mut u32).write_unaligned(0x42a00000);
        ((this + 0x280) as *mut u32).write_unaligned(0x3f800000);
        ((this + 0x284) as *mut u32).write_unaligned(0xbb8);
        ((this + 0x258) as *mut u32).write_unaligned(0x41200000);
        ((this + 0x25c) as *mut u32).write_unaligned(0x42340000);
        ((this + 0x264) as *mut u32).write_unaligned(0x447a0000);
        ((this + 0x23c) as *mut u32).write_unaligned(0xe);
        ((this + 0x274) as *mut u32).write_unaligned(0x2ee0);
        ((this + 0x230) as *mut u32).write_unaligned(0x1388);
        ((this + 0x268) as *mut u32).write_unaligned(0x40000000);
        lf_checker_rt::callee_thiscall!(FOLLOW_UP, u32, this, 0)
    }
});
