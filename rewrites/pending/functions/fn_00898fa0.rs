// original: 0x00898fa0 audio_ptrkey_find
/// Finds a key in a 24-entry pointer-keyed slot table.
///
/// Scans the eight-byte slots for the key, returning a pointer at the value
/// half of the matching slot, or 0 at the first empty slot or when the table
/// is full of other keys.
export!(thiscall, rw_00898fa0(this: u32, key: u32) -> u32 {
    unsafe {
        for i in 0..24u32 {
            let slot = this.wrapping_add(i.wrapping_mul(8));
            let v = core::ptr::read_unaligned(slot as *const u32);
            if v == key {
                return slot.wrapping_add(4);
            }
            if v == 0 {
                return 0;
            }
        }
        0
    }
});
