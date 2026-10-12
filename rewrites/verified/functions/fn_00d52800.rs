// original: 0x00D52800 CCamAnimated::destructor

/// Restores the animated-camera vtable, invokes the task teardown helper, then
/// forwards to the base camera destructor. The original's final E9 tail jump
/// returns the base helper's EAX value.
lf_checker_rt::export!(thiscall, rw_00d52800(this: u32) -> u32 {
    const DETACH_AND_TEARDOWN: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 2;
    const VT_CCAM_ANIMATED: u32 = 0x00ee_57a4;

    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT_CCAM_ANIMATED)); }
    let _ = lf_checker_rt::callee_thiscall!(DETACH_AND_TEARDOWN, u32, this);
    lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
});