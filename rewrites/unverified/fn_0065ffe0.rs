// original: 0x0065FFE0 rage::snConnectToPeerTask::vf0
/// Calls the base initializer, then calls the TLS-selected virtual cleanup method when flag bit zero is set and `this` is non-null. Returns the original `this` pointer. The method is thiscall with one stack flag word; its TLS callback context comes from slot zero.
lf_checker_rt::export!(thiscall, rw_0065ffe0(this: u32, cleanup_flags: u32) -> u32 {
    unsafe {
        const BASE_INITIALIZE_CALL: u32 = 1;
        const TLS_CALLBACK_CALL: u32 = 2;
        const TLS_SLOT: usize = 0;
        const TLS_CALLBACK_CONTEXT: u32 = 8;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_INITIALIZE_CALL, u32, this);
        if cleanup_flags & 1 != 0 && this != 0 {
            let tls_vector_entry = lf_checker_rt::tls_slot(TLS_SLOT);
            let callback_context = unsafe {
                read_word(tls_vector_entry.wrapping_add(TLS_CALLBACK_CONTEXT))
            };
            let _: u32 = lf_checker_rt::callee_thiscall!(
                TLS_CALLBACK_CALL,
                u32,
                callback_context,
                this
            );
        }
        this
    }
});
