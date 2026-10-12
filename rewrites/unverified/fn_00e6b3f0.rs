// original: 0x00E6B3F0 scenario_task_records_initialize (proposed)

/// Initialise every record of a fixed table, then register teardown at exit.
///
/// Calls the record helper (thiscall, row pointer in ECX) for each of the
/// 20 rows starting at `0x016f80d0`, stepping
/// `0x20` bytes per row, then calls the C run-time exit
/// registration (cdecl, one stack word: the teardown routine
/// `0x00e72c00`). The value left in EAX is the registration callee's
/// answer. Preserves ESI and EDI.
///
/// Original: 0x00E6B3F0 (cdecl, no arguments; counted-down loop).
lf_checker_rt::export!(cdecl, rw_00E6B3F0() -> u32 {
    unsafe {
        const FIRST: u32 = 0x016f80d0;
        const STRIDE: u32 = 0x20;
        const COUNT: u32 = 20;
        const HOOK: u32 = 0x00e72c00;
        const ROW_CALLEE: u32 = 1;
        const ATEXIT_CALLEE: u32 = 2;
        let mut row = lf_checker_rt::relocated(FIRST);
        let mut i: u32 = 0;
        while i < COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(ROW_CALLEE, u32, row);
            row = row.wrapping_add(STRIDE);
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(ATEXIT_CALLEE, u32, lf_checker_rt::relocated(HOOK))
    }
});
