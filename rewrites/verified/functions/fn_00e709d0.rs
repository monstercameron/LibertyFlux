// original: 0x00E709D0 cleanup_critical_section_00e709d0

/// Passes the fixed critical-section address to the imported Windows
/// `DeleteCriticalSection` routine using the stdcall convention. This
/// wrapper has no semantic return value; the checker compares the call.
lf_checker_rt::export!(cdecl, rw_00e709d0() -> u32 {
    const CRITICAL_SECTION_VA: u32 = 0x01B4A8B0;
    let _ = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(CRITICAL_SECTION_VA));
    0
});
