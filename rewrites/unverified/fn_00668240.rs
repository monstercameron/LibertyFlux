// original: 0x00668240 ptx_event_emitter_release_owned_resources

/// Releases non-null resources at offsets +0x44 and +0x40 through TLS slot 0's manager vtable slot +0x0c, in that order. It then replaces the object's vtable with the relocated event-emitter table. If offset +0x0c contains a child handle, the function calls the direct child-release helper and then releases that handle through the same manager slot. The return channel is unspecified; the proof compares all field writes, calls, stack adjustment, and faults.
lf_checker_rt::export!(thiscall, rw_00668240(this: u32) -> u32 {
    const CHILD_HANDLE: u32 = 0x0c;
    const FIRST_RESOURCE: u32 = 0x40;
    const SECOND_RESOURCE: u32 = 0x44;
    const MANAGER_FIELD: u32 = 0x08;
    const RELEASE_SLOT: u32 = 0x0c;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    let tls_root = lf_checker_rt::tls_slot(0);
    let manager = unsafe { read_u32(tls_root.wrapping_add(MANAGER_FIELD)) };
    let manager_vtable = unsafe { read_u32(manager) };
    let release_target = unsafe { read_u32(manager_vtable.wrapping_add(RELEASE_SLOT)) };
    let release: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(release_target as usize) };
    for offset in [SECOND_RESOURCE, FIRST_RESOURCE] {
        let resource = unsafe { read_u32(this.wrapping_add(offset)) };
        if resource != 0 {
            let _ignored_result = release(manager, resource);
        }
    }
    unsafe { write_u32(this, lf_checker_rt::relocated(0x00FE35B4)) };
    let child = unsafe { read_u32(this.wrapping_add(CHILD_HANDLE)) };
    if child != 0 {
        let _ignored_helper_result = lf_checker_rt::callee_thiscall!(2, u32, child);
        let _ignored_result = release(manager, child);
    }
    0
});
