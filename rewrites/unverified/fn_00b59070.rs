// original: 0x00b59070 CVehicleFactory::vf0

/// Deleting destructor step for a vehicle factory: run the teardown
/// routine (scripted callee 1) with `this`, then, when bit 0 of the flag
/// word is set, release the object through the allocator (scripted callee
/// 2, cdecl). Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b59070(this: u32, flag: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if flag & 1 != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
