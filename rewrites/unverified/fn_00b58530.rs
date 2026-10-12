// original: 0x00b58530 CDummyObject::vf0

/// Deleting destructor step for a dummy object: run the chained teardown
/// (scripted callee 1) with `this`, then, when bit 0 of the flag word is
/// set, release the object through the pool found in the global at
/// 0x0166d9ec (scripted callee 2, thiscall). Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b58530(this: u32, flag: u32) -> u32 {
    unsafe {
        const POOL: u32 = 0x0166_d9ec;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if flag & 1 != 0 {
            let pool = lf_checker_rt::global::<u32>(POOL).read_unaligned();
            lf_checker_rt::callee_thiscall!(2, u32, pool, this);
        }
        this
    }
});
