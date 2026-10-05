// original: 0x00a902a0 stream_countdown_gate

/// Runs the pending consumer call while the countdown lasts.
///
/// No arguments. When the ready byte at file VA 0x12FB21D is not 1,
/// returns the incoming EAX (fixed to 0 by the contract). Otherwise reads
/// the countdown at 0x12FB224: at zero it clears the ready byte and the tag
/// byte at 0x12FB21E and returns 0; otherwise it decrements the countdown
/// and calls the consumer (callee 1, cdecl) with (0x12FB240, tag). A
/// consumer answer with a zero low byte keeps the flags and is returned;
/// any other answer clears both bytes first. One call at most.
/// Original: 0x00A902A0 (cdecl, no words), 64 bytes.
lf_checker_rt::export!(cdecl, rw_00a902a0() -> u32 {
    unsafe {
        const READY: u32 = 0x12FB21D;
        const TAG: u32 = 0x12FB21E;
        const COUNT: u32 = 0x12FB224;
        const VEC: u32 = 0x12FB240;
        const CONSUME: u32 = 1;
        if (lf_checker_rt::global::<u8>(READY) as *const u8).read() != 1 {
            return 0;
        }
        let slot = lf_checker_rt::global::<u32>(COUNT);
        let left = slot.read();
        if left == 0 {
            (lf_checker_rt::global::<u8>(READY) as *mut u8).write(0);
            (lf_checker_rt::global::<u8>(TAG) as *mut u8).write(0);
            return 0;
        }
        slot.write(left.wrapping_sub(1));
        let tag = (lf_checker_rt::global::<u8>(TAG) as *const u8).read() as u32;
        let ans: u32 =
            lf_checker_rt::callee_cdecl!(CONSUME, u32, lf_checker_rt::relocated(VEC), tag);
        // `(an instruction of the original)`: only the answer's low byte decides.
        if (ans & 0xFF) != 0 {
            (lf_checker_rt::global::<u8>(READY) as *mut u8).write(0);
            (lf_checker_rt::global::<u8>(TAG) as *mut u8).write(0);
        }
        ans
    }
});
