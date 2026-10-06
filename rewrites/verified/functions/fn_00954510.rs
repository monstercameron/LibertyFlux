// original: 0x00954510 bound_delta_store (proposed)

/// Commit a guarded delta between two globals.
///
/// Reads `BOUND`; returns 0 when it is above `arg` (UNSIGNED 32-bit
/// comparison). Returns 0 when `BOUND` differs from `EXPECT`. Otherwise
/// stores `EXPECT - SUB` (wrapping) into `OUT` and returns 1.
/// Original is cdecl/1, returns AL.
lf_checker_rt::export!(cdecl, rw_00954510(arg: u32) -> u32 {
    const BOUND: u32 = 0x0120CA5C;
    const EXPECT: u32 = 0x011F702C;
    const SUB: u32 = 0x011F7028;
    const OUT: u32 = 0x011F7030;
    let bound = unsafe { lf_checker_rt::global::<u32>(BOUND).read() };
    if bound > arg {
        return 0;
    }
    let expect = unsafe { lf_checker_rt::global::<u32>(EXPECT).read() };
    if bound != expect {
        return 0;
    }
    let sub = unsafe { lf_checker_rt::global::<u32>(SUB).read() };
    unsafe { lf_checker_rt::global::<u32>(OUT).write(expect.wrapping_sub(sub)) };
    1
});
