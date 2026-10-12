// original: 0x00BDF1F0 CTaskSimpleDuckToggle::CTaskSimpleDuckToggle

/// Runs the base simple-task constructor, stores the relocated duck-toggle
/// vtable at offset zero, and records the supplied mode at offset 0x14. The
/// thiscall receiver is returned in EAX; the single 32-bit mode is popped by
/// the constructor.
lf_checker_rt::export!(thiscall, rw_bdf1f0(this: u32, mode: u32) -> u32 {
    unsafe {
        const BASE_TASK_CTOR: u32 = 1;
        const DUCK_TOGGLE_VTABLE: u32 = 0x00EB8BE4;
        const MODE_FIELD: u32 = 0x14;

        let _ = lf_checker_rt::callee_thiscall!(BASE_TASK_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(DUCK_TOGGLE_VTABLE));
        (this.wrapping_add(MODE_FIELD) as *mut u32).write_unaligned(mode);
        this
    }
});
