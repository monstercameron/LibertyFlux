// original: 0x009e54c0 CPlayerPed::vf16

/// Run two job calls when the ready check passes, 1 or 0 in `al`.
///
/// Calls the ready check (`thiscall` on this, no stack words); a zero
/// low byte returns 0. Otherwise reads the job word at `this + 0x21c`
/// twice and issues two cdecl calls `(this, job + 0x80, job)`, cleaning
/// all six words itself, and returns 1. Only `al` is defined on return.
/// `thiscall`, no stack words.
lf_checker_rt::export!(thiscall, rw_009e54c0(this: u32) -> u32 {
    unsafe {
        const JOB: u32 = 0x21c;
        const JOB_BIAS: u32 = 0x80;
        const READY: u32 = 1;
        const RUN_A: u32 = 2;
        const RUN_B: u32 = 3;
        let ok = lf_checker_rt::callee_thiscall!(READY, u32, this);
        if (ok & 0xff) == 0 {
            return 0;
        }
        let v = ((this + JOB) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(RUN_A, u32, this, v.wrapping_add(JOB_BIAS), v);
        let v = ((this + JOB) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(RUN_B, u32, this, v.wrapping_add(JOB_BIAS), v);
        1
    }
});
