// original: 0x009BA200 CCamScriptInstruction_SetFollowVehiclePitchLimitDown::vf2
/// Copy the instruction's float operand into the follow-vehicle pitch-limit-down global.
///
/// `this` is the instruction object (thiscall); the operand is the float at
/// `+0x08`. The value is copied bit for bit to the global, so NaN payloads
/// survive. No calls, no branches.
lf_checker_rt::export!(thiscall, rw_009BA200(this: u32) -> u32 {
    unsafe {
        const OPERAND: u32 = 0x08;
        const PITCH_LIMIT_DOWN: u32 = 0x103B764;
        let bits = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
        lf_checker_rt::global::<u32>(PITCH_LIMIT_DOWN).write_unaligned(bits);
        0
    }
});
