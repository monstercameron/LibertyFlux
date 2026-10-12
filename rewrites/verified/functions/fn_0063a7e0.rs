// original: 0x0063A7E0 rage::ptxRenderState::create

/// Request a render-state object from the registry interface in TLS slot
/// zero. A non-null object is initialized with type 2, zeroed words at `+4`,
/// `+8`, and `+0xC`, the word `0x100` at `+0x10`, and byte 1 at `+0x12`.
/// A null factory result is returned unchanged. The factory is called through
/// the registry's vtable slot 2 with stack arguments `0x14`, `0x10`, and zero.
lf_checker_rt::export!(cdecl, rw_0063A7E0() -> u32 {
    const TLS_REGISTRY_SLOT: usize = 0;
    const REGISTRY_INTERFACE: u32 = 8;
    const FACTORY: u32 = 1;
    const TYPE_FIELD: u32 = 0x00;
    const FIRST_ZERO_WORD: u32 = 0x04;
    const SECOND_ZERO_WORD: u32 = 0x08;
    const FINAL_ZERO_WORD: u32 = 0x0C;
    const CAPACITY_WORD: u32 = 0x10;
    const READY_BYTE: u32 = 0x12;

    let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
    let interface = unsafe {
        (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32).read_unaligned()
    };
    let object = lf_checker_rt::callee_thiscall!(FACTORY, u32, interface, 0x14u32, 0x10u32, 0u32);
    if object == 0 {
        return 0;
    }

    unsafe {
        (object.wrapping_add(TYPE_FIELD) as *mut u32).write_unaligned(2);
        (object.wrapping_add(FIRST_ZERO_WORD) as *mut u32).write_unaligned(0);
        (object.wrapping_add(SECOND_ZERO_WORD) as *mut u32).write_unaligned(0);
        (object.wrapping_add(CAPACITY_WORD) as *mut u16).write_unaligned(0x100);
        (object.wrapping_add(FINAL_ZERO_WORD) as *mut u32).write_unaligned(1);
        (object.wrapping_add(READY_BYTE) as *mut u8).write(1);
    }
    object
});
