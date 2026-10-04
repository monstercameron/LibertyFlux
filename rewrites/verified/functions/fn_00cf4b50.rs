// original: 0x00cf4b50 climb_task_aim_setup (proposed)

/// Sets up a climb task's aim state: runs the aim probe with the target
/// object, the constants 9 and 106 and the float 1000.0, then runs the aim
/// commit with the task's second word, the task's third word as a float and
/// the constants 0.5 and -0.7, returning the commit's result. Finally sets
/// bit 2 of the flag byte at `+0x89`, clears the low two bits of the target's
/// word at `+0x26c` and writes 8 to `+0x14`.
///
/// Original: 0x00cf4b50 (thiscall: ecx holds the object, three stack words).
lf_checker_rt::export!(thiscall, rw_00cf4b50(this: u32, target: u32, second: u32, third: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const COMMIT_CALLEE: u32 = 2;
        const PROBE_FLOAT: u32 = 0x447a_0000; // 1000.0f
        const COMMIT_B: u32 = 0x3f00_0000; // 0.5f
        const COMMIT_C: u32 = 0xbf66_6666; // -0.7f (approx)
        const FLAG_OFFSET: u32 = 0x89;
        const TARGET_MASK_OFF: u32 = 0x26c;
        const STATE_SLOT: u32 = 0x14;
        lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, this, target, 9, 0x6a, PROBE_FLOAT);
        let out = lf_checker_rt::callee_thiscall!(COMMIT_CALLEE, u32, this, second, third, COMMIT_B, COMMIT_C);
        let flags = ((this + FLAG_OFFSET) as *const u8).read();
        ((this + FLAG_OFFSET) as *mut u8).write(flags | 4);
        let tw = ((target + TARGET_MASK_OFF) as *const u32).read_unaligned();
        ((target + TARGET_MASK_OFF) as *mut u32).write_unaligned(tw & 0xffff_fffc);
        ((this + STATE_SLOT) as *mut u32).write_unaligned(8);
        out
    }
});
