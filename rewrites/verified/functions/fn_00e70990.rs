// original: 0x00E70990 cleanup_critical_section_00e70990

/// Passes the fixed critical-section address to the imported Windows
/// `DeleteCriticalSection` routine using the stdcall convention. This
/// wrapper has no semantic return value; the checker compares the call.
lf_checker_rt::export!(cdecl, rw_00e70990() -> u32 {
    const CRITICAL_SECTION_VA: u32 = 0x01B4A880;
    let _ = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(CRITICAL_SECTION_VA));
    0
});
