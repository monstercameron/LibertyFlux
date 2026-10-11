// original: 0x00668FE0 ptx_event_scheduler_destroy

/// Calls the registration cleanup helper, writes the relocated scheduler vtable, and adjusts the resource at +0x90 with the incoming context word. TLS slot 1 supplies the registration manager for fields +0x94 and +0x9c. A missing manager or -1 registration status clears the corresponding field; nonzero values receive the helper's wrapping delta. A successful update of +0x9c also triggers the fastcall notification with the updated value and TLS slot 0's action word. The function returns this.
lf_checker_rt::export!(thiscall, rw_00668FE0(this: u32, helper_context: u32) -> u32 {
    const RESOURCE: u32 = 0x90;
    const FIRST_REGISTRATION: u32 = 0x94;
    const SECOND_REGISTRATION: u32 = 0x9c;
    const TLS_ACTION: u32 = 0x04;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    let _ignored_cleanup_result = lf_checker_rt::callee_thiscall!(1, u32, this, this);
    unsafe { write_u32(this, lf_checker_rt::relocated(0x00FE356C)) };
    let resource_slot = this.wrapping_add(RESOURCE);
    let resource = unsafe { read_u32(resource_slot) };
    if resource != 0 {
        let delta = lf_checker_rt::callee_thiscall!(2, u32, helper_context, resource);
        unsafe { write_u32(resource_slot, resource.wrapping_add(delta)) };
    }
    let manager = lf_checker_rt::tls_slot(1);
    let manager_data = if manager == 0 { 0 } else { unsafe { read_u32(manager) } };
    let first_slot = this.wrapping_add(FIRST_REGISTRATION);
    if manager == 0 {
        unsafe { write_u32(first_slot, 0) };
    } else {
        let status = lf_checker_rt::callee_thiscall!(3, u32, manager_data, first_slot);
        if status == u32::MAX {
            unsafe { write_u32(first_slot, 0) };
        } else {
            let value = unsafe { read_u32(first_slot) };
            if value != 0 {
                let delta = lf_checker_rt::callee_thiscall!(2, u32, manager, value);
                unsafe { write_u32(first_slot, value.wrapping_add(delta)) };
            }
        }
    }
    let second_slot = this.wrapping_add(SECOND_REGISTRATION);
    let second_value = unsafe { read_u32(second_slot) };
    if manager == 0 || second_value == 0 {
        unsafe { write_u32(second_slot, 0) };
    } else {
        let status = lf_checker_rt::callee_thiscall!(3, u32, manager_data, second_slot);
        if status == u32::MAX {
            unsafe { write_u32(second_slot, 0) };
        } else {
            let delta = lf_checker_rt::callee_thiscall!(2, u32, manager, second_value);
            let updated = second_value.wrapping_add(delta);
            unsafe { write_u32(second_slot, updated) };
            let tls_root = lf_checker_rt::tls_slot(0);
            let action = unsafe { read_u32(tls_root.wrapping_add(TLS_ACTION)) };
            let _ignored_result = lf_checker_rt::callee_fastcall!(4, u32, updated, action);
        }
    }
    this
});
