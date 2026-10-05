// original: 0x00a8a310 pool_select_slot_a (proposed)

/// Select a slot offset by a small discriminator, then forward to the slot
/// setter.
///
/// `this` is the pool object, `which` is 0, 1 or 3 for the live paths and
/// `base` is the slot base: `which` 0 and 3 forward `base+3`, `which` 1
/// forwards `base+6`. Any other `which` returns `which-3` (the leftover of
/// the comparisons) and makes no call.
///
/// Original: 0x00A8A310 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a8a310(this: u32, which: u32, base: u32) -> u32 {
    unsafe {
        const CALLEE_SET_SLOT: u32 = 1;
        if which == 0 || which == 3 {
            return lf_checker_rt::callee_thiscall!(
                CALLEE_SET_SLOT,
                u32,
                this,
                base.wrapping_add(3)
            );
        }
        if which == 1 {
            return lf_checker_rt::callee_thiscall!(
                CALLEE_SET_SLOT,
                u32,
                this,
                base.wrapping_add(6)
            );
        }
        which.wrapping_sub(3)
    }
});
