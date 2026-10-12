// original: 0x00D6AEF0 retain_replay_ui_child

/// Returns zero when the requested child pointer is null. Otherwise forwards the child pointer at object offset 0x1c and the request word to StoreAndAcquireChildRef, returning that helper's result. The method uses thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00d6aef0(this: u32, request: u32) -> u32 {
    if request == 0 {
        0
    } else {
        unsafe {
            const CHILD_POINTER: u32 = 0x1c;
            let child = (this.wrapping_add(CHILD_POINTER) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(1, u32, child, request)
        }
    }
});
