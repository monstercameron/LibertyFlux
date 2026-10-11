// original: 0x00CDABB0 task_object_cleanup_54 (proposed)

/// Install the object's vtable, release the optional pointer at `this+0x54` with the field address, and tail-call the common task destructor. The zero-argument thiscall returns the tail-call result.
lf_checker_rt::export!(thiscall, rw_00cdabb0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const OBJECT_VTABLE: u32 = 0x00edd164;
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        let pointer = read_word(this + 0x54);
        if pointer != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, pointer, this + 0x54);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
