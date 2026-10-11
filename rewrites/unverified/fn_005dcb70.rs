// original: 0x005DCB70 CPedIntelligenceFactoryNY::vf0

/// Release the current pedestrian intelligence factory and optionally its TLS service.
///
/// The method writes the factory vtable, releases a non-null scanner through
/// the pool helper and TLS service vtable, clears the shared scanner pointer,
/// then calls the base factory cleanup. Flag bit zero triggers a second
/// service-vtable call with the factory as its argument. The method returns
/// `this`; the service method is loaded from slot three of its 32-bit vtable.
lf_checker_rt::export!(thiscall, rw_005dcb70(this: u32, flags: u32) -> u32 {
    const FACTORY_VTABLE: u32 = 0x00FE0BF4;
    const FACTORY_POINTER_SLOT: u32 = 0x018B6F0C;
    const TLS_INTERFACE_OFFSET: u32 = 8;
    const RELEASE_SLOT_OFFSET: u32 = 12;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn call_service_release(argument: u32, tls_interface_offset: u32, slot_offset: u32) -> u32 {
        let tls_root = lf_checker_rt::tls_slot(0);
        let service = unsafe { read_u32(tls_root.wrapping_add(tls_interface_offset)) };
        let vtable = unsafe { read_u32(service) };
        let method = unsafe { read_u32(vtable.wrapping_add(slot_offset)) };
        let release: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(method as usize)
        };
        release(service, argument)
    }

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FACTORY_VTABLE));
    }

    let scanner = unsafe {
        lf_checker_rt::global::<u32>(FACTORY_POINTER_SLOT).read_unaligned()
    };
    if scanner != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, scanner);
        let _ = unsafe { call_service_release(scanner, TLS_INTERFACE_OFFSET, RELEASE_SLOT_OFFSET) };
    }

    unsafe {
        lf_checker_rt::global::<u32>(FACTORY_POINTER_SLOT).write_unaligned(0);
    }
    let _ = lf_checker_rt::callee_thiscall!(3, u32, this);

    if flags & 1 != 0 {
        let _ = unsafe { call_service_release(this, TLS_INTERFACE_OFFSET, RELEASE_SLOT_OFFSET) };
    }

    this
});
