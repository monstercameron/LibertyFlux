// original: 0x00d57080 CCamHistory::destructor

/// Runs the task teardown helper for CCamHistory, installs the class vtable, then forwards `this` to the shared camera base destructor and returns its result.
///
/// `this` is a 32-bit pointer to the CCamHistory object; the object uses the shared camera layout.
/// Original calling convention: thiscall with 0 explicit stack argument(s).
lf_checker_rt::export!(thiscall, rw_00d57080(this: u32) -> u32 {
        #[inline(always)]
        unsafe fn write32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const CALLEE_TASK_TEARDOWN: u32 = 1;
        const CALLEE_BASE_DESTRUCTOR: u32 = 2;
        const VTABLE_FILE_VA: u32 = 0x00ee6cc4;
        unsafe { write32(this, lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_TASK_TEARDOWN, u32, this);
        lf_checker_rt::callee_thiscall!(CALLEE_BASE_DESTRUCTOR, u32, this)
});
