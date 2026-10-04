// original: 0x00d29040 CPedTargetting::vf11 (symbols)

/// Test whether a ped's current target equals the queried one.
///
/// Reads the ped record `arg0`: when its byte at `+0x210` is set, bails out
/// unless the state word at `+0xa74` is neither 1 nor 2; then requires the
/// pointer at `+0x224` to be nonzero. Otherwise queries the target helper
/// (intercepted) with that pointer plus `0x2e0` and returns whether its
/// answer equals `arg1`. Any bail-out returns 0. Only the low byte of the
/// result is defined.
///
/// Original: 0x00D29040 (thiscall, two stack arguments; `this` unused).
lf_checker_rt::export!(thiscall, rw_00d29040(_this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x210;
        const STATE_OFF: u32 = 0xa74;
        const TARGET_OFF: u32 = 0x224;
        const HELPER_BIAS: u32 = 0x2e0;
        if unsafe { ((arg0 + FLAG_OFF) as *const u8).read() } != 0 {
            let state = unsafe { ((arg0 + STATE_OFF) as *const u32).read_unaligned() };
            if state == 1 || state == 2 {
                return 0;
            }
        }
        let target = unsafe { ((arg0 + TARGET_OFF) as *const u32).read_unaligned() };
        if target == 0 {
            return 0;
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(1, u32, target.wrapping_add(HELPER_BIAS));
        if answer == arg1 { 1 } else { 0 }
    }
});
