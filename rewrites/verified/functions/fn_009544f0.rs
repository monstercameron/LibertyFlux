// original: 0x009544F0 bound_state_check (proposed)

/// Check an argument against a limit/state global pair.
///
/// Reads `LIMIT`; returns 0 when `arg` is above it (UNSIGNED 32-bit
/// comparison). Otherwise returns whether `LIMIT` equals the second
/// global `STATE`. Pure read, no writes. Original is cdecl/1, returns AL.
lf_checker_rt::export!(cdecl, rw_009544F0(arg: u32) -> u32 {
    const LIMIT: u32 = 0x0120CA3C;
    const STATE: u32 = 0x011F7028;
    let limit = unsafe { lf_checker_rt::global::<u32>(LIMIT).read() };
    if arg > limit {
        return 0;
    }
    let state = unsafe { lf_checker_rt::global::<u32>(STATE).read() };
    (limit == state) as u32
});
