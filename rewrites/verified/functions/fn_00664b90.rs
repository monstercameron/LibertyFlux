// original: 0x00664B90 rage::snSendInvitesTask::vf0
/// Sets the task vtable, dispatches a non-null request at +0x98 through the service object at +0x60, releases the optional member at +0x18, and conditionally calls the TLS callback selected by flag bit zero. Returns `this`; the method is thiscall with one stack flag word.
lf_checker_rt::export!(thiscall, rw_00664b90(this: u32, cleanup_flags: u32) -> u32 {
    unsafe {
        const REQUEST_CALLBACK_CALL: u32 = 1;
        const RELEASE_MEMBER_CALL: u32 = 2;
        const TLS_CALLBACK_CALL: u32 = 3;
        const SERVICE_POINTER: u32 = 0x60;
        const MEMBER_POINTER: u32 = 0x18;
        const REQUEST_POINTER: u32 = 0x98;
        const TLS_SLOT: usize = 0;
        const TLS_CALLBACK_CONTEXT: u32 = 8;
        const INITIAL_VTABLE: u32 = 0x00fe33e0;
        const FINAL_VTABLE: u32 = 0x00fe4d60;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let request = unsafe { read_word(this.wrapping_add(REQUEST_POINTER)) };
        unsafe { write_word(this, lf_checker_rt::relocated(INITIAL_VTABLE)) };
        if request != 0 {
            let service = unsafe { read_word(this.wrapping_add(SERVICE_POINTER)) };
            let callback_context = unsafe { read_word(service) };
            let _: u32 = lf_checker_rt::callee_thiscall!(
                REQUEST_CALLBACK_CALL,
                u32,
                callback_context,
                request
            );
        }

        let member = unsafe { read_word(this.wrapping_add(MEMBER_POINTER)) };
        unsafe { write_word(this, lf_checker_rt::relocated(FINAL_VTABLE)) };
        if member != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                RELEASE_MEMBER_CALL,
                u32,
                member,
                this
            );
        }

        if cleanup_flags & 1 != 0 {
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
