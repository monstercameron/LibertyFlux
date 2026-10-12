// original: 0x008768E0 rage::crmtRequestExtrapolate::vf0


/// Clear the extrapolation start and end words and elapsed ticks, install
/// the request vtable, tear down the embedded observer twice if its member
/// pointer at +0x0c is non-null, then install the observer's final vtable.
/// If the low bit of the stack flag is set, release this request through the
/// manager at TLS slot 0. Return `this`. This is thiscall with one stack
/// flag word and callee cleanup.
lf_checker_rt::export!(thiscall, rw_008768e0(this: u32, flags: u32) -> u32 {
    const SUBOBJECT: u32 = 0x04;
    const MEMBER: u32 = 0x0c;
    const ELAPSED_TICKS: u32 = 0x14;
    const EXTRAPOLATION_START: u32 = 0x1c;
    const EXTRAPOLATION_END: u32 = 0x20;
    const REQUEST_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const FINAL_VTABLE: u32 = 0x00e86afc;
    const OBSERVER_TEARDOWN: u32 = 1;
    const MANAGER_OFFSET: u32 = 8;
    const RELEASE_SLOT: u32 = 0x0c;

    unsafe {
        ((this + EXTRAPOLATION_START) as *mut u32).write_unaligned(0);
        ((this + EXTRAPOLATION_END) as *mut u32).write_unaligned(0);
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));

        let subobject = this + SUBOBJECT;
        let member = ((this + MEMBER) as *const u32).read_unaligned();
        if member != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member, subobject);
        }
        let member_again = ((this + MEMBER) as *const u32).read_unaligned();
        (subobject as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        if member_again != 0 {
            let _ = lf_checker_rt::callee_thiscall!(OBSERVER_TEARDOWN, u32, member_again, subobject);
        }
        (subobject as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));

        if flags & 1 != 0 {
            let tls_base = lf_checker_rt::tls_slot(0);
            let manager = ((tls_base + MANAGER_OFFSET) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(RELEASE_SLOT as usize) as *const u32)
                .read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let _ = release(manager, this);
        }
    }
    this
});
