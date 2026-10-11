// original: 0x00cab640 CTaskComplexGoToPointAndStandStillTimed::CTaskComplexGoToPointAndStandStillTimed_2

/// Constructs the alternate timed go-to-point task using the first scalar
/// word directly instead of selecting it through the float helper. The six
/// words forwarded to the embedded move-task constructor are that scalar,
/// the target-vector pointer, two settings and two zero flags. The fifth
/// incoming argument is stored at `+0x40`; the derived vtables and zeroed
/// timer fields match the other overload.
lf_checker_rt::export!(thiscall, rw_00cab640(
    this: u32,
    selected_float: u32,
    target_vector: u32,
    start_limit: u32,
    target_limit: u32,
    duration: u32
) -> u32 {
    const MOVE_TASK_CTOR_CALLEE: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_852c;
    const BASE_TASK_VTABLE_VA: u32 = 0x00ed_8584;
    const DURATION: u32 = 0x40;

    let _ = lf_checker_rt::callee_thiscall!(
        MOVE_TASK_CTOR_CALLEE,
        u32,
        this,
        selected_float,
        target_vector,
        start_limit,
        target_limit,
        0,
        0
    );
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(BASE_TASK_VTABLE_VA));
        ((this + DURATION) as *mut u32).write_unaligned(duration);
        ((this + 0x44) as *mut u32).write_unaligned(0);
        ((this + 0x48) as *mut u32).write_unaligned(0);
        ((this + 0x4c) as *mut u16).write_unaligned(0);
    }
    this
});
