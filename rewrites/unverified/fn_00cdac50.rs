// original: 0x00CDAC50 task_object_cleanup_60 (proposed)

/// Install the object's vtable, call the nested handle cleanup on `this+0x60`, then tail-call the common task destructor. The zero-argument thiscall returns the tail-call result.
lf_checker_rt::export!(thiscall, rw_00cdac50(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const OBJECT_VTABLE: u32 = 0x00edd254;
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this + 0x60);
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
