// original: 0x00879300 rage::crmtFrameBufferDynamic::vf0

/// Destroy a dynamic frame buffer and its owned synchronization state.
///
/// Install the dynamic buffer vtable, call the owned-list cleanup helper,
/// call the imported critical-section deleter for the object at `+0x0c`, and
/// install the non-deleting vtable. If the low flag bit is set, release the
/// object through the manager's TLS vtable slot `+0x0c`. Return the original
/// object pointer.
///
/// Original calling convention: thiscall with one scalar-deleting flag word.
lf_checker_rt::export!(thiscall, rw_00879300(this: u32, flags: u32) -> u32 {
    const DYNAMIC_VTABLE: u32 = 0x00fe82f0;
    const FINAL_VTABLE: u32 = 0x00eaf2ec;
    const CLEANUP_HELPER: u32 = 1;
    const DELETE_CRITICAL_SECTION_SLOT: u32 = 0x00e731d0;
    const TLS_MANAGER_FIELD: u32 = 8;
    const RELEASE_SLOT: u32 = 0x0c;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(DYNAMIC_VTABLE));
        let _cleanup_result = lf_checker_rt::callee_thiscall!(CLEANUP_HELPER, u32, this);
        let section = this.wrapping_add(0x0c);
        let delete_target = lf_checker_rt::global::<u32>(DELETE_CRITICAL_SECTION_SLOT).read_unaligned();
        let delete_section: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(delete_target as usize);
        let _delete_result = delete_section(section);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FINAL_VTABLE));
        if flags & 1 != 0 {
            let thread_state = lf_checker_rt::tls_slot(0);
            let manager = (thread_state.wrapping_add(TLS_MANAGER_FIELD) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
            let _release_result = release(manager, this);
        }
    }
    this
});
