// original: 0x0063A840 rage::ptxTriggerEvent::create

/// Ask the registry interface in TLS slot zero for a trigger-event object
/// using stack arguments `0x70`, `0x10`, and zero. A null factory result
/// returns zero; otherwise the object is passed in ECX to the tail-called
/// initializer, whose result is returned. The factory is its vtable slot 2.
lf_checker_rt::export!(cdecl, rw_0063A840() -> u32 {
    const TLS_REGISTRY_SLOT: usize = 0;
    const REGISTRY_INTERFACE: u32 = 8;
    const FACTORY: u32 = 1;
    const INITIALIZE_EVENT: u32 = 2;

    let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
    let interface = unsafe {
        (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32).read_unaligned()
    };
    let object = lf_checker_rt::callee_thiscall!(FACTORY, u32, interface, 0x70u32, 0x10u32, 0u32);
    if object == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(INITIALIZE_EVENT, u32, object)
});
