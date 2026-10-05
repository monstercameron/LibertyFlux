// original: 0x009DC050 pool_foreach_callback (proposed)

/// Run a fixed callback over every item of the 0x160-stride pool.
///
/// The pool header and the callback address are both constants; the callee
/// walks the pool and invokes the callback per item. The callback's result
/// is the return value.
///
/// Original: 0x009DC050 (cdecl, no arguments, one outgoing thiscall).
lf_checker_rt::export!(cdecl, rw_009DC050() -> u32 {
    unsafe {
        const POOL_VA: u32 = 0x0103ADA0;
        const CALLBACK_VA: u32 = 0x00C743E0;
        const FOREACH: u32 = 1;

        lf_checker_rt::callee_thiscall!(
            FOREACH, u32,
            lf_checker_rt::relocated(POOL_VA),
            lf_checker_rt::relocated(CALLBACK_VA)
        )
    }
});
