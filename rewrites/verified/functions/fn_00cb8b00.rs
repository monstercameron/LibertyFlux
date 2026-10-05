// original: 0x00cb8b00 CTaskSimpleMoveTrackingEntity::vf5
/// Dispatch a tracking task on its mode argument (vf5).
///
/// With mode `a1` of 1 returns 1 at once; with 2 calls the re-acquire
/// callee with (`a0`, 1) (thiscall, three stack arguments; `a2` is unread)
/// and returns its answer with the low byte forced to 1; with any other
/// mode sets bit 2 in the flag byte at `[this + 0x58]` and returns `a1`
/// with its low byte cleared (the original's `(an instruction of the original)` clears only the
/// low byte). The callee is intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00cb8b00(this: u32, a0: u32, a1: u32, _a2: u32) -> u32 {
    unsafe {
        /// Flag byte and bit set for an unrecognised mode.
        const FLAG_OFF: u32 = 0x58;
        const MODE_BIT: u8 = 2;
        /// Callee id of the re-acquire routine.
        const REACQUIRE: u32 = 1;
        if a1 == 1 {
            (a1 & 0xFFFFFF00) | 1
        } else if a1 == 2 {
            let v: u32 = lf_checker_rt::callee_thiscall!(REACQUIRE, u32, a0, 1);
            (v & 0xFFFFFF00) | 1
        } else {
            let b = ((this + FLAG_OFF) as *const u8).read();
            ((this + FLAG_OFF) as *mut u8).write(b | MODE_BIT);
            a1 & 0xFFFFFF00
        }
    }
});
