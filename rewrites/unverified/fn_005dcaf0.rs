// original: 0x005DCAF0 CEventFactoryNY::~CEventFactoryNY__deleting

/// Finalize a New York event factory and optionally release its TLS object.
///
/// The deleting destructor writes the adjusted factory vtable to `this`,
/// calls the base destructor, and invokes one method from the current
/// thread's slot-zero service when flag bit zero is set. The service pointer
/// comes from the TLS root's word at offset 8; its vtable's fourth word is
/// called with the service as `this` and the factory as its stack argument.
/// The factory pointer is returned in EAX. The object and service use the
/// game's 32-bit pointer layout.
lf_checker_rt::export!(thiscall, rw_005dcaf0(this: u32, flags: u32) -> u32 {
    const FACTORY_VTABLE: u32 = 0x00FE0BE8;
    const SERVICE_OFFSET: u32 = 8;
    const VTABLE_METHOD_OFFSET: u32 = 12;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FACTORY_VTABLE));
    }
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

    if flags & 1 != 0 {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let tls_root = lf_checker_rt::tls_slot(0);
        let service = unsafe { read_u32(tls_root.wrapping_add(SERVICE_OFFSET)) };
        let service_vtable = unsafe { read_u32(service) };
        let method = unsafe {
            read_u32(service_vtable.wrapping_add(VTABLE_METHOD_OFFSET))
        };
        let release: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(method as usize)
        };
        let _ = release(service, this);
    }

    this
});
