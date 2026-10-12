// original: 0x00E6ECA0 network_release_shared_resource_array (proposed)

/// Release the 10 records just below a table top, descending.
///
/// Calls the release helper (thiscall, record pointer in ECX) for each of
/// the 10 words just below `0x019d2f48`, descending one word
/// at a time. The value left in EAX is the last call's answer. Preserves
/// ESI and EDI.
///
/// Original: 0x00E6ECA0 (cdecl, no arguments; counted-down loop).
lf_checker_rt::export!(cdecl, rw_00E6ECA0() -> u32 {
    unsafe {
        const TOP: u32 = 0x019d2f48;
        const COUNT: u32 = 10;
        const RELEASE_CALLEE: u32 = 1;
        let mut row = lf_checker_rt::relocated(TOP);
        let mut last: u32 = 0;
        let mut i: u32 = 0;
        while i < COUNT {
            row = row.wrapping_sub(4);
            last = lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, row);
            i += 1;
        }
        last
    }
});
