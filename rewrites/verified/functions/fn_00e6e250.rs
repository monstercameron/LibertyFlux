// original: 0x00E6E250 mainloop_timing_release_thread_cleanup

/// If the global cleanup pointer is non-null, resolve the current thread's
/// cleanup object through TLS and its third vtable method, call it, then clear
/// the pointer and adjacent state word and set the byte flag.
lf_checker_rt::export!(cdecl, rw_00e6e250() -> u32 {
    let cleanup_argument = unsafe {
        lf_checker_rt::global::<u32>(THREAD_CLEANUP_ARGUMENT_VA).read()
    };
    if cleanup_argument != 0 {
        let tls_root = lf_checker_rt::tls_slot(TLS_ROOT_SLOT) as *const u32;
        let cleanup_object = unsafe { tls_root.add(TLS_OBJECT_FIELD_WORD).read_unaligned() };
        let vtable = unsafe { (cleanup_object as *const u32).read_unaligned() };
        let method = unsafe {
            ((vtable as *const u32).add(VTABLE_METHOD_WORD)).read_unaligned()
        };
        let call: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(method as usize)
        };
        let _ = call(cleanup_object, cleanup_argument);
    }

    unsafe {
        lf_checker_rt::global::<u32>(THREAD_CLEANUP_ARGUMENT_VA).write(0);
        lf_checker_rt::global::<u32>(THREAD_CLEANUP_STATE_WORD_VA).write(0);
        lf_checker_rt::global::<u8>(THREAD_CLEANUP_STATE_FLAG_VA).write(1);
    }
    0
});

const THREAD_CLEANUP_ARGUMENT_VA: u32 = 0x0110_db84;

const THREAD_CLEANUP_STATE_FLAG_VA: u32 = 0x0110_db80;

const THREAD_CLEANUP_STATE_WORD_VA: u32 = 0x0110_db88;

const TLS_OBJECT_FIELD_WORD: usize = 2;

const TLS_ROOT_SLOT: usize = 0;

const VTABLE_METHOD_WORD: usize = 3;
