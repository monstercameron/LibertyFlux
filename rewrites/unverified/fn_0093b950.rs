// original: 0x0093B950 stream_scale_gate (proposed)

/// Latch the streaming mode the first time a nonzero request arrives.
///
/// When the mode global is already nonzero, or the request is zero, leaves
/// it alone and answers the entry EAX (always zero under this contract).
/// Otherwise probes once and latches 2 for a zero low byte, 1 for nonzero,
/// answering the latched value.
lf_checker_rt::export!(cdecl, rw_0093b950(request: u32) -> u32 {
    unsafe {
        const MODE_GLOBAL: u32 = 0x11A4EF8;
        const PROBE: u32 = 1;
        const MODE_IDLE: u32 = 2;
        const MODE_BUSY: u32 = 1;
        let mode = lf_checker_rt::global::<u32>(MODE_GLOBAL);
        if mode.read_unaligned() != 0 || request == 0 {
            0
        } else {
            let t: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
            let latched = if (t & 0xFF) == 0 { MODE_IDLE } else { MODE_BUSY };
            mode.write_unaligned(latched);
            latched
        }
    }
});
