// original: 0x00CC7640 euphoria_widget_dtor_chain (proposed)

/// Destructor prologue: plant the base vtables, then tail-call the base destructor.
///
/// Writes the primary vtable at `this` and the secondary vtable at
/// `this + 0x50`, then forwards `this` to the base destructor and returns
/// its result. The original ends in a jump, so no return address of its own
/// is left on the stack; the rewrite expresses that as a call whose value is
/// returned.
///
/// Original: 0x00CC7640 (thiscall, no stack words, tail jump).
lf_checker_rt::export!(thiscall, rw_00cc7640(this: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const VTABLE_MAIN: u32 = 0x00ED9814;
        const VTABLE_SUB: u32 = 0x00ED991C;
        const SUB_VTABLE_AT: u32 = 0x50;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_MAIN));
        (this.wrapping_add(SUB_VTABLE_AT) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_SUB));
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this)
    }
});
