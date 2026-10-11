// original: 0x00E6E0D0 invoke_thread_local_cleanup_00e6e0d0

/// Read the per-thread object pointer from TLS slot 0, load its child
/// service object at offset 8, and invoke the object's cleanup vtable slot with
/// ECX set to that object and stack arguments 0x00f1b54c and 0. The vtable entry
/// is represented by a scripted checker callee; the call's registers, arguments,
/// return value and stack cleanup remain compared.

lf_checker_rt::export!(cdecl, rw_invoke_thread_local_cleanup_00e6e0d0() -> u32 {
    unsafe {
        const TLS_OBJECT_CHILD_OFFSET: u32 = 0x08;
        const CLEANUP_ARGUMENT_VA: u32 = 0x00F1B54C;
        let tls_object = lf_checker_rt::tls_slot(0);
        let service_object = ((tls_object.wrapping_add(TLS_OBJECT_CHILD_OFFSET)) as *const u32)
            .read_unaligned();
        let cleanup_argument = lf_checker_rt::relocated(CLEANUP_ARGUMENT_VA);
        lf_checker_rt::callee_thiscall!(1, u32, service_object, cleanup_argument, 0u32)
    }
});
