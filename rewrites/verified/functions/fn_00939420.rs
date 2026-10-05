// original: 0x00939420 stream_float_select (proposed)

/// Select one of two streaming pace values, as a float.
///
/// When the override flag byte is nonzero, or the probe call reports
/// nonzero in its low byte, answers the fallback global; otherwise answers
/// the live one. Returns the selected dword as a float in ST0.
lf_checker_rt::export!(cdecl, rw_00939420() -> f32 {
    unsafe {
        const FLAG: u32 = 0x11609F6;
        const LIVE: u32 = 0x11735BC;
        const FALLBACK: u32 = 0x117359C;
        const PROBE: u32 = 1;
        let addr = if (lf_checker_rt::global::<u8>(FLAG).read() != 0) {
            FALLBACK
        } else {
            let t: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
            if (t & 0xFF) == 0 { LIVE } else { FALLBACK }
        };
        f32::from_bits((lf_checker_rt::relocated(addr) as *const u32).read_unaligned())
    }
});
