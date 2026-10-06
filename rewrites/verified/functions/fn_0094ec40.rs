// original: 0x0094EC40 conditional_forward_pair (proposed)

/// Probe callee 1 with the shared handle; on a live low byte forward to callee 2.
///
/// Reads the shared handle word `HANDLE`. Calls callee 1 with (`a`, handle).
/// Only the LOW byte of its answer is tested (`(an instruction of the original)`): when zero the
/// full answer is returned and nothing else happens, otherwise callee 2 runs
/// with (`a`, handle, `b`) and its answer is returned. Note the width: a
/// non-zero answer with a zero low byte (e.g. 0x100) takes the early return.
///
/// Original: 0x0094EC40 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0094EC40(a: u32, b: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x10475B8;
        const PROBE: u32 = 1;
        const FORWARD: u32 = 2;
        let g = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read();
        let r1 = lf_checker_rt::callee_cdecl!(PROBE, u32, a, g);
        if (r1 & 0xFF) != 0 {
            lf_checker_rt::callee_cdecl!(FORWARD, u32, a, g, b)
        } else {
            r1
        }
    }
});
