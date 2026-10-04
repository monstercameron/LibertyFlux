// original: 0x00d69670 prev_indexed_entry_or_null
// s16f06: fetch the previous indexed table entry, or null (thiscall/0).
//
// Like the indexed fetch, but the child stores a one-based position: the
// entry below it is returned, and position zero (or less) means empty.
export!(thiscall, rw_s16f06(this: *const u8) -> u32 {
    unsafe {
        let child = *((this.add(4)) as *const u32);
        if child == 0 {
            return 0;
        }
        let pos = *((child.wrapping_add(0xA0)) as *const u32);
        if (pos.wrapping_sub(1) as i32) < 0 {
            return 0;
        }
        let holder = *((child.wrapping_add(0x9C)) as *const u32);
        let table = *(holder as *const u32);
        *((table.wrapping_add(pos.wrapping_mul(4).wrapping_sub(4))) as *const u32)
    }
});
