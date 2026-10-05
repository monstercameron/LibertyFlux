// original: 0x00b97480 NativeImpl_START_KILL_FRENZY

/// Looks up a kill-frenzy target and starts it with eight parameters.
///
/// Resolves `a0` through `LOOKUP_CALLEE` (with the `KIND` tag); when that
/// succeeds, registers `a0` on the manager `OBJ` through `REG_CALLEE`. Then
/// calls `START_CALLEE` with (`a1`..`a4`, the lookup-or-register result,
/// `a5`..`a8`, 0). No value is returned.
///
/// Original: 0x00B97480 (cdecl, nine stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b97480(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    const LOOKUP_CALLEE: u32 = 1;
    const REG_CALLEE: u32 = 2;
    const START_CALLEE: u32 = 3;
    const OBJ: u32 = 0x0116BFF0;
    const KIND: u32 = 0x00EB58E8;
    let r: u32 = lf_checker_rt::callee_cdecl!(
        LOOKUP_CALLEE, u32, a0, lf_checker_rt::relocated(KIND)
    );
    let mid = if r != 0 {
        lf_checker_rt::callee_thiscall!(REG_CALLEE, u32, lf_checker_rt::relocated(OBJ), a0)
    } else {
        r
    };
    let _: u32 = lf_checker_rt::callee_cdecl!(
        START_CALLEE, u32, a1, a2, a3, a4, mid, a5, a6, a7, a8, 0
    );
    0
});
