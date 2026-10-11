// original: 0x00CDAB80 task_object_cleanup_28_60 (proposed)

/// Install the object's vtable, release the optional pointer at `this+0x28` with the field address, call the nested handle cleanup on `this+0x60`, and tail-call the common task destructor. The zero-argument thiscall returns the tail-call result.
lf_checker_rt::export!(thiscall, rw_00cdab80(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const OBJECT_VTABLE: u32 = 0x00edd2cc;
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));
        let pointer = read_word(this + 0x28);
        if pointer != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, pointer, this + 0x28);
        }
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this + 0x60);
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
