// original: 0x00a9f410 stream_global_lookup (proposed)

/// Look up `key` through the shared context's table.
///
/// The context pointer is read from the global at file address 0x01bb6674
/// and the table routine is called with it in ECX and stack arguments
/// (`key`, 0); its answer is the result.
///
/// Original: 0x00a9f410 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a9f410(key: u32) -> u32 {
    unsafe {
        const CONTEXT_GLOBAL: u32 = 0x01bb6674;
        const LOOKUP: u32 = 1;
        let ctx = lf_checker_rt::global::<u32>(CONTEXT_GLOBAL).read_unaligned();
        lf_checker_rt::callee_thiscall!(LOOKUP, u32, ctx, key, 0)
    }
});
