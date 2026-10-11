// original: 0x00626D30 rage::snMigrateSessionTask::vf0

/// Release the base task first, then conditionally notify the current thread's TLS callback.
/// The receiver is in ECX and the one stack argument is a flags word; bit 0 enables the
/// notification only when the receiver is non-null. The callback chain is TLS slot 0, an
/// object pointer at +8, a vtable pointer at +0, and a callback pointer at vtable +0x0C.
/// The callback receives the TLS object in ECX and the original receiver as its stack word.
/// The method returns the original receiver in EAX. The checker fabricates the TLS object and
/// function table so both sides call the same scripted target.
lf_checker_rt::export!(thiscall, rw_00626d30(this: u32, flags: u32) -> u32 {
    unsafe {
        const RELEASE_BASE: u32 = 1;
        const TLS_SLOT: usize = 0;
        const TLS_OBJECT_LINK: u32 = 8;
        const VTABLE_TARGET_SLOT: u32 = 12;
        const OPTIONAL_RELEASE_FLAG: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let _ = lf_checker_rt::callee_thiscall!(RELEASE_BASE, u32, this);
        if flags & OPTIONAL_RELEASE_FLAG != 0 && this != 0 {
            let tls_object = lf_checker_rt::tls_slot(TLS_SLOT);
            let callback_object = read_u32(tls_object.wrapping_add(TLS_OBJECT_LINK));
            let callback_vtable = read_u32(callback_object);
            let callback_target = read_u32(callback_vtable.wrapping_add(VTABLE_TARGET_SLOT));
            let callback: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callback_target as usize);
            let _ = callback(callback_object, this);
        }
        this
    }
});
