// original: 0x00668A20 ptx_event_emitter_update_registrations

/// Calls the registration cleanup helper with this pointer as its stack flag, then writes the relocated update vtable. Nonzero resources at +0x40 and +0x44 are adjusted by the sizing helper using the incoming context word. For each registration at +0x48 and +0x4c, TLS slot 1 supplies the manager. Missing managers or a -1 helper result clear that field; otherwise a nonzero registration is adjusted by the helper's wrapping delta. The function returns this.
lf_checker_rt::export!(thiscall, rw_00668A20(this: u32, helper_context: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00FE35FC;
    const RESOURCE_A: u32 = 0x40;
    const RESOURCE_B: u32 = 0x44;
    const REGISTRATION_A: u32 = 0x48;
    const REGISTRATION_B: u32 = 0x4c;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    let _ignored_cleanup_result = lf_checker_rt::callee_thiscall!(1, u32, this, this);
    unsafe { write_u32(this, lf_checker_rt::relocated(VTABLE_VA)) };
    for offset in [RESOURCE_A, RESOURCE_B] {
        let value = unsafe { read_u32(this.wrapping_add(offset)) };
        if value != 0 {
            let delta = lf_checker_rt::callee_thiscall!(2, u32, helper_context, value);
            let updated = if false { value.wrapping_sub(delta) } else { value.wrapping_add(delta) };
            unsafe { write_u32(this.wrapping_add(offset), updated) };
        }
    }
    let registration_manager = lf_checker_rt::tls_slot(1);
    for offset in [REGISTRATION_A, REGISTRATION_B] {
        let slot = this.wrapping_add(offset);
        if registration_manager == 0 {
            unsafe { write_u32(slot, 0) };
            continue;
        }
        let manager_data = unsafe { read_u32(registration_manager) };
        let status = lf_checker_rt::callee_thiscall!(3, u32, manager_data, slot);
        if status == u32::MAX {
            unsafe { write_u32(slot, 0) };
            continue;
        }
        let value = unsafe { read_u32(slot) };
        if value != 0 {
            let delta = lf_checker_rt::callee_thiscall!(2, u32, registration_manager, value);
            let updated = value.wrapping_add(delta);
            unsafe { write_u32(slot, updated) };
        }
    }
    this
});
