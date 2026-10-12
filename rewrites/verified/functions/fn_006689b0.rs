// original: 0x006689B0 ptx_event_emitter_registration_cleanup

/// Installs the relocated registration-cleanup vtable. If TLS slot 1 or the handle at +0x0c is null, it clears the handle and returns this. Otherwise it calls the manager-data helper with a pointer to the handle slot; a -1 status also clears the slot. For other statuses it adds the manager's returned delta to the handle with 32-bit wrapping. A nonzero updated handle triggers the TLS slot 0 action helper. The function returns this.
lf_checker_rt::export!(thiscall, rw_006689B0(this: u32, _destructor_flags: u32) -> u32 {
    const CHILD_FIELD: u32 = 0x0c;
    const TLS_DATA_FIELD: u32 = 0x04;
    const MANAGER_OBJECT: u32 = 0x00;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    unsafe { write_u32(this, lf_checker_rt::relocated(0x00FE35B4)) };
    let child_slot = this.wrapping_add(CHILD_FIELD);
    let child_handle = unsafe { read_u32(child_slot) };
    let manager = lf_checker_rt::tls_slot(1);
    if manager == 0 || child_handle == 0 {
        unsafe { write_u32(child_slot, 0) };
        return this;
    }
    let manager_data = unsafe { read_u32(manager.wrapping_add(MANAGER_OBJECT)) };
    let status = lf_checker_rt::callee_thiscall!(1, u32, manager_data, child_slot);
    if status == u32::MAX {
        unsafe { write_u32(child_slot, 0) };
        return this;
    }
    let delta = lf_checker_rt::callee_thiscall!(2, u32, manager, child_handle);
    let updated_handle = child_handle.wrapping_add(delta);
    unsafe { write_u32(child_slot, updated_handle) };
    if updated_handle != 0 {
        let tls_root = lf_checker_rt::tls_slot(0);
        let action = unsafe { read_u32(tls_root.wrapping_add(TLS_DATA_FIELD)) };
        let _ignored_result = lf_checker_rt::callee_stdcall!(3, u32, action);
    }
    this
});
