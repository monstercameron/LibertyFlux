// original: 0x009BA210 CCamScriptInstruction_SetFollowVehiclePitchLimitUp::vf2
/// Copy the instruction's float operand into the follow-vehicle pitch-limit-up global.
///
/// Same shape as the pitch-limit-down setter: float at `this+0x08` copied bit
/// for bit to its own global. No calls, no branches.
lf_checker_rt::export!(thiscall, rw_009BA210(this: u32) -> u32 {
    unsafe {
        const OPERAND: u32 = 0x08;
        const PITCH_LIMIT_UP: u32 = 0x103B760;
        let bits = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
        lf_checker_rt::global::<u32>(PITCH_LIMIT_UP).write_unaligned(bits);
        0
    }
});
