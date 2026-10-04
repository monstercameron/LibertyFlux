// original: 0x00cf6520 climb_task_aim_setup_idle (proposed)

/// Sets the busy bit (0x20) at `+0xbe0` of the target, and when the state
/// word at `+0x10` is clear runs the aim probe with the target and constants
/// (9, 0x69, 1000.0) followed by the aim commit with the task's second word,
/// third word as a float and constants (0.4, -0.7), writing 2 to `+0x14` and
/// returning the commit's result. When the state word is set the target
/// itself is returned.
///
/// Original: 0x00cf6520 (thiscall: ecx holds the object, three stack words).
lf_checker_rt::export!(thiscall, rw_00cf6520(this: u32, target: u32, second: u32, third: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const COMMIT_CALLEE: u32 = 2;
        const PROBE_FLOAT: u32 = 0x447a_0000; // 1000.0f
        const COMMIT_B: u32 = 0x3ecc_cccd; // 0.4f (approx)
        const COMMIT_C: u32 = 0xbfaf_5c29; // -0.7f (approx)
        let w = ((target + 0xbe0) as *const u32).read_unaligned();
        ((target + 0xbe0) as *mut u32).write_unaligned(w | 0x20);
        let state = ((this + 0x10) as *const u32).read_unaligned();
        if state != 0 {
            return target;
        }
        lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, this, target, 9, 0x69, PROBE_FLOAT);
        let out = lf_checker_rt::callee_thiscall!(COMMIT_CALLEE, u32, this, second, third, COMMIT_B, COMMIT_C);
        ((this + 0x14) as *mut u32).write_unaligned(2);
        out
    }
});
