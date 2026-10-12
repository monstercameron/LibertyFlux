// original: 0x00BE6B30 CTaskSimpleSayAudio::vf5
/// Abort-check for a say-audio task against ped `a0`.
///
/// Probes the ped's voice state (thiscall on `a0+0x570`); when the probe's
/// low byte is nonzero and the task's state field at `this+0x14` equals 3,
/// stops the voice (second thiscall on `a0+0x570`). Always finishes with a
/// thiscall on `a0` itself. Returns that call's high three bytes with low
/// byte forced to 1. Original is thiscall with three stack words (only the
/// first is read) and pops 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6b30(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    const PROBE: u32 = 1;
    const STOP: u32 = 2;
    const FINISH: u32 = 3;
    const VOICE_OFF: u32 = 0x570;
    const STATE_OFF: u32 = 0x14;
    const STOP_STATE: u32 = 3;
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        let voice = a0.wrapping_add(VOICE_OFF);
        let probed = lf_checker_rt::callee_thiscall!(PROBE, u32, voice);
        if probed as u8 != 0 && rd32(this + STATE_OFF) == STOP_STATE {
            lf_checker_rt::callee_thiscall!(STOP, u32, voice);
        }
        let done = lf_checker_rt::callee_thiscall!(FINISH, u32, a0);
        (done & 0xFFFFFF00) | 1
    }
});
