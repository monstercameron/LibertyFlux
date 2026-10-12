// original: 0x00D53600 CCamSkeleton::vf0

/// Calls the class destructor first. When bit zero of the deletion flag is set,
/// it then loads the pool object from the global slot and releases this object
/// through that pool. Returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_00d53600(this: u32, flags: u32) -> u32 {
    const DESTROY_OBJECT: u32 = 1;
    const RELEASE_POOL_SLOT: u32 = 2;
    const POOL_GLOBAL_VA: u32 = 0x012f_b1a0;

    let _ = lf_checker_rt::callee_thiscall!(DESTROY_OBJECT, u32, this);
    if flags & 1 != 0 {
        let pool = unsafe { lf_checker_rt::global::<u32>(POOL_GLOBAL_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_POOL_SLOT, u32, pool, this);
    }
    this
});