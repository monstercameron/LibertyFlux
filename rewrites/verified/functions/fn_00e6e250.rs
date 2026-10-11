// original: 0x00E6E250 mainloop_timing_release_thread_cleanup

/// If the cleanup pointer is nonzero, calls the third method of the current
/// thread's cleanup object with that pointer. In either case, clears the pointer
/// and neighboring state word, sets the adjacent flag, and returns zero.
pub extern "cdecl" fn rw_00e6e250() -> u32 {
    const CLEANUP_ARGUMENT: u32 = 0x0110_db84;
    const CLEANUP_STATE_WORD: u32 = 0x0110_db88;
    const CLEANUP_STATE_FLAG: u32 = 0x0110_db80;

    let argument = unsafe { lf_checker_rt::global::<u32>(CLEANUP_ARGUMENT).read() };
    if argument != 0 {
        let tls_root = lf_checker_rt::tls_slot(0) as *const u32;
        let cleanup_object = unsafe { tls_root.add(2).read_unaligned() };
        let vtable = unsafe { (cleanup_object as *const u32).read_unaligned() };
        let method = unsafe { ((vtable as *const u32).add(3)).read_unaligned() };
        let call: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(method as usize)
        };
        let _ = call(cleanup_object, argument);
    }

    unsafe {
        lf_checker_rt::global::<u32>(CLEANUP_ARGUMENT).write(0);
        lf_checker_rt::global::<u32>(CLEANUP_STATE_WORD).write(0);
        lf_checker_rt::global::<u8>(CLEANUP_STATE_FLAG).write(1);
    }
    0
}
