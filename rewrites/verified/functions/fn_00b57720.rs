// original: 0x00b57720 count_active_entries
// rs05f17: count active list entries.
//
// Counts entries of the list at `this+0x1A28` whose kind word is 1 and whose
// flag at `+0x44` is set.
export!(thiscall, rw_b57720(this: *const u8) -> u32 {
    unsafe {
        let mut node = *(this.add(0x1A28) as *const *const u8);
        let mut count = 0u32;
        while !node.is_null() {
            if *(node.add(0x48) as *const u16) == 1
                && *(node.add(0x44) as *const u32) != 0
            {
                count += 1;
            }
            node = *(node.add(0x8C) as *const *const u8);
        }
        count
    }
});
