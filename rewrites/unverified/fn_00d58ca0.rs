// original: 0x00d58ca0 CCamViewFind::vf5
/// Tick this finder: while armed and inside its time window, probe for the
/// target; past the window's end, shut down.
///
/// Reads the clock global `CLOCK`, subtracts the start at `[this+0x144]` and
/// returns 1 at once when byte `ARMED` (+0x140) is clear or the elapsed time
/// is not positive. Otherwise it compares the elapsed time against the span
/// (`[this+0x148]` minus start): past the end it shuts down (intercepted
/// callee 1, argument 0) and returns 0, inside the window it probes
/// (intercepted callee 2) and returns whether the probe answered nonzero.
///
/// Original: thiscall, no stack arguments; comparisons are signed.
lf_checker_rt::export!(thiscall, rw_00d58ca0 (this: u32) -> u32 {
    unsafe {
        const CLOCK: u32 = 0x011735C4;
        const ARMED: u32 = 0x140;
        const START: u32 = 0x144;
        const END: u32 = 0x148;
        const SHUTDOWN: u32 = 1;
        const PROBE: u32 = 2;
        let now = lf_checker_rt::global::<u32>(CLOCK).read_unaligned();
        let start = ((this + START) as *const u32).read_unaligned();
        let elapsed = now.wrapping_sub(start);
        if ((this + ARMED) as *const u8).read() == 0 || (elapsed as i32) <= 0 {
            return 1;
        }
        let span = ((this + END) as *const u32).read_unaligned().wrapping_sub(start);
        if (elapsed as i32) > (span as i32) {
            let _: u32 = lf_checker_rt::callee_thiscall!(SHUTDOWN, u32, this, 0);
            0
        } else {
            let ans: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, this);
            if ans & 0xFF != 0 { 1 } else { 0 }
        }
    }
});
