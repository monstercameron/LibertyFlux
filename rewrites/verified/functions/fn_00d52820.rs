// original: 0x00D52820 CCamAnimated::vf0

/// Calls the animated-camera base teardown, conditionally releases the object
/// through the pool pointer stored in the global pool slot when flag bit zero
/// is set, and returns the original object pointer. The release call receives
/// the pool in ECX and the object as its single stack argument.
lf_checker_rt::export!(thiscall, rw_00d52820(this: u32, flags: u32) -> u32 {
    const BASE_TEARDOWN: u32 = 1;
    const POOL_RELEASE: u32 = 2;
    const POOL_GLOBAL_VA: u32 = 0x012f_b1a0;

    let _ = lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
    if flags & 1 != 0 {
        let pool = unsafe { lf_checker_rt::global::<u32>(POOL_GLOBAL_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(POOL_RELEASE, u32, pool, this);
    }
    this
});
