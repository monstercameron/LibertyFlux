// original: 0x00E70B30 cleanup_tls_object_00e70b30

/// Reads the object pointer at image address `0x01B4AF14` and requires the 16-bit state at `0x01B4AF1A` to be nonzero. A nonzero object is passed to manager vtable slot 3 as a thiscall receiver plus one stack argument. The manager is reached through TLS slot 0 at offset `+8`. The wrapper's EAX is not compared by this proof.
lf_checker_rt::export!(cdecl, rw_00e70b30() -> u32 {
    const OBJECT_POINTER_VA: u32 = 0x01B4AF14;
    const OBJECT_STATE_VA: u32 = 0x01B4AF1A;
    const MANAGER_TLS_SLOT: usize = 0;
    const MANAGER_INTERFACE_OFFSET: u32 = 8;
    const MANAGER_VTABLE_OFFSET: u32 = 0;
    const CLEANUP_SLOT_OFFSET: u32 = 12;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    unsafe {
        let state = read_u16(lf_checker_rt::relocated(OBJECT_STATE_VA));
        if state != 0 {
            let object = read_u32(lf_checker_rt::relocated(OBJECT_POINTER_VA));
            if state != 0 && object != 0 {
                let tls_array = lf_checker_rt::tls_slot(MANAGER_TLS_SLOT);
                let manager = read_u32(tls_array.wrapping_add(MANAGER_INTERFACE_OFFSET));
                let vtable = read_u32(manager.wrapping_add(MANAGER_VTABLE_OFFSET));
                let cleanup = read_u32(vtable.wrapping_add(CLEANUP_SLOT_OFFSET));
                let call: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(cleanup as usize);
                let _ = call(manager, object);
            }
        }
        0
    }
});
