// original: 0x009B9FD0 CCamScriptInstruction_SetCinematicButtonEnabled::vf2

/// Execute the SetCinematicButtonEnabled script instruction: write 1 to the
/// cinematic-button flag global when the operand byte at `this+0x08` is
/// zero, else 0 (an inverted enable: `sete`).
///
/// Only the flag byte is written; the three neighbouring bytes of its word
/// keep their values. No calls, no return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B9FD0(this: u32) -> u32 {
    unsafe {
        const OPERAND: u32 = 0x08;
        const FLAG: u32 = 0x012B_D1BF;
        let is_zero = ((this + OPERAND) as *const u8).read() == 0;
        lf_checker_rt::global::<u8>(FLAG).write(is_zero as u8);
        0
    }
});
