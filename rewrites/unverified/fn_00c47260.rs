// original: 0x00c47260 CCamScript::vf6 (symbols)
/// Reset the script camera's timer slots to their idle values.
///
/// Writes 0 to the halfword at `this + FLAG`, and `IDLE` (1000) to the
/// words at `this + TIMER_A` and `this + TIMER_B`. Returns 1 in the low
/// byte (the original sets only `al`).
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c47260(this: u32) -> u32 {
    const FLAG: u32 = 0x148;
    const TIMER_A: u32 = 0x140;
    const TIMER_B: u32 = 0x144;
    const IDLE: u32 = 1000;
    unsafe {
        ((this + FLAG) as *mut u16).write_unaligned(0);
        ((this + TIMER_A) as *mut u32).write_unaligned(IDLE);
        ((this + TIMER_B) as *mut u32).write_unaligned(IDLE);
    }
    1
});
