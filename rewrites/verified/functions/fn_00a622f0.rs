// original: 0x00A622F0 cobject_scanner_deleting_destructor

/// Installs the object-scanner vtable and calls the base destructor. When the
/// low cleanup flag is set, it also forwards the object to the deleting
/// helper. The object pointer is in ECX and the cleanup flags are one stack
/// word. The function returns the original object pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00A622F0(this: u32, cleanup_flags: u32) -> u32 {
    const OBJECT_SCANNER_VTABLE: u32 = 0x00e9d818;
    const DELETE_STORAGE: u32 = 0x01;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_SCANNER_VTABLE));
    }
    lf_checker_rt::callee_thiscall!(1, u32, this);

    if cleanup_flags & DELETE_STORAGE != 0 {
        lf_checker_rt::callee_cdecl!(2, u32, this);
    }

    this
});
