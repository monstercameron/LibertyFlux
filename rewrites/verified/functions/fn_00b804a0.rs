// original: 0x00B804A0 CTaskComplexCarDrive::vf21

/// Forward to the shared driving-task helper with the embedded task object at
/// `this + 0x40`, advanced by `0x0dd4` bytes. The helper's EAX result becomes
/// this function's result; the adjusted address uses 32-bit wrapping arithmetic.
lf_checker_rt::export!(thiscall, rw_b804a0(this: u32) -> u32 {
    unsafe {
        const EMBEDDED_TASK: u32 = 0x40;
        const HELPER_VIEW: u32 = 0x0dd4;
        const DRIVE_HELPER: u32 = 1;
        let embedded_task = ((this + EMBEDDED_TASK) as *const u32).read_unaligned();
        let helper_view = embedded_task.wrapping_add(HELPER_VIEW);
        lf_checker_rt::callee_thiscall!(DRIVE_HELPER, u32, helper_view)
    }
});
