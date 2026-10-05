// original: 0x00c46f90 ccamscript_ctor (proposed)
/// Construct a script camera: base object, vtable, then its own setup.
///
/// Calls the base constructor (callee 1) with `this`, plants the
/// `CCamScript` vtable, then calls the sibling setup routine (callee 2).
/// Returns `this`.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c46f90(this: u32) -> u32 {
    const VTABLE_CCAMSCRIPT: u32 = 0x00ec9274;
    const BASE_CTOR: u32 = 1;
    const SETUP: u32 = 2;
    unsafe {
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_CCAMSCRIPT));
        lf_checker_rt::callee_thiscall!(SETUP, u32, this);
    }
    this
});
