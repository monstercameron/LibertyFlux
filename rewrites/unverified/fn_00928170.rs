// original: 0x00928170 input_slot_index_by_key (proposed)

/// Find the index of an input slot whose id matches the key stored in an object.
///
/// `obj` points to an object holding a 32-bit key at `+0x60`. Eight slot
/// records live in global memory starting at `TABLE`, each `STRIDE` (0x100)
/// bytes apart, with the slot id in the first dword of each record. The
/// scan compares the key against each id in order and returns the zero-based
/// index of the first match, or 0xFFFF_FFFF when no slot matches.
///
/// Original: 0x00928170 (cdecl, one stack word). Leaf: no calls, no writes.
lf_checker_rt::export!(cdecl, rw_00928170(obj: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x60;
        const TABLE: u32 = 0x011A_0BF0;
        const TABLE_END: u32 = 0x011A_13F0;
        const STRIDE: u32 = 0x100;
        let key = (obj.wrapping_add(KEY_OFF) as *const u32).read_unaligned();
        let mut slot = lf_checker_rt::relocated(TABLE);
        let end = lf_checker_rt::relocated(TABLE_END);
        let mut index: u32 = 0;
        while slot < end {
            if (slot as *const u32).read_unaligned() == key {
                return index;
            }
            slot = slot.wrapping_add(STRIDE);
            index = index.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
