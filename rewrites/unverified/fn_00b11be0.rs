// original: 0x00B11BE0 CDynamicEntity::vf51

/// Run the dynamic entity's base update, then call its fraction update only
/// while the shared initialization byte is zero. Finish by forwarding this
/// object to the tail-called entity helper and return that helper's result.
/// All three functions are checker stand-ins. This is a no-argument thiscall
/// method.
lf_checker_rt::export!(thiscall, rw_00b11be0(this: u32) -> u32 {
    const INITIALIZED_VA: u32 = 0x0159_3312;

    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    let initialized = unsafe { (lf_checker_rt::global::<u8>(INITIALIZED_VA) as *const u8).read() };
    if initialized == 0 {
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
    }
    lf_checker_rt::callee_thiscall!(3, u32, this)
});
