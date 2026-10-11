// original: 0x00cab5d0 CTaskComplexGoToPointAndStandStillTimed::CTaskComplexGoToPointAndStandStillTimed

/// Constructs a timed go-to-point task. The first argument selects a scripted
/// single-precision value; that value, the target-vector pointer and two
/// scalar words are forwarded to the embedded move-task constructor. The
/// fifth argument is stored at `+0x40`. The derived vtables are installed,
/// `+0x44..+0x4d` is cleared, and the constructor returns `this`.
lf_checker_rt::export!(thiscall, rw_00cab5d0(
    this: u32,
    mode: u32,
    target_vector: u32,
    start_limit: u32,
    target_limit: u32,
    duration: u32
) -> u32 {
    const FLOAT_SELECTOR_CALLEE: u32 = 1;
    const MOVE_TASK_CTOR_CALLEE: u32 = 2;
    const TASK_VTABLE_VA: u32 = 0x00ed_852c;
    const BASE_TASK_VTABLE_VA: u32 = 0x00ed_8584;
    const DURATION: u32 = 0x40;

    let selected_float = lf_checker_rt::callee_cdecl!(
        FLOAT_SELECTOR_CALLEE,
        f32,
        mode
    );
    let _ = lf_checker_rt::callee_thiscall!(
        MOVE_TASK_CTOR_CALLEE,
        u32,
        this,
        selected_float.to_bits(),
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
