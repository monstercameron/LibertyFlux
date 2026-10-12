// original: 0x00B46300 CInformFriendsEvent_dtor_thunk

/// Destroy an inform-friends event: install its own virtual table, then tail-call the base event destructor and return its result.
///
/// `this` points to the event. The virtual-table word is installed before
/// the tail call so the base destructor dispatches correctly; the call
/// itself is intercepted by the contract.
lf_checker_rt::export!(thiscall, rw_00b46300(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EAD864;
    const BASE_DTOR: u32 = 1;

    unsafe {
        (this as *mut u32).write_unaligned(0x00EAD864);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
