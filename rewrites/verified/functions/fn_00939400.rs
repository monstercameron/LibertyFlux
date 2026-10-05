// original: 0x00939400 stream_ptr_select (proposed)

/// Select one of two streaming pointers.
///
/// When the override flag byte is nonzero, or the probe call reports
/// nonzero in its low byte, answers the fallback global; otherwise answers
/// the live one.
lf_checker_rt::export!(cdecl, rw_00939400() -> u32 {
    unsafe {
        const FLAG: u32 = 0x11609F6;
        const LIVE: u32 = 0x11735B4;
        const FALLBACK: u32 = 0x1173594;
        const PROBE: u32 = 1;
        if (lf_checker_rt::global::<u8>(FLAG).read() != 0) {
            return (lf_checker_rt::relocated(FALLBACK) as *const u32).read_unaligned();
        }
        let t: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
        let addr = if (t & 0xFF) == 0 { LIVE } else { FALLBACK };
        (lf_checker_rt::relocated(addr) as *const u32).read_unaligned()
    }
});
