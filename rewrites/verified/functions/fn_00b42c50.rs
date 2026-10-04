// original: 0x00b42c50 check_derived_index_under_limit
/// Report whether a derived slot index fits under the global limit.
///
/// Extracts bits 21..24 of the pointed-to flags word, adds the global base
/// and returns 1 when the global limit covers the sum, else 0.
export!(stdcall, rw_b42c50(p: u32) -> u32 {
    unsafe {
        let v = (p as *const u32).read();
        let idx = ((v >> 0x15) & 0xf).wrapping_add((global::<u32>(0x16B8FA0)).read());
        let limit = (global::<u32>(0xEEDE18)).read();
        if limit >= idx { 1 } else { 0 }
    }
});
