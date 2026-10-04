// original: 0x00951200 remove_validated_entry
/// Delete a table entry after validating key and tag.
///
/// Selects one of two global tables, checks the slot's key against `key`
/// and its tag against the high half of `id`, and only then clears the
/// entry (key zero, tag all-ones). Any failed check, a null key, a negative
/// id or an out-of-range slot leaves the tables untouched.
export!(cdecl, rw_00951200(which: u32, id: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_A: u32 = 0x01208970;
        const TABLE_B: u32 = 0x012142D0;
        const LIMIT: i16 = 0x818;
        if key == 0 {
            return 0;
        }
        if (id as i32) < 0 {
            return 0;
        }
        let slot = id as u16 as i16;
        if slot >= LIMIT {
            return 0;
        }
        let base = if which == 1 { TABLE_A } else { TABLE_B };
        let entry = base.wrapping_add((slot as i32 as u32).wrapping_mul(8));
        if *global::<u32>(entry) != key {
            return 0;
        }
        if *global::<u16>(entry + 4) != (id >> 16) as u16 {
            return 0;
        }
        *global::<u32>(entry) = 0;
        *global::<u16>(entry + 4) = 0xFFFF;
        0
    }
});
