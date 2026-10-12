// original: 0x00E70AE0 timing_manager_cleanup_single

/// Reads one object pointer from image data and, when it is nonzero,
/// calls slot 3 of the manager vtable with the manager in ECX and the
/// object as the one stack argument. The callee's EAX is returned; a
/// null object leaves the loaded zero in EAX.
lf_checker_rt::export!(cdecl, rw_00e70ae0() -> u32 {
    const OBJECT_POINTER_VA: u32 = 0x01C9A3E8;
    const MANAGER_TLS_SLOT: usize = 0;
    const MANAGER_INTERFACE_OFFSET: u32 = 8;
    const MANAGER_VTABLE_OFFSET: u32 = 0;
    const CLEANUP_SLOT_OFFSET: u32 = 12;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let object = read_u32(lf_checker_rt::relocated(OBJECT_POINTER_VA));
        if object == 0 {
            0
        } else {
            let tls_array = lf_checker_rt::tls_slot(MANAGER_TLS_SLOT);
            let manager = read_u32(tls_array.wrapping_add(MANAGER_INTERFACE_OFFSET));
            let vtable = read_u32(manager.wrapping_add(MANAGER_VTABLE_OFFSET));
            let cleanup = read_u32(vtable.wrapping_add(CLEANUP_SLOT_OFFSET));
            let call: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(cleanup as usize);
            call(manager, object)
        }
    }
});
