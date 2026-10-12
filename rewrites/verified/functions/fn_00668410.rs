// original: 0x00668410 ptx_event_effect_destroy

/// Calls the child object's vtable slot +0 with argument 1 when the pointer at +0x9c is non-null. It releases the resource at +0x90 through the TLS slot 0 manager, writes the relocated destructor vtable at +0, then sends a non-null child handle at +0x0c through the direct helper and manager. The return channel is unspecified; the proof observes the write, calls, stack adjustment, and faults.
lf_checker_rt::export!(thiscall, rw_00668410(this: u32) -> u32 {
    const CHILD_OBJECT: u32 = 0x9c;
    const RESOURCE: u32 = 0x90;
    const CHILD_HANDLE: u32 = 0x0c;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    let tls_root = lf_checker_rt::tls_slot(0);
    let manager = unsafe { read_u32(tls_root.wrapping_add(0x08)) };
    let manager_vtable = unsafe { read_u32(manager) };
    let release_target = unsafe { read_u32(manager_vtable.wrapping_add(0x0c)) };
    let release: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(release_target as usize) };
    let child_object = unsafe { read_u32(this.wrapping_add(CHILD_OBJECT)) };
    if child_object != 0 {
        let child_vtable = unsafe { read_u32(child_object) };
        let child_target = unsafe { read_u32(child_vtable) };
        let child_release: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(child_target as usize) };
        let _ignored_result = child_release(child_object, 1);
    }
    let resource = unsafe { read_u32(this.wrapping_add(RESOURCE)) };
    if resource != 0 { let _ignored_result = release(manager, resource); }
    unsafe { write_u32(this, lf_checker_rt::relocated(0x00FE35B4)) };
    let child_handle = unsafe { read_u32(this.wrapping_add(CHILD_HANDLE)) };
    if child_handle != 0 {
        let _ignored_helper_result = lf_checker_rt::callee_thiscall!(3, u32, child_handle);
        let _ignored_result = release(manager, child_handle);
    }
    0
});
