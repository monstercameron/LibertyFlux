// original: 0x00a1eb00 cam_dual_probe_nonzero (proposed)

/// Probes an object twice and reports whether either probe is nonzero.
///
/// A null `a0` returns 0. Otherwise the first probe runs a getter callee
/// on `a0` and feeds its answer through a converter callee; when that
/// result is nonzero the function returns 1. When it is zero, a second
/// getter/converter pair runs and the function returns whether its result
/// is nonzero. The original converts each integer result to float and
/// compares against 0.0, which for integer sources is exactly a
/// not-equal-zero test, so the rewrite compares the integers directly.
/// Only `al` carries the result.
///
/// Original: 0x00a1eb00 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a1eb00(a0: u32) -> u32 {
    unsafe {
        const GET_A: u32 = 1;
        const CONVERT: u32 = 2;
        const GET_B: u32 = 3;
        if a0 == 0 {
            return 0;
        }
        let t1 = lf_checker_rt::callee_thiscall!(GET_A, u32, a0);
        let v1 = lf_checker_rt::callee_cdecl!(CONVERT, u32, t1);
        if v1 != 0 {
            return 1;
        }
        let t2 = lf_checker_rt::callee_thiscall!(GET_B, u32, a0);
        let v2 = lf_checker_rt::callee_cdecl!(CONVERT, u32, t2);
        u32::from(v2 != 0)
    }
});
