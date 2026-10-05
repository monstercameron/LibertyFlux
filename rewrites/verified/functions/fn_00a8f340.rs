// original: 0x00A8F340 pool_readiness_gate (proposed)

/// Report whether the pool is ready after ensuring it is awake.
///
/// The count helper (with 0) and the level helper run first; a level of 0
/// or 1 returns 0 at once. When the awake byte at `this+0x73` is clear the
/// wake helper (cdecl) runs. A zero count then returns 0, otherwise the
/// probe helper (with 0) decides: its low byte non-zero means ready.
///
/// Original: thiscall, no stack words, low byte in AL. Four callees:
/// count (thiscall one arg), level (thiscall no args), wake (cdecl one
/// arg), probe (thiscall one arg).
lf_checker_rt::export!(thiscall, rw_00A8F340(this: u32) -> u32 {
    unsafe {
        const AWAKE_OFF: u32 = 0x73;
        const COUNT_HELPER: u32 = 1;
        const LEVEL_HELPER: u32 = 2;
        const WAKE: u32 = 3;
        const PROBE: u32 = 4;
        let n: u32 = lf_checker_rt::callee_thiscall!(COUNT_HELPER, u32, this, 0);
        let level: u32 = lf_checker_rt::callee_thiscall!(LEVEL_HELPER, u32, this);
        if level <= 1 {
            return 0;
        }
        if ((this + AWAKE_OFF) as *const u8).read() == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(WAKE, u32, this);
        }
        if n == 0 {
            return 0;
        }
        let t: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, this, 0);
        (t as u8 != 0) as u32
    }
});
