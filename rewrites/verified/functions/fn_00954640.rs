// original: 0x00954640 state_zero_or_one (proposed)

/// Report whether the mode word is in its idle states.
///
/// Reads the global dword `MODE` and returns 1 when it is 0 or 1,
/// otherwise 0. Pure read, no writes. Original is cdecl/0, returns AL.
lf_checker_rt::export!(cdecl, rw_00954640() -> u32 {
    const MODE: u32 = 0x01037720;
    let mode = unsafe { lf_checker_rt::global::<u32>(MODE).read() };
    ((mode == 0) || (mode == 1)) as u32
});
