// original: 0x009BA1B0 CCamScriptInstruction_SetFollowPedPitchLimitUp::vf2

/// Execute the SetFollowPedPitchLimitUp script instruction: copy the instruction's 32-bit
/// operand at `this+0x08` into its dedicated engine global.
///
/// The original moves the value through an SSE register (`movss`), which is
/// a bit-copy with no arithmetic, so signalling NaNs and payloads pass
/// through unchanged. No calls, no return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009BA1B0(this: u32) -> u32 {
    unsafe {
        const OPERAND: u32 = 0x08;
        const GLOBAL: u32 = 0x0103BFE0;
        let bits = ((this + OPERAND) as *const u32).read_unaligned();
        lf_checker_rt::global::<u32>(GLOBAL).write(bits);
        0
    }
});
