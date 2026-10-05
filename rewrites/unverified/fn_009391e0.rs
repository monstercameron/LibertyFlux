// original: 0x009391E0 stream_capped_count (proposed)

/// Cap a counted value at the streaming limit.
///
/// Calls the counter, reads the limit from its global, and returns the
/// smaller of the two, compared as unsigned 32-bit integers.
lf_checker_rt::export!(cdecl, rw_009391e0() -> u32 {
    unsafe {
        const LIMIT_GLOBAL: u32 = 0x11A4EE8;
        const COUNTER: u32 = 1;
        let count: u32 = lf_checker_rt::callee_cdecl!(COUNTER, u32,);
        let limit = lf_checker_rt::global::<u32>(LIMIT_GLOBAL).read_unaligned();
        if limit < count { limit } else { count }
    }
});
