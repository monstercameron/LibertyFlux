// original: 0x00d58770 CCamRadar::vf2
/// Detach this camera's radar child: shut it down, clear the attached bit
/// and drop the link.
///
/// Calls the child's shutdown routine (intercepted callee 1, thiscall, one
/// argument: 0) with the child at `[this+0x128]`, clears bit `CHILD_BIT`
/// (0x20) in `[this+0x13c]`, zeroes the link at `[this+0x128]` and returns
/// the shutdown answer with its low byte forced to 1.
///
/// Original: thiscall, no stack arguments; `(an instruction of the original)` over the last answer.
lf_checker_rt::export!(thiscall, rw_00d58770 (this: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x128;
        const FLAGS: u32 = 0x13c;
        const CHILD_BIT: u8 = 0x20;
        const SHUTDOWN: u32 = 1;
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let ans: u32 = lf_checker_rt::callee_thiscall!(SHUTDOWN, u32, child, 0);
        let flags = (this + FLAGS) as *mut u8;
        flags.write(flags.read() & !CHILD_BIT);
        ((this + CHILD) as *mut u32).write_unaligned(0);
        (ans & 0xFFFFFF00) | 1
    }
});
