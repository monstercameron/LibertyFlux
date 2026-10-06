// original: 0x00953B90 probe_status_twice (proposed)

/// Probe an object twice and accept unless the probe is absent or saturated.
///
/// Calls the status probe (a thiscall taking no stack arguments) with
/// `this` twice. Returns 0 when the first call returns 0, or when the
/// second call returns exactly `ABSENT` (0xFFFFFF); otherwise returns 1.
/// Both comparisons are exact equalities. Original is thiscall/0,
/// returns AL.
lf_checker_rt::export!(thiscall, rw_00953B90(this: u32) -> u32 {
    const PROBE: u32 = 1;
    const ABSENT: u32 = 0xFFFFFF;
    let first = lf_checker_rt::callee_thiscall!(PROBE, u32, this);
    if first == 0 {
        return 0;
    }
    let second = lf_checker_rt::callee_thiscall!(PROBE, u32, this);
    (second != ABSENT) as u32
});
