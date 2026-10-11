// original: 0x00B80640 CTaskComplexDriveToPoint::vf21

/// Run the shared driving-task method on `this`, then copy the global game
/// timer word into the embedded task at `this + 0x40 + 0x0e38`. The timer word
/// is returned in EAX. The shared method's result is ignored.
lf_checker_rt::export!(thiscall, rw_b80640(this: u32) -> u32 {
    unsafe {
        const EMBEDDED_TASK: u32 = 0x40;
        const TIMER_FIELD: u32 = 0x0e38;
        const TIMER_GLOBAL: u32 = 0x011735b4;
        const DRIVE_METHOD: u32 = 1;

        let _: u32 = lf_checker_rt::callee_thiscall!(DRIVE_METHOD, u32, this);
        let embedded_task = ((this + EMBEDDED_TASK) as *const u32).read_unaligned();
        let timer = (lf_checker_rt::relocated(TIMER_GLOBAL) as *const u32).read_unaligned();
        ((embedded_task + TIMER_FIELD) as *mut u32).write_unaligned(timer);
        timer
    }
});
