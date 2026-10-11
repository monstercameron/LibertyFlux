// original: 0x00876710 crmt_request_source_initialize

/// Initialize the embedded request source. Install its base and observer
/// vtables, clear the observer links, invoke the shared setup hook with the
/// embedded source at +0x04, clear request state at +0x14 and +0x18, then
/// install the concrete request vtable. The setup hook is reached through
/// the shared data slot and has no stack arguments. Return `this`.
lf_checker_rt::export!(thiscall, rw_00876710(this: u32) -> u32 {
    const SUBOBJECT: u32 = 0x04;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const SOURCE_VTABLE: u32 = 0x00fe7fc8;
    const REQUEST_VTABLE: u32 = 0x00fe8204;
    const SETUP_HOOK_VA: u32 = 0x00fe7fb8;

    unsafe {
        let source = this + SUBOBJECT;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(SOURCE_VTABLE));
        ((source + 4) as *mut u32).write_unaligned(0);
        (source as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        ((source + 8) as *mut u32).write_unaligned(0);
        ((source + 0x0c) as *mut u32).write_unaligned(0);

        let hook_target = lf_checker_rt::global::<u32>(SETUP_HOOK_VA).read_unaligned();
        let setup: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(hook_target as usize);
        let _ = setup(source);

        ((this + 0x14) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(REQUEST_VTABLE));
        ((this + 0x18) as *mut u32).write_unaligned(0);
        this
    }
});
