// original: 0x00cca4c0 vtable_init_with_tail_call
/// Constructor step: install the vtable, push a constant, hand off twice.
///
/// Takes the object pointer in ECX. Writes the vtable pointer at the
/// object's head, calls the field initialiser with the same object
/// pointer and one constant word, then transfers control to the chained
/// constructor with the same object pointer, returning whatever that
/// call answers.
export!(thiscall, rw_00cca4c0(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00ED9E1C);
        callee_thiscall!(2, u32, this, 0xC1000000);
        callee_thiscall!(1, u32, this)
    }
});
