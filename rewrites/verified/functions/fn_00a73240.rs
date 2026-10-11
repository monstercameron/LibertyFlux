// original: 0x00A73240 CTaskSimpleMovePlayer::vf5

/// Clears the task's word at offset `0x24` and returns true.
///
/// The three stack arguments are unused. The thiscall callee pops all three
/// words; the result is the low byte of EAX.
lf_checker_rt::export!(thiscall, rw_00a73240(this: u32, _arg0: u32, _arg1: u32, _arg2: u32) -> u8 {
    unsafe {
        const STATE_OFFSET: u32 = 0x24;
        ((this + STATE_OFFSET) as *mut u32).write_unaligned(0);
        1
    }
});
