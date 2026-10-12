// original: 0x005C0AF0 CViewportPrimaryOrtho::vf0

/// Replace the viewport's virtual table, run the base viewport teardown, and
/// optionally release the object through the current thread's cleanup hook.
/// `this` is the viewport object; `delete_flags` is the deleting-destructor
/// flag word, where bit 0 requests the cleanup-hook call. The hook is found
/// through TLS slot 0, at offset 8 from the slot's environment pointer, and
/// invoked through vtable slot 3 with the viewport as its sole stack argument.
/// The function returns the original `this` pointer and pops one stack word.
lf_checker_rt::export!(thiscall, rw_005c0af0(this: u32, delete_flags: u32) -> u32 {
    const VIEWPORT_VTABLE: u32 = 0x00FE0A08;
    const CLEANUP_OBJECT_OFFSET: u32 = 8;
    const RELEASE_SLOT_OFFSET: u32 = 12;
    const DELETE_OBJECT_FLAG: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VIEWPORT_VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

        if delete_flags & DELETE_OBJECT_FLAG != 0 {
            let environment = lf_checker_rt::tls_slot(0);
            let cleanup_object = read_u32(environment + CLEANUP_OBJECT_OFFSET);
            let vtable = read_u32(cleanup_object);
            let release = read_u32(vtable + RELEASE_SLOT_OFFSET);
            let release_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(release as usize);
            let _ = release_fn(cleanup_object, this);
        }

        this
    }
});
