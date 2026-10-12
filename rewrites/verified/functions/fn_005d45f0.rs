// original: 0x005D45F0 CBaseDC::vf0

/// Replace a base drawing context's virtual table and, when deleting flag
/// bit 0 is set, release it through the current thread's cleanup object.
/// `this` is the drawing-context object; the cleanup hook is read from TLS
/// slot 0, then called through its vtable slot 3 with `this`. The function
/// returns `this` and removes the deleting-flag word from the stack.
lf_checker_rt::export!(thiscall, rw_005d45f0(this: u32, delete_flags: u32) -> u32 {
    const BASE_DC_VTABLE_VA: u32 = 0x00E7E048;
    const CLEANUP_OBJECT_OFFSET: u32 = 8;
    const RELEASE_SLOT_OFFSET: u32 = 12;
    const DELETE_OBJECT_FLAG: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_DC_VTABLE_VA));
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
