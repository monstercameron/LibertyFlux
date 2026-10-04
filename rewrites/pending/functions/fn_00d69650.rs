// original: 0x00d69650 indexed_entry_or_null
// s16f05: fetch the indexed table entry, or null (thiscall/0).
//
// Follows this record's child pointer, bounds-checks the child's signed
// index (negative means empty) and returns the table slot, or null when
// any link in the chain is missing.
export!(thiscall, rw_s16f05(this: *const u8) -> u32 {
    unsafe {
        let child = *((this.add(4)) as *const u32);
        if child == 0 {
            return 0;
        }
        let index = *((child.wrapping_add(0xA0)) as *const i32);
        if index < 0 {
            return 0;
        }
        let holder = *((child.wrapping_add(0x9C)) as *const u32);
        let table = *(holder as *const u32);
        *((table.wrapping_add((index as u32).wrapping_mul(4))) as *const u32)
    }
});
