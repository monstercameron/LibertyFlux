// original: 0x0059D330 streaming_registry_lazy_init

/// Initialize the streaming registry on the first call and reload text state.
///
/// The no-argument cdecl routine sets the device-registration flag to `0x40`
/// and registers file paths. If the relocated registry global is null, it reads
/// TLS slot zero, follows the thread-state manager at `+8`, and calls that
/// manager's vtable slot `+8` to allocate a `0x718`-byte registry object with
/// alignment `0x10` and flags zero. A non-null allocation is initialized and
/// stored in the registry global; a null allocation leaves that global zero.
/// Finally, the text-system context receives one reload request and the method
/// returns true in `AL`. A pre-existing non-null registry skips the TLS path.
lf_checker_rt::export!(cdecl, rw_0059d330() -> u8 {
    const DEVICE_REGISTRATION_FLAG: u32 = 0x0105B354;
    const REGISTRY_GLOBAL: u32 = 0x01BB5628;
    const TEXT_SYSTEM_CONTEXT: u32 = 0x0116BFF0;
    const TLS_SLOT: usize = 0;
    const MANAGER_POINTER: u32 = 0x08;
    const ALLOCATOR_VTABLE: u32 = 0x00;
    const ALLOCATE_SLOT: u32 = 0x08;
    const REGISTRY_SIZE: u32 = 0x718;
    const REGISTRY_ALIGNMENT: u32 = 0x10;
    const ALLOCATION_FLAGS: u32 = 0;
    const RELOAD_REQUEST: u32 = 1;
    const REGISTRATION_VALUE: u32 = 0x40;
    const TRUE: u8 = 1;

    unsafe {
        lf_checker_rt::global::<u32>(DEVICE_REGISTRATION_FLAG)
            .write_unaligned(REGISTRATION_VALUE);
        let _ = lf_checker_rt::callee_cdecl!(1, u32);

        let registry_slot = lf_checker_rt::global::<u32>(REGISTRY_GLOBAL);
        if registry_slot.read_unaligned() == 0 {
            let thread_state = lf_checker_rt::tls_slot(TLS_SLOT);
            let manager = (thread_state.wrapping_add(MANAGER_POINTER) as *const u32)
                .read_unaligned();
            let allocator_vtable = (manager.wrapping_add(ALLOCATOR_VTABLE) as *const u32)
                .read_unaligned();
            let allocate_method = (allocator_vtable.wrapping_add(ALLOCATE_SLOT) as *const u32)
                .read_unaligned();
            let _ = core::hint::black_box(allocate_method);

            let registry = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                manager,
                REGISTRY_SIZE,
                REGISTRY_ALIGNMENT,
                ALLOCATION_FLAGS
            );
            if registry != 0 {
                let _ = lf_checker_rt::callee_thiscall!(3, u32, registry);
                registry_slot.write_unaligned(registry);
            } else {
                registry_slot.write_unaligned(0);
            }
        }

        let text_context = lf_checker_rt::relocated(TEXT_SYSTEM_CONTEXT);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, text_context, RELOAD_REQUEST);
        TRUE
    }
});
