// original: 0x00B7A310 vehicle_task_apply_cleanup_when_subobject_ready

/// Checks a vehicle task's embedded cleanup component.
///
/// Calls the component check at `+0xBB0`, tests only the low byte of its return value, and when that byte is nonzero calls cleanup at the same subobject with the one-word value 0xFA. The object pointer is the function's sole stdcall argument; the function returns the last call's EAX value when cleanup runs, otherwise the check's EAX value.
lf_checker_rt::export!(stdcall, rw_00b7a310(task: u32) -> u32 {
    unsafe {
        const CHECK_COMPONENT: u32 = 1;
        const APPLY_CLEANUP: u32 = 2;
        const COMPONENT_OFFSET: u32 = 0xBB0;
        const CLEANUP_CODE: u32 = 0xFA;

        let component = task.wrapping_add(COMPONENT_OFFSET);
        let check_result = lf_checker_rt::callee_thiscall!(CHECK_COMPONENT, u32, component);
        if check_result as u8 != 0 {
            lf_checker_rt::callee_thiscall!(APPLY_CLEANUP, u32, component, CLEANUP_CODE)
        } else {
            check_result
        }
    }
});
