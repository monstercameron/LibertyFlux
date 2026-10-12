// original: 0x00879890 rage::crmtComposer::vf0

/// Delete a base composer and conditionally release its thread object.
///
/// Install the composer vtable, invoke the component release/reset helper,
/// then install the final vtable. When the low flag bit is set, read the
/// manager from TLS slot zero and call its vtable release method at slot
/// `+0x0c`, passing the manager in ECX and this composer as the stack argument.
/// Return the original pointer.
///
/// Original calling convention: thiscall with one stack flag word.
lf_checker_rt::export!(thiscall, rw_00879890(this: u32, flags: u32) -> u32 {
    const COMPOSER_VTABLE: u32 = 0x00fe8304;
    const FINAL_VTABLE: u32 = 0x00fe83bc;
    const COMPONENT_RESET: u32 = 1;
    const TLS_MANAGER_FIELD: u32 = 8;
    const RELEASE_SLOT: u32 = 0x0c;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(COMPOSER_VTABLE));
        let _reset_result = lf_checker_rt::callee_thiscall!(COMPONENT_RESET, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));
        if flags & 1 != 0 {
            let thread_state = lf_checker_rt::tls_slot(0);
            let manager = ((thread_state.wrapping_add(TLS_MANAGER_FIELD)) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable.wrapping_add(RELEASE_SLOT)) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let _release_result = release(manager, this);
        }
    }
    this
});
