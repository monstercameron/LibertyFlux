// original: 0x00cbbb70 CTaskSimpleMoveTrackingEntity::vf1
/// Store the new tracking speed into two object slots (vf1, leaf).
///
/// Copies the float argument `a0` into `[this + 0x4]` and `[this + 0x10]`
/// (thiscall, two stack arguments; `a1` is unread). The original briefly
/// spills the old `[this + 0x4]` to its own frame first, which is dead.
/// No return value (the original never writes eax), no calls.
lf_checker_rt::export!(thiscall, rw_00cbbb70(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        /// Speed slots receiving the new value.
        const SPEED_A: u32 = 0x4;
        const SPEED_B: u32 = 0x10;
        ((this + SPEED_A) as *mut u32).write_unaligned(a0);
        ((this + SPEED_B) as *mut u32).write_unaligned(a0);
        0
    }
});
