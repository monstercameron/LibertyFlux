// original: 0x009d0f60 get_or_initialize_timing_singleton

/// If the low guard bit is clear, set it, call the initializer, and register the cleanup callback. Then return the singleton pointer stored in shared state.
lf_checker_rt::export!(cdecl, rw_009d0f60() -> eax {
    const GUARD_VA: u32 = 0x012958A4;
    const SINGLETON_VA: u32 = 0x01295888;
    const INITIALIZER_ID: u32 = 1;
    const REGISTER_EXIT_ID: u32 = 2;
    const EXIT_CALLBACK_VA: u32 = 0x00E71FF0;
    unsafe {
        let flags = lf_checker_rt::global::<u32>(GUARD_VA).read_unaligned();
        if flags & 1 == 0 {
            lf_checker_rt::global::<u32>(GUARD_VA).write_unaligned(flags | 1);
            let _ = lf_checker_rt::callee_cdecl!(INITIALIZER_ID, u32);
            let _ = lf_checker_rt::callee_cdecl!(REGISTER_EXIT_ID, u32, lf_checker_rt::relocated(EXIT_CALLBACK_VA));
        }
        lf_checker_rt::global::<u32>(SINGLETON_VA).read_unaligned()
    }
});
