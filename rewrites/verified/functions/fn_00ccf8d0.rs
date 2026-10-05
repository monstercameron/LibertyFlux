// original: 0x00ccf8d0 melee_state_check
/// Report whether the melee object still needs its follow-up check.
///
/// Returns 1 (in AL) when the count at `this+0x24` is zero. Otherwise asks
/// the inner object at `this+0x14` (thiscall, no arguments) when non-null,
/// returning 0 when it answers non-zero, and tail-calls the follow-up check
/// otherwise, returning its answer. Thiscall, no stack arguments.
export!(thiscall, rw_00ccf8d0(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x24;
        const INNER_OFF: u32 = 0x14;
        if (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        let inner = (this.wrapping_add(INNER_OFF) as *const u32).read_unaligned();
        if inner != 0 {
            let ans: u32 = callee_thiscall!(1, u32, inner);
            if ans & 0xff != 0 {
                return 0;
            }
        }
        callee_thiscall!(2, u32, this)
    }
});
