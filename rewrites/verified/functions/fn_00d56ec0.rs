// original: 0x00d56ec0 ccam_1stperson_dtor

/// Camera-object destructor: install this class's virtual table, run the
/// shared member teardown, then tail-jump to the shared base destructor.
///
/// `this` points to the object. Word 0 (the vtable slot) is set to the class
/// table `VTABLE` first (a file address; the original's store carries a
/// relocation entry), so virtual calls keep resolving to this class.
/// The member teardown takes `this` in ECX and returns nothing observed; the
/// base destructor is entered by tail jump with `this` in ECX, and its return
/// value becomes this function's return value.
///
/// Original: 0x00d56ec0 (thiscall, no stack arguments; ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_00d56ec0(this: u32) -> u32 {
    unsafe {
        /// Class virtual table (file address; relocated at load).
        const VTABLE: u32 = 0x00ee6714;
        /// Shared member teardown (intercepted; thiscall, no arguments).
        const MEMBER_DTOR: u32 = 1;
        /// Shared base destructor (intercepted tail call; thiscall).
        const BASE_DTOR: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, this);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
