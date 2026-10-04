// original: 0x00a6ed60 task_request_abort_or_release (proposed)

/// Runs two teardown hooks on the task `arg` (callees 1 and 2, both called
/// with `arg` in ecx), clears status bit 3 of the control block linked from
/// the task, then asks the task whether it finished (callee 3, its low byte
/// is the answer) and reports the outcome to the task (callee 4).
///
/// Layout: the control block pointer is read from `arg + TASK_CTL`; when it
/// is non-null the flag word is at `ctl + 0x70 + CTL_FLAGS` (i.e. the flags
/// live `CTL_FLAGS` bytes past the block's header), otherwise the flag word
/// address is `CTL_FLAGS` itself and the access faults, exactly like the
/// original. The final report passes `-1` and `1` when the task finished and
/// this object's enable byte (`this + ENABLED`) is non-zero, else `-1` and
/// `0`. Returns the report call's result.
///
/// Original: thiscall, one stack word (`arg`), callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a6ed60(this: u32, arg: u32) -> u32 {
    unsafe {
        const TASK_CTL: u32 = 0x228;
        const CTL_HEADER: u32 = 0x70;
        const CTL_FLAGS: u32 = 0x3d0;
        const KEEP_MASK: u32 = 0xffff_fff7;
        const ENABLED: u32 = 0x19;
        const TEARDOWN_A: u32 = 1;
        const TEARDOWN_B: u32 = 2;
        const ASK_DONE: u32 = 3;
        const REPORT: u32 = 4;

        lf_checker_rt::callee_thiscall!(TEARDOWN_A, u32, arg);
        lf_checker_rt::callee_thiscall!(TEARDOWN_B, u32, arg);
        let ctl = ((arg as *const u32).wrapping_byte_offset(TASK_CTL as isize)).read_unaligned();
        let flags_at = if ctl == 0 {
            CTL_FLAGS
        } else {
            ctl.wrapping_add(CTL_HEADER).wrapping_add(CTL_FLAGS)
        } as *mut u32;
        flags_at.write_unaligned(flags_at.read_unaligned() & KEEP_MASK);
        let done: u32 = lf_checker_rt::callee_thiscall!(ASK_DONE, u32, arg);
        let enabled = ((this as *const u8).wrapping_byte_offset(ENABLED as isize)).read();
        let finished = if (done as u8) != 0 && enabled != 0 { 1u32 } else { 0u32 };
        lf_checker_rt::callee_thiscall!(REPORT, u32, arg, finished, 0xffff_ffff)
    }
});
