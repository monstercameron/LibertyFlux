// original: 0x00967BD0 register_timing_pair
/// Find or insert the `(a0, a1)` pair in the six-slot table at `this`.
///
/// The table holds six key dwords at `this + 0x31E0` with a companion
/// value 0x18 bytes past each key (i.e. at `this + 0x31F8 + i * 4`).
/// Scans slots 0..6: a slot whose key equals `a0` and whose companion
/// equals `a1` ends the scan with no write; the first zero key takes the
/// pair. A full table with no match writes nothing. The companion word is
/// read only for slots whose key equals `a0`, matching the original's
/// fault behaviour. No return value (`ret: none`).
///
/// Original: 0x00967BD0 (thiscall, two stack words).

export!(thiscall, rw_00967BD0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const KEYS: u32 = 0x31E0;
        const COMPANION_DELTA: u32 = 0x18;
        for i in 0..6u32 {
            let slot = this.wrapping_add(KEYS).wrapping_add(i * 4);
            let key = (slot as *const u32).read_unaligned();
            if key == a0
                && (slot.wrapping_add(COMPANION_DELTA) as *const u32).read_unaligned() == a1
            {
                return 0;
            }
            if key == 0 {
                (slot as *mut u32).write_unaligned(a0);
                (slot.wrapping_add(COMPANION_DELTA) as *mut u32).write_unaligned(a1);
                return 0;
            }
        }
        0
    }
});
