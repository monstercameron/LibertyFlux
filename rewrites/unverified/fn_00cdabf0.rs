// original: 0x00CDABF0 task_object_cleanup_4c_50 (proposed)

/// Install the object's vtable, release the optional pointer at `this+0x4c` with the field address, call the nested handle cleanup on `this+0x50`, and tail-call the common task destructor. The zero-argument thiscall returns the tail-call result.
lf_checker_rt::export!(thiscall, rw_00cdabf0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const OBJECT_VTABLE: u32 = 0x00edd614;
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        let pointer = read_word(this + 0x4c);
        if pointer != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, pointer, this + 0x4c);
        }
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this + 0x50);
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
