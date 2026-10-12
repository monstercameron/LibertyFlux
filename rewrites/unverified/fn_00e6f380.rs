// original: 0x00E6F380 cleanup_tls_state_00e6f380

/// Optionally calls the manager's slot-3 cleanup method for the
/// pointer at 0x0110EEF4, then clears that pointer and the
/// dword at 0x0110EEF8. It sets byte 0x0110EEF0 to one
/// while preserving the other bytes in that dword.
lf_checker_rt::export!(cdecl, rw_00e6f380() -> u32 {
    const OBJECT_POINTER_VA: u32 = 0x0110EEF4;
    const RESULT_VA: u32 = 0x0110EEF8;
    const STATE_BYTE_VA: u32 = 0x0110EEF0;
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
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
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
        write_u32(lf_checker_rt::relocated(RESULT_VA), 0);
        write_u8(lf_checker_rt::relocated(STATE_BYTE_VA), 1);
        0
    }
});
