// original: 0x009412f0 streaming_queue_add_byte (proposed)

/// Queue the streaming probe with the given one-byte parameter.
///
/// Zero-extends the low byte of the stack argument and calls the queue
/// worker (thiscall, `this` from the global registry) with the probe
/// address and that value, returning its result.
///
/// Original: 0x009412f0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_009412f0(arg: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x012E22A4;
        const PROBE: u32 = 0x00941310;
        const CALLEE: u32 = 1;
        let this = lf_checker_rt::global::<u32>(REGISTRY).read();
        lf_checker_rt::callee_thiscall!(
            CALLEE,
            u32,
            this,
            lf_checker_rt::relocated(PROBE),
            arg & 0xFF
        )
    }
});
