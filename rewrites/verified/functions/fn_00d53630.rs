// original: 0x00D53630 CCamFree::constructor

/// Runs the base camera constructor, installs the free-camera vtable, invokes
/// its default-state initializer, then returns the object pointer. Both helper
/// calls use thiscall with the object in ECX and are intercepted by the
/// proof contract.
lf_checker_rt::export!(thiscall, rw_00d53630(this: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const INITIALIZE_DEFAULT_STATE: u32 = 2;
    const VT_CCAM_FREE: u32 = 0x00ee_5ab0;

    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_CCAM_FREE)); }
    let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_DEFAULT_STATE, u32, this);
    this
});