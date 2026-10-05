// original: 0x00a8b950 CInteriorInst::vf0

/// Destroy the interior instance, freeing it when the flag asks.
///
/// `this` is the instance and `flags` the destroy mode. Always runs the
/// destructor callee; when bit 0 of `flags` is set, also hands the instance
/// to the pool free callee with the pool manager object. Returns `this`.
///
/// Original: 0x00A8B950 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8b950(this: u32, flags: u32) -> u32 {
    unsafe {
        const CALLEE_DTOR: u32 = 1;
        const CALLEE_FREE: u32 = 2;
        const POOL_MANAGER: u32 = 0x12fb214;
        const FREE_FLAG: u32 = 1;
        lf_checker_rt::callee_thiscall!(CALLEE_DTOR, u32, this);
        if flags & FREE_FLAG != 0 {
            let manager =
                lf_checker_rt::global::<u32>(POOL_MANAGER).read_unaligned();
            lf_checker_rt::callee_thiscall!(CALLEE_FREE, u32, manager, this);
        }
        this
    }
});
