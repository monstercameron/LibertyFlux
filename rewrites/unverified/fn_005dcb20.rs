// original: 0x005DCB20 initialize_ped_intelligence_factory

/// Initialize a pedestrian intelligence factory and its event scanner.
///
/// The constructor initializes its vtable, obtains the current allocator
/// interface from TLS slot zero, and requests a 0x1c-byte block with a
/// 0x10-byte alignment and zero flags. A null allocation clears the shared
/// factory pointer. Otherwise, the same three-word argument block is reused
/// to construct the factory's scanner, whose returned pointer is stored in
/// that global. The factory pointer is returned in EAX. Both interfaces use
/// the game's 32-bit object and vtable layout.
lf_checker_rt::export!(thiscall, rw_005dcb20(this: u32) -> u32 {
    const FACTORY_VTABLE: u32 = 0x00FE0BF4;
    const TLS_INTERFACE_OFFSET: u32 = 8;
    const ALLOCATOR_SLOT_OFFSET: u32 = 8;
    const FACTORY_POINTER_SLOT: u32 = 0x018B6F0C;
    const FACTORY_SIZE: u32 = 0x1C;
    const FACTORY_ALIGNMENT: u32 = 0x10;
    const ALLOCATION_FLAGS: u32 = 0;

    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FACTORY_VTABLE));
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { read_u32(tls_root.wrapping_add(TLS_INTERFACE_OFFSET)) };
    let allocator_vtable = unsafe { read_u32(allocator_interface) };
    let allocate_address = unsafe {
        read_u32(allocator_vtable.wrapping_add(ALLOCATOR_SLOT_OFFSET))
    };
    let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
        core::mem::transmute(allocate_address as usize)
    };
    let allocation = allocate(
        allocator_interface,
        FACTORY_SIZE,
        FACTORY_ALIGNMENT,
        ALLOCATION_FLAGS,
    );

    let factory = if allocation == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(
            3,
            u32,
            allocation,
            FACTORY_SIZE,
            FACTORY_ALIGNMENT,
            ALLOCATION_FLAGS,
        )
    };
    unsafe {
        lf_checker_rt::global::<u32>(FACTORY_POINTER_SLOT).write_unaligned(factory);
    }

    this
});
