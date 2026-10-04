// original: 0x00e5c770 timing_slots_poll16
/// Polls sixteen timer slots spaced 0x40 bytes apart.
///
/// Invokes the slot step routine (thiscall/0, stubbed by the checker) on each
/// of the sixteen slots in order and returns the last step answer, matching
/// the value the original leaves in EAX.
export!(cdecl, rw_00e5c770() -> u32 {
    unsafe {
        const BASE: u32 = 0x018DE9E0;
        const COUNT: u32 = 16;
        const STRIDE: u32 = 0x40;
        let mut slot = relocated(BASE);
        let mut last = 0u32;
        let mut i = 0u32;
        while i < COUNT {
            last = callee_thiscall!(1, u32, slot);
            slot = slot.wrapping_add(STRIDE);
            i += 1;
        }
        last
    }
});

