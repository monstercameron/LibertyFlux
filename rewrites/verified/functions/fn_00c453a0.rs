// original: 0x00c453a0 CCamFinal::~CCamFinal
/// Install the final-camera vtable, invoke its base cleanup helper, then tail
/// call the final-camera cleanup helper. Return the latter helper's EAX value.
lf_checker_rt::export!(thiscall, rw_00c453a0(this: u32) -> u32 {
    const BASE_CLEANUP: u32 = 1;
    const FINAL_CLEANUP: u32 = 2;
    const VTABLE_FILE_VA: u32 = 0x00ec8fa0;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
    }
    let _ = lf_checker_rt::callee_thiscall!(BASE_CLEANUP, u32, this);
    lf_checker_rt::callee_thiscall!(FINAL_CLEANUP, u32, this)
});
