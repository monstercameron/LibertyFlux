// original: 0x00CC7690 euphoria_handler_dtor_chain (proposed)

/// Destructor prologue: plant the base vtable, then tail-call the base destructor.
///
/// Writes the base vtable at `this`, then forwards `this` to the base
/// destructor and returns its result. The original ends in a jump; the
/// rewrite expresses that as a returned call.
///
/// Original: 0x00CC7690 (thiscall, no stack words, tail jump).
lf_checker_rt::export!(thiscall, rw_00cc7690(this: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const VTABLE: u32 = 0x00ED978C;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this)
    }
});
