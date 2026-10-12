// original: 0x00BE70C0 CTaskSimpleClearLookAt::vf17
/// Process a clear-look-at task for ped `a0`.
///
/// Probes the ped's look-at slot (thiscall on `a0+0xBB0`); when the probe's
/// low byte is nonzero, clears it with duration 500 (thiscall on `a0+0xBB0`
/// with argument `DURATION`). Returns the last call's high three bytes with
/// low byte forced to 1. `this` is unread. Original is thiscall with one
/// stack word and pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_00be70c0(_this: u32, a0: u32) -> u32 {
    const PROBE: u32 = 1;
    const CLEAR: u32 = 2;
    const SLOT_OFF: u32 = 0xBB0;
    const DURATION: u32 = 500;
    unsafe {
        let slot = a0.wrapping_add(SLOT_OFF);
        let probed = lf_checker_rt::callee_thiscall!(PROBE, u32, slot);
        if probed as u8 != 0 {
            let cleared = lf_checker_rt::callee_thiscall!(CLEAR, u32, slot, DURATION);
            (cleared & 0xFFFFFF00) | 1
        } else {
            (probed & 0xFFFFFF00) | 1
        }
    }
});
