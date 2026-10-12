// original: 0x00D97800 CPedFormation::~CPedFormation

/// Marks the object as a `CPedFormation`, optionally sends its teardown
/// message, and returns the object pointer. The boolean input is tested only
/// in bit zero; when set, the message helper receives the object as one cdecl
/// stack argument. The vtable pointer is based on the worker's relocated image
/// so it follows the original's relocation at any load base.
lf_checker_rt::export!(thiscall, rw_00d97800(this: u32, flags: u32) -> u32 {
    const VTABLE: u32 = 0x00ee_e284;
    const SEND_MESSAGE: u32 = 1;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    if flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(SEND_MESSAGE, u32, this);
    }
    this
});
