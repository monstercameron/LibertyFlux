// original: 0x00CDAAE0 script_vm_release_optional_handle (proposed)

/// Release the optional pointer stored at `this+0x68`, passing the field address to the helper. A null field skips the call and returns zero; a non-null field returns the scripted helper result. The routine uses thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cdaae0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let pointer = read_word(this + 0x68);
        if pointer != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, pointer, this + 0x68)
        } else {
            pointer
        }
    }
});
