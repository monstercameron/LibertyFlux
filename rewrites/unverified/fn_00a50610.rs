// original: 0x00a50610 vehicle_teardown_all_slots (proposed)

/// Tear down all 256 slots in order through the teardown callee.
///
/// Each slot address (`this + 0x100` stepping 0xe0) goes to the teardown
/// callee in ecx with no stack words. Thiscall, no stack words, one callee,
/// no result.
lf_checker_rt::export!(thiscall, rw_00a50610(this: u32) -> u32 {
    unsafe {
        const DOWN: u32 = 1;
        const SLOTS_BASE: u32 = 0x100;
        const SLOT_STRIDE: u32 = 0xe0;
        const NSLOTS: u32 = 0x100;
        let mut slot = this.wrapping_add(SLOTS_BASE);
        let mut left = NSLOTS;
        loop {
            lf_checker_rt::callee_thiscall!(DOWN, u32, slot);
            slot = slot.wrapping_add(SLOT_STRIDE);
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
        0
    }
});
