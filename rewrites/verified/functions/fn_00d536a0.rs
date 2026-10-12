// original: 0x00D536A0 CCamFree::initialize_default_state

/// Dispatches the object's virtual slot at vtable offset 0x18, calls the
/// default-state helper, then returns the helper's EAX value with its low byte
/// forced to one. Both calls use this object in ECX; the first is supplied by
/// the proof's object vtable and the second is a direct helper call.
lf_checker_rt::export!(thiscall, rw_00d536a0(this: u32) -> u32 {
    const VIRTUAL_INITIALIZER: u32 = 1;
    const DIRECT_INITIALIZER: u32 = 2;

    let _ = lf_checker_rt::callee_thiscall!(VIRTUAL_INITIALIZER, u32, this);
    let helper_result = lf_checker_rt::callee_thiscall!(DIRECT_INITIALIZER, u32, this);
    (helper_result & !0xff) | 1
});
