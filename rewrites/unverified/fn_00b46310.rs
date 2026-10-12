// original: 0x00B46310 CInformGroupEvent_dtor_thunk

/// Destroy an inform-group event: install its own virtual table, then tail-call the base event destructor and return its result.
///
/// `this` points to the event. The virtual-table word is installed before
/// the tail call so the base destructor dispatches correctly; the call
/// itself is intercepted by the contract.
lf_checker_rt::export!(thiscall, rw_00b46310(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EAD86C;
    const BASE_DTOR: u32 = 1;

    unsafe {
        (this as *mut u32).write_unaligned(0x00EAD86C);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
