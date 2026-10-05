// original: 0x00CC7660 euphoria_controller_dtor_chain (proposed)

/// Destructor prologue: tear down the sub-object, then tail-call the base.
///
/// Writes the primary vtable at `this` and the secondary vtable at
/// `this + 0x50`, destroys the sub-object at `this + 0x100`, then forwards
/// `this` to the base destructor and returns its result. The original ends in
/// a jump; the rewrite expresses that as a returned call.
///
/// Original: 0x00CC7660 (thiscall, no stack words, tail jump).
lf_checker_rt::export!(thiscall, rw_00cc7660(this: u32) -> u32 {
    unsafe {
        const SUB_CALLEE: u32 = 1;
        const BASE_CALLEE: u32 = 2;
        const VTABLE_MAIN: u32 = 0x00ED994C;
        const VTABLE_SUB: u32 = 0x00ED9A58;
        const SUB_VTABLE_AT: u32 = 0x50;
        const SUB_AT: u32 = 0x100;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_MAIN));
        (this.wrapping_add(SUB_VTABLE_AT) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_SUB));
        lf_checker_rt::callee_thiscall!(SUB_CALLEE, u32, this.wrapping_add(SUB_AT));
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this)
    }
});
