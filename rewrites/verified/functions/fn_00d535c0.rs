// original: 0x00D535C0 CCamSkeleton::constructor

/// Runs the CCam base constructor on the object, installs the
/// CCamSkeleton vtable, and returns the same object pointer. The base call
/// takes only this in ECX; its implementation is represented by the
/// contract's scripted stand-in.
lf_checker_rt::export!(thiscall, rw_00d535c0(this: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const VT_CCAM_SKELETON: u32 = 0x00ee_590c;

    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_CCAM_SKELETON)); }
    this
});