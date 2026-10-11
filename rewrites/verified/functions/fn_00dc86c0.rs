// original: 0x00dc86c0 CTaskComplexEvasiveStep::vf0
/// Run the first cleanup helper, then conditionally notify the global task
/// manager when the low bit of the mode byte is set. The method returns the
/// task pointer and its thiscall ABI removes one four-byte stack argument.
lf_checker_rt::export!(thiscall, rw_00dc86c0(this_ptr: u32, mode: u32) -> u32 {
    unsafe {
        const FIRST_CLEANUP: u32 = 1;
        const GLOBAL_MANAGER_VA: u32 = 0x0167e2a0;
        const NOTIFY_MANAGER: u32 = 2;
        let _first_result = lf_checker_rt::callee_thiscall!(FIRST_CLEANUP, u32, this_ptr);
        if mode & 1 != 0 {
            let manager = (lf_checker_rt::relocated(GLOBAL_MANAGER_VA) as *const u32).read_unaligned();
            let _notified = lf_checker_rt::callee_thiscall!(NOTIFY_MANAGER, u32, manager, this_ptr);
        }
        this_ptr
    }
});
