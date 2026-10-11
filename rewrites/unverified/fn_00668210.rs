// original: 0x00668210 rage::ptxEventEmitter::vf0

/// Calls the class cleanup helper with this object first. If bit 0 of the incoming flags word is set, the function releases this through the manager in TLS slot 0, using the manager's vtable slot +0x0c. It returns the original this pointer. The cleanup helper is intercepted as an outgoing call; this contract checks its call and the wrapper's optional manager release.
lf_checker_rt::export!(thiscall, rw_00668210(this: u32, flags: u32) -> u32 {
    const TLS_MANAGER_FIELD: u32 = 0x08;
    const RELEASE_SLOT: u32 = 0x0c;
    let _ignored_cleanup_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    if (flags & 1 != 0) && this != 0 {
        let tls_root = lf_checker_rt::tls_slot(0);
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let manager = unsafe { read_u32(tls_root.wrapping_add(TLS_MANAGER_FIELD)) };
        let vtable = unsafe { read_u32(manager) };
        let target = unsafe { read_u32(vtable.wrapping_add(RELEASE_SLOT)) };
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let _ignored_result = release(manager, this);
    }
    this
});
