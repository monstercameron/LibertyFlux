// original: 0x00E70AB0 cleanup_tls_object_00e70ab0

/// Reads the object pointer at image address `0x01111030` and A nonzero object is passed to manager vtable slot 3 as a thiscall receiver plus one stack argument. The manager is reached through TLS slot 0 at offset `+8`. The pointer global is cleared after the optional call. The wrapper's EAX is not compared by this proof.
lf_checker_rt::export!(cdecl, rw_00e70ab0() -> u32 {
    const OBJECT_POINTER_VA: u32 = 0x01111030;
    const MANAGER_TLS_SLOT: usize = 0;
    const MANAGER_INTERFACE_OFFSET: u32 = 8;
    const MANAGER_VTABLE_OFFSET: u32 = 0;
    const CLEANUP_SLOT_OFFSET: u32 = 12;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let object = read_u32(lf_checker_rt::relocated(OBJECT_POINTER_VA));
        if object != 0 {
            let tls_array = lf_checker_rt::tls_slot(MANAGER_TLS_SLOT);
            let manager = read_u32(tls_array.wrapping_add(MANAGER_INTERFACE_OFFSET));
            let vtable = read_u32(manager.wrapping_add(MANAGER_VTABLE_OFFSET));
            let cleanup = read_u32(vtable.wrapping_add(CLEANUP_SLOT_OFFSET));
            let call: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(cleanup as usize);
            let _ = call(manager, object);
        }
        write_u32(lf_checker_rt::relocated(OBJECT_POINTER_VA), 0);
        0
    }
});
