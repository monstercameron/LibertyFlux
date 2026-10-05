// original: 0x00ab68f0 idmap_find_b (proposed)

/// Look up `key` in the wide-node id map and return the payload pointer.
///
/// Same shape as the sibling lookup at 0x00ab6880 with a different header
/// layout: bucket array at `+4`, 16-bit bucket count at `+8`, chain link at
/// `+0x8c`. A zero count returns null without dividing; otherwise bucket
/// `key % count` is walked until an entry whose word at `+0` equals the key,
/// and that entry plus 4 is returned, or null when the chain ends.
///
/// Original: 0x00ab68f0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ab68f0(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        const NEXT_OFF: u32 = 0x8C;
        const PAYLOAD_OFF: u32 = 4;
        let count = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let mut entry = ((table + (key % count) * 4) as *const u32).read_unaligned();
        while entry != 0 {
            if (entry as *const u32).read_unaligned() == key {
                return entry.wrapping_add(PAYLOAD_OFF);
            }
            entry = ((entry + NEXT_OFF) as *const u32).read_unaligned();
        }
        0
    }
});
