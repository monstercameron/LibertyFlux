// original: 0x00a3af90 vehicle_slot_gt (proposed)

/// Compare one of four 16-bit slots against a 16-bit bound (unsigned).
///
/// `which` (0..3) selects the slot at `obj + 0x170 + which*2` through a jump
/// table; any other value answers 0. Otherwise answers 1 exactly when the
/// slot is strictly greater than `bound`. Thiscall/2, callee cleans 8,
/// returns AL.
lf_checker_rt::export!(thiscall, rw_00a3af90(obj: u32, which: u32, bound: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x170;
        if which > 3 {
            return 0;
        }
        let slot = core::ptr::read_unaligned((obj + SLOTS + which * 2) as *const u16);
        ((slot as u32) > (bound & 0xFFFF)) as u32
    }
});
