// original: 0x00b576f0 count_entries_with_id
// rs05f16: count list entries with a given id.
//
// Counts entries of the list at `this+0x1A28` whose kind word is 1, whose
// flag at `+0x44` is set and whose id at `+0xC` equals `wanted`.
export!(thiscall, rw_b576f0(this: *const u8, wanted: u32) -> u32 {
    unsafe {
        let mut node = *(this.add(0x1A28) as *const *const u8);
        let mut count = 0u32;
        while !node.is_null() {
            if *(node.add(0x48) as *const u16) == 1
                && *(node.add(0x44) as *const u32) != 0
                && *(node.add(0x0C) as *const u32) == wanted
            {
                count += 1;
            }
            node = *(node.add(0x8C) as *const *const u8);
        }
        count
    }
});
