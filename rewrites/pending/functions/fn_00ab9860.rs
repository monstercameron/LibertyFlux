// original: 0x00ab9860 two_slot_search
/// Search two 0x40-spaced dwords at `this` for `needle`.
///
/// On a match the original writes the index over its incoming argument
/// slot (invisible to the caller: that slot is clobbered by the return
/// path) and tail-dispatches to the slot resolver with (this, index);
/// when nothing matches it returns -1 (as u32).
lf_checker_rt::export!(thiscall, rw_00ab9860(this_ptr: u32, needle: u32) -> u32 {
    for i in 0..2u32 {
        // SAFETY: the caller guarantees a readable object; the checker
        // backs it with heap memory on every trial.
        let v = unsafe { ((this_ptr.wrapping_add(i * 0x40)) as *const u32).read_unaligned() };
        if v == needle {
            return lf_checker_rt::callee_thiscall!(10, u32, this_ptr, i);
        }
    }
    0xFFFF_FFFF
});

