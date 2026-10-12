// original: 0x008777C0 rage::crmtComposerOptimized::vf0

/// Delete the optimized composer object's components and optionally release it.
///
/// Install the intermediate composer vtable, invoke the component-reset
/// helper, install the optimized vtable, and invoke the helper again. Then
/// install the final vtable. If the low flag bit is set, read the thread
/// manager from TLS slot zero and call its vtable slot `+0x0c` with the manager
/// in ECX and this object as the stack argument. Return the original pointer.
///
/// Original calling convention: thiscall with one stack flag word.
lf_checker_rt::export!(thiscall, rw_008777c0(this: u32, flags: u32) -> u32 {
    const INTERMEDIATE_VTABLE: u32 = 0x00fe8234;
    const COMPOSER_VTABLE: u32 = 0x00fe8304;
    const FINAL_VTABLE: u32 = 0x00fe83bc;
    const COMPONENT_RESET: u32 = 1;
    const TLS_MANAGER_FIELD: u32 = 8;
    const RELEASE_SLOT: u32 = 0x0c;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(INTERMEDIATE_VTABLE));
        let _first_reset = lf_checker_rt::callee_thiscall!(COMPONENT_RESET, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(COMPOSER_VTABLE));
        let _second_reset = lf_checker_rt::callee_thiscall!(COMPONENT_RESET, u32, this);
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
