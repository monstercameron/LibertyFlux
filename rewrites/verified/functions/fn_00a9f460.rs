// original: 0x00a9f460 stream_find_slot_by_state (proposed)

/// Find the first of 64 fixed slots whose state word equals 3.
///
/// `this` points to an object holding 64 slots of 0x70 bytes starting at
/// `+0x1cb0`; the state word is each slot's first word. Slots are scanned in
/// order and the address of the first slot holding exactly 3 is returned.
/// When no slot holds 3 the result is null. The comparison is an exact
/// 32-bit equality (no masking, no range).
///
/// Original: 0x00a9f460 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00a9f460(this: u32) -> u32 {
    unsafe {
        const SLOTS_OFF: u32 = 0x1cb0;
        const SLOT_STRIDE: u32 = 0x70;
        const SLOT_COUNT: u32 = 64;
        const WANTED: u32 = 3;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let at = this.wrapping_add(SLOTS_OFF).wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            if (at as *const u32).read_unaligned() == WANTED {
                return at;
            }
            i += 1;
        }
        0
    }
});
