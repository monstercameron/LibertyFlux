// original: 0x00a496b0 vehicle_classify_occupant_ptr
/// Classify a pointer against the four occupant slots at `this+0xF50`.
///
/// Returns 0, 2 or 1 when the argument equals the slot at `+0xF50`, `+0xF54`
/// or `+0xF58`; 3 when it equals `+0xF5C`; -1 otherwise (thiscall, one stack
/// word). The codes are deliberately out of order in the original.
export!(thiscall, rw_00a496b0(this: u32, p: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0xf50;
        if p == (this.wrapping_add(SLOTS) as *const u32).read_unaligned() {
            return 0;
        }
        if p == (this.wrapping_add(SLOTS + 4) as *const u32).read_unaligned() {
            return 2;
        }
        if p == (this.wrapping_add(SLOTS + 8) as *const u32).read_unaligned() {
            return 1;
        }
        if p == (this.wrapping_add(SLOTS + 12) as *const u32).read_unaligned() {
            3
        } else {
            0xFFFFFFFF
        }
    }
});
