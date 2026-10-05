// original: 0x00941920 streaming_queue_add_default (proposed)

/// Queue the default streaming probe with a zero parameter.
///
/// Calls the queue worker (thiscall, `this` from the global registry) with
/// the default probe address and a zero argument, and returns its result.
///
/// Original: 0x00941920 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00941920() -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x012E22A4;
        const PROBE: u32 = 0x009414E0;
        const CALLEE: u32 = 1;
        let this = lf_checker_rt::global::<u32>(REGISTRY).read();
        lf_checker_rt::callee_thiscall!(
            CALLEE,
            u32,
            this,
            lf_checker_rt::relocated(PROBE),
            0u32
        )
    }
});
