// original: 0x00cfde10 task_cover_probe_and_begin (proposed)

/// Probes a cover slot and, when the probe succeeds, begins the cover task.
///
/// `obj` (third word) points to the task-side record. The probe (callee 1,
/// thiscall, one stack arg = constant 0x779) runs with ECX = the dword at
/// `obj + 0x224` plus 0x44. A zero probe result returns 0 (the low byte is
/// cleared while the upper bits, zero here, are kept). Otherwise the begin
/// function (callee 2, cdecl, four words: probe result, first arg, second
/// arg, `obj`) runs and the function returns its result with the low byte
/// forced to 1, upper 24 bits preserved.
///
/// Original: 0x00cfde10 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00cfde10(a0: u32, a1: u32, obj: u32) -> u32 {
    unsafe {
        const INNER_SLOT: u32 = 0x224;
        const INNER_ADJ: u32 = 0x44;
        const PROBE_ARG: u32 = 0x779;
        const PROBE_CALLEE: u32 = 1;
        const BEGIN_CALLEE: u32 = 2;
        let inner = (obj.wrapping_add(INNER_SLOT) as *const u32)
            .read_unaligned()
            .wrapping_add(INNER_ADJ);
        let probe: u32 = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, inner, PROBE_ARG);
        if probe == 0 {
            return probe & 0xFFFF_FF00;
        }
        let begun: u32 = lf_checker_rt::callee_cdecl!(BEGIN_CALLEE, u32, probe, a0, a1, obj);
        (begun & 0xFFFF_FF00) | 1
    }
});
