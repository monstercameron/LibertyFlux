// original: 0x00b05d10 construct_helper
/// Constructor step: install the vtable, run one initialiser, hand off.
///
/// Takes the object pointer in ECX. Writes the vtable pointer at the
/// object's head, calls the field initialiser with the same object
/// pointer, then transfers control to the chained constructor with the
/// same object pointer, returning whatever that call answers.
export!(thiscall, rw_00b05d10(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00EAA588);
        callee_thiscall!(2, u32, this);
        callee_thiscall!(1, u32, this)
    }
});
