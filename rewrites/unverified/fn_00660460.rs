// original: 0x00660460 rage::snAddRemoteGamerTask::vf0
/// Sets the initial and final task vtables, initializes the embedded subobject at +0x538, releases the optional member at +0x18, and conditionally calls the TLS callback selected by flag bit zero. Returns `this`; the method is thiscall with one stack flag word.
lf_checker_rt::export!(thiscall, rw_00660460(this: u32, cleanup_flags: u32) -> u32 {
    unsafe {
        const SUBOBJECT_INITIALIZE_CALL: u32 = 1;
        const RELEASE_MEMBER_CALL: u32 = 2;
        const TLS_CALLBACK_CALL: u32 = 3;
        const SUBOBJECT_OFFSET: u32 = 0x538;
        const MEMBER_POINTER: u32 = 0x18;
        const TLS_SLOT: usize = 0;
        const TLS_CALLBACK_CONTEXT: u32 = 8;
        const INITIAL_VTABLE: u32 = 0x00fe3244;
        const FINAL_VTABLE: u32 = 0x00fe4d60;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        unsafe { write_word(this, lf_checker_rt::relocated(INITIAL_VTABLE)) };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUBOBJECT_INITIALIZE_CALL,
            u32,
            this.wrapping_add(SUBOBJECT_OFFSET)
        );

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
