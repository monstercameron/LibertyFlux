// original: 0x00d407a0 CTaskSimpleJumpLaunch::vf5

/// Decide whether a jump-launch task accepts the requested transition.
///
/// `this` is the task, `kind` (arg1) the requested transition, arg0 and
/// arg2 unused. Returns 0 unless `kind` equals 2. On 2, the settle callee
/// runs (thiscall on `this` with `SETTLE_ARG`, -32.0f as bits), bit 3 of
/// the task flag byte at `TASK_FLAGS` (+0x92) is cleared, and 1 is
/// returned. Only al carries the result.
///
/// Original: 0x00d407a0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d407a0(this: u32, _a0: u32, kind: u32, _a2: u32) -> u32 {
    unsafe {
        const SETTLE: u32 = 1;
        const SETTLE_ARG: u32 = 0xc100_0000;
        const TASK_FLAGS: u32 = 0x92;
        const KEEP_MASK: u8 = 0xf7;

        if kind != 2 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SETTLE, u32, this, SETTLE_ARG);
        let flags = (this + TASK_FLAGS) as *mut u8;
        flags.write(flags.read() & KEEP_MASK);
        1
    }
});
