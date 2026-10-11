// original: 0x006627B0 rage::snJoinSessionTask::vf0
/// Initializes the nested task subobject at +0x798 and its embedded object at +0xF00, releases optional members at +0x7B0 and +0x18, installs final vtables, and conditionally calls the TLS callback selected by flag bit zero. Returns `this`; the method is thiscall with one stack flag word.
lf_checker_rt::export!(thiscall, rw_006627b0(this: u32, cleanup_flags: u32) -> u32 {
    unsafe {
        const SUBOBJECT_INITIALIZE_CALL: u32 = 1;
        const SUBOBJECT_RELEASE_CALL: u32 = 2;
        const TASK_RELEASE_CALL: u32 = 3;
        const TLS_CALLBACK_CALL: u32 = 4;
        const SUBOBJECT: u32 = 0x798;
        const SUBOBJECT_INIT_OFFSET: u32 = 0x768;
        const MEMBER_POINTER: u32 = 0x18;
        const TLS_SLOT: usize = 0;
        const TLS_CALLBACK_CONTEXT: u32 = 8;
        const ROOT_INITIAL_VTABLE: u32 = 0x00fe3278;
        const SUBOBJECT_INITIAL_VTABLE: u32 = 0x00fe3380;
        const FINAL_VTABLE: u32 = 0x00fe4d60;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let subobject = this.wrapping_add(SUBOBJECT);
        unsafe { write_word(this, lf_checker_rt::relocated(ROOT_INITIAL_VTABLE)) };
        unsafe { write_word(subobject, lf_checker_rt::relocated(SUBOBJECT_INITIAL_VTABLE)) };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUBOBJECT_INITIALIZE_CALL,
            u32,
            subobject.wrapping_add(SUBOBJECT_INIT_OFFSET)
        );

        let subobject_member = unsafe { read_word(subobject.wrapping_add(MEMBER_POINTER)) };
        unsafe { write_word(subobject, lf_checker_rt::relocated(FINAL_VTABLE)) };
        if subobject_member != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                SUBOBJECT_RELEASE_CALL,
                u32,
                subobject_member,
                subobject
            );
        }

        let task_member = unsafe { read_word(this.wrapping_add(MEMBER_POINTER)) };
        unsafe { write_word(this, lf_checker_rt::relocated(FINAL_VTABLE)) };
        if task_member != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                TASK_RELEASE_CALL,
                u32,
                task_member,
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
