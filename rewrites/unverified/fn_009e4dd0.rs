// original: 0x009e4dd0 ped_slots_unlink_all (proposed)

/// Unlink every occupied slot of the four-slot table at `this + 0xd58`.
///
/// For each of the four dwords that is nonzero, calls the unlink callee
/// (`thiscall`: this = the slot's value, one stack word = the slot's
/// address) and returns the last call's result. When no slot is occupied
/// the original returns its entry `eax`, which a rewrite cannot observe;
/// the contract therefore always keeps at least one slot occupied (noted
/// in `narrowed`). `thiscall`, no stack words.
lf_checker_rt::export!(thiscall, rw_009e4dd0(this: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0xd58;
        const SLOT_COUNT: u32 = 4;
        const UNLINK: u32 = 1;
        let mut last: u32 = 0;
        let mut slot: u32 = this.wrapping_add(SLOT_BASE);
        for _ in 0..SLOT_COUNT {
            let value = (slot as *const u32).read_unaligned();
            if value != 0 {
                last = lf_checker_rt::callee_thiscall!(UNLINK, u32, value, slot);
            }
            slot = slot.wrapping_add(4);
        }
        last
    }
});
