// original: 0x00cfc220 CTaskComplexCombatBustPed::CTaskComplexCombatBustPed
/// CTaskComplexCombatBustPed::CTaskComplexCombatBustPed constructor.
///
/// Initializes the documented task fields, forwards base/member work through the
/// declared call stubs, and returns the object pointer. Field offsets and the
/// 32-bit pointer layout follow the merged structure notes where available.
lf_checker_rt::export!(thiscall, rw_00cfc220(this: *mut u8, task_type: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EE018C;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32, task_type);
        *(this as *mut u32) = lf_checker_rt::relocated(TASK_VTABLE);
        this as u32
    }
});
