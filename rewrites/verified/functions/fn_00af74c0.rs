// original: 0x00AF74C0 veh_handle_reset (proposed)

/// Reset a handle slot and re-initialise the object in place.
///
/// Clears the dword at `this`, runs the full initialiser (callee 1) on the
/// same object, and returns `this`.
///
/// Original: 0x00AF74C0 (thiscall, no stack arguments, object in EAX).
lf_checker_rt::export!(thiscall, rw_00AF74C0(this: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        (this as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(INIT, u32, this);
        this
    }
});
