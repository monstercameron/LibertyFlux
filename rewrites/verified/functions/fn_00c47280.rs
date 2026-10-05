// original: 0x00c47280 ccamscripted_ctor (proposed)
/// Construct a scripted camera: base, member at +0x140, then setup.
///
/// Calls the base constructor (callee 1) with `this`, plants the
/// `CCamScripted` vtable, constructs the member at `this + MEMBER`
/// (callee 2), then calls the sibling setup routine (callee 3).
/// Returns `this`.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c47280(this: u32) -> u32 {
    const VTABLE_CCAMSCRIPTED: u32 = 0x00ec9400;
    const MEMBER: u32 = 0x140;
    const BASE_CTOR: u32 = 1;
    const MEMBER_CTOR: u32 = 2;
    const SETUP: u32 = 3;
    unsafe {
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_CCAMSCRIPTED));
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, this + MEMBER);
        lf_checker_rt::callee_thiscall!(SETUP, u32, this);
    }
    this
});
