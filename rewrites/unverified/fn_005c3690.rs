// original: 0x005C3690 CPedFactoryNY::vf0

/// Installs the derived factory vtable, invokes its base initializer, and
/// conditionally notifies a TLS-owned service when argument bit 0 is set. The
/// service is reached through virtual slot +0x0c and receives this object as
/// its sole stack argument. The object pointer is returned in EAX.
lf_checker_rt::export!(thiscall, rw_005c3690(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00fe0ac4;
        const SERVICE_VTABLE: u32 = 0x00;
        const SERVICE_SLOT: u32 = 0x0c;
        const BASE_INITIALIZER_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn call_thiscall1(target: u32, object: u32, argument: u32) -> u32 {
            let method: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(object, argument)
        }

        write_word(this, lf_checker_rt::relocated(VTABLE_FILE_VA));
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER_CALLEE, u32, this);
        if flags & 1 != 0 {
            let tls_context = lf_checker_rt::tls_slot(0);
            let service = read_word(tls_context + 8);
            let vtable = read_word(service + SERVICE_VTABLE);
            let target = read_word(vtable + SERVICE_SLOT);
            let _ = call_thiscall1(target, service, this);
        }
        this
    }
});
