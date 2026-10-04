// original: 0x00CAB2D0 task_promote_spare (proposed)

/// Promote the spare task slot to current when no task is running.
///
/// `this` holds a current-task pointer at `+0x04`, a running flag word at
/// `+0x08` and a spare-task pointer at `+0x0C`. When the spare is null, or a
/// task is already running, nothing changes. Otherwise the spare moves to
/// current and the spare is cleared. Returns the spare's entry value on every
/// path (matching the original's eax). The single stack argument is not read.
/// No calls.
///
/// Original: 0x00CAB2D0 (thiscall, one unread stack word).
lf_checker_rt::export!(thiscall, rw_00cab2d0(this: u32, _a0: u32) -> u32 {
    unsafe {
        const CURRENT: u32 = 0x04;
        const RUNNING: u32 = 0x08;
        const SPARE: u32 = 0x0C;
        let spare = (this.wrapping_add(SPARE) as *const u32).read_unaligned();
        if spare == 0 {
            return 0;
        }
        let running = (this.wrapping_add(RUNNING) as *const u32).read_unaligned();
        if running != 0 {
            return spare;
        }
        (this.wrapping_add(CURRENT) as *mut u32).write_unaligned(spare);
        (this.wrapping_add(SPARE) as *mut u32).write_unaligned(0);
        spare
    }
});
