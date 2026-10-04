// original: 0x005d5870 table_find_key
/// Look `key` up in the hash table: bucket `key % count`, then walk the
/// chain (links at node offset `+0xc`) comparing key words. Returns the
/// payload address just past the matching key, or null.
export!(thiscall, rw_005d5870(this_ptr: u32, key: u32) -> u32 {
    let count = unsafe { ((this_ptr + 4) as *const u16).read() } as u32;
    if count == 0 {
        return 0;
    }
    let table = unsafe { (this_ptr as *const u32).read() };
    let mut node = unsafe {
        (table.wrapping_add((key % count).wrapping_mul(4)) as *const u32).read()
    };
    if node == 0 {
        return 0;
    }
    loop {
        if unsafe { (node as *const u32).read() } == key {
            return node.wrapping_add(4);
        }
        node = unsafe { ((node + 0x0C) as *const u32).read() };
        if node == 0 {
            return 0;
        }
    }
});
