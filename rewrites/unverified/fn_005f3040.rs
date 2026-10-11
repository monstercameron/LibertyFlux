// original: 0x005f3040 rage::atAny::PlaceHolder::vf0

/// Replaces this placeholder object's vtable, then dispatches its TLS callback
/// when flag bit zero is set. The callback receives this object as its only
/// stack argument, and the method returns this pointer.
lf_checker_rt::export!(thiscall, rw_005f3040(this: u32, flags: u32) -> u32 {
    const PLACEHOLDER_VTABLE: u32 = 0x00fe_1774;
    const TLS_OBJECT: u32 = 0x08;
    const CALLBACK_SLOT: u32 = 0x0c;
    const CALLBACK_FLAG: u32 = 1;

    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(PLACEHOLDER_VTABLE)) };
    if flags & CALLBACK_FLAG != 0 {
        let tls_block = lf_checker_rt::tls_slot(0);
        let callback_object = unsafe {
            (tls_block.wrapping_add(TLS_OBJECT) as *const u32).read_unaligned()
        };
        let vtable = unsafe { (callback_object as *const u32).read_unaligned() };
        let target = unsafe {
            (vtable.wrapping_add(CALLBACK_SLOT) as *const u32).read_unaligned()
        };
        // SAFETY: the contract plants an intercepted thiscall stub in this table slot.
        let callback: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(target as usize)
        };
        let _ = callback(callback_object, this);
    }
    this
});
