// original: 0x00E6EC00 network_initialize_stats_record_resources (proposed)

/// Initialise one record, then release the two records below a table top.
///
/// Calls the setup helper (thiscall, `0x019d2ee0` in ECX) once, then
/// calls the release helper (thiscall) twice with the two words just below
/// `0x019d2f10`, descending. The value left in EAX is the last call's
/// answer. Preserves ESI and EDI.
///
/// Original: 0x00E6EC00 (cdecl, no arguments; counted-down loop of two).
lf_checker_rt::export!(cdecl, rw_00E6EC00() -> u32 {
    unsafe {
        const OBJ: u32 = 0x019d2ee0;
        const TOP: u32 = 0x019d2f10;
        const SETUP_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        let _: u32 =
            lf_checker_rt::callee_thiscall!(SETUP_CALLEE, u32, lf_checker_rt::relocated(OBJ));
        let mut row = lf_checker_rt::relocated(TOP);
        row = row.wrapping_sub(4);
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, row);
        row = row.wrapping_sub(4);
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, row)
    }
});
