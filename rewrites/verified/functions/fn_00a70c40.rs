// original: 0x00a70c40 CTaskComplexPlayerIdles::vf18

/// Build hook of the player-idles task: assembles the idle task's two
/// operands through the manager and links them with the report call.
///
/// Fetches the manager three times through the anchor at `MANAGER_ANCHOR`
/// (callee 1 each time). A null first fetch yields null at once. Otherwise
/// the secondary operand comes from the build routine (callee 2, six words:
/// `0, BLEND, 0, 0, 0, 0`) when the second fetch is live, else null; the
/// primary operand comes from the fetch routine (callee 3, manager in ecx)
/// when the third fetch is live, else null. The report routine (callee 4)
/// then runs with the first manager in ecx and the words
/// `(primary, secondary, 0, 0)`, and its result is returned.
///
/// Original: thiscall, one stack word that is never read, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a70c40(_this: u32, _arg: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const BLEND: u32 = 0xbf80_0000; // -1.0f, passed through as bits
        const GET_MANAGER: u32 = 1;
        const BUILD_SECONDARY: u32 = 2;
        const FETCH_PRIMARY: u32 = 3;
        const REPORT: u32 = 4;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        let secondary: u32 = if probe == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BUILD_SECONDARY, u32, probe, 0, BLEND, 0, 0, 0, 0)
        };
        let probe2: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        let primary: u32 = if probe2 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(FETCH_PRIMARY, u32, probe2)
        };
        lf_checker_rt::callee_thiscall!(REPORT, u32, mgr, primary, secondary, 0, 0)
    }
});
