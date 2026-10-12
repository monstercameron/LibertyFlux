// original: 0x00D535E0 CCamSkeleton::destructor

/// Restores the class vtable, invokes the task teardown helper, and forwards
/// to the base camera destructor. The final helper is returned directly by the
/// original tail jump; the contract intercepts both calls.
lf_checker_rt::export!(thiscall, rw_00d535e0(this: u32) -> u32 {
    const DETACH_AND_TEARDOWN: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 2;
    const VT_CCAM_SKELETON: u32 = 0x00ee_590c;

    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_CCAM_SKELETON)); }
    let _ = lf_checker_rt::callee_thiscall!(DETACH_AND_TEARDOWN, u32, this);
    lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
});