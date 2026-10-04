// original: 0x005e5f80 hash_table_find_string
// rw_005e5f80: look up a string key in a chained hash table.
//
// Hashes the key, picks the bucket by remainder, and walks the chain
// comparing full strings; an empty tag selects the shared empty string on
// either side. Returns the value pointer of the first match, or null.
export!(thiscall, rw_005e5f80(this: u32, key: u32) -> u32 {
    unsafe {
        const EMPTY: u32 = 0xFC9C85;
        let obj = this as *const u8;
        let count = *((obj.add(4)) as *const u16);
        if count == 0 {
            return 0;
        }
        let key_tag = *((key as *const u8).add(4) as *const u16);
        let key_text = if key_tag == 0 {
            relocated(EMPTY)
        } else {
            *(key as *const u32)
        };
        let hash: u32 = callee_cdecl!(1, u32, key_text);
        let table = *(this as *const u32) as *const u32;
        let mut node = *table.add((hash % count as u32) as usize);
        while node != 0 {
            let tag = *((node as *const u8).add(4) as *const u16);
            let text = if tag == 0 {
                relocated(EMPTY)
            } else {
                *(node as *const u32)
            };
            let canon: u32 = callee_thiscall!(2, u32, key);
            let mut i = 0usize;
            let same = loop {
                let x = *((canon as *const u8).add(i));
                let y = *((text as *const u8).add(i));
                if x != y {
                    break false;
                }
                if x == 0 {
                    break true;
                }
                i += 1;
            };
            if same {
                return node.wrapping_add(8);
            }
            node = *((node as *const u8).add(0xC) as *const u32);
        }
        0
    }
});
