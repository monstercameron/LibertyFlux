// original: 0x00876790 crmt_request_source_replace


/// Initialize the embedded source request through its global setup hook,
/// clear request state, acquire a reference to a non-null incoming source,
/// release the previously installed source if the initialized member still
/// contains one, then store the new pointer and return `this`. The source
/// object starts at +0x04; request state is at +0x14 and +0x18. This is a
/// thiscall method with one stack pointer argument and callee cleanup.
lf_checker_rt::export!(thiscall, rw_00876790(this: u32, new_source: u32) -> u32 {
    const SUBOBJECT: u32 = 0x04;
    const SOURCE_VTABLE: u32 = 0x00fe7fc8;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const REQUEST_VTABLE: u32 = 0x00fe8204;
    const SETUP_HOOK_VA: u32 = 0x00fe7fb8;
    const ADD_REF_SLOT: u32 = 4;
    const RELEASE_SLOT: u32 = 8;

    #[inline(always)]
    unsafe fn invoke_slot(object: u32, slot: u32) -> u32 {
        unsafe {
            let vtable = (object as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(slot) as *const u32).read_unaligned();
            let method: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            method(object)
        }
    }

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

        if new_source != 0 { let _ = invoke_slot(new_source, ADD_REF_SLOT); }
        let old_source = ((this + 0x18) as *const u32).read_unaligned();
        if old_source != 0 { let _ = invoke_slot(old_source, RELEASE_SLOT); }
        ((this + 0x18) as *mut u32).write_unaligned(new_source);
    }
    this
});
