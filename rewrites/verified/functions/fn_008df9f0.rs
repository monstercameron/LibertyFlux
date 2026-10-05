// original: 0x008df9f0 hash_lookup_3c (proposed)

/// Look up `key` in a chained hash table, returning its value word.
///
/// `this` holds the bucket array pointer at `+0x3c` and the bucket count
/// (16-bit) at `+0x40`. Nodes are `[key, value, next]` triples. The bucket
/// is `key % count`; its chain is walked for a node whose key matches, and
/// the value word (at node `+4`) is returned. Returns `0xffffffff` when the
/// count is zero, the bucket is empty, or no node matches; a match whose
/// value address computes to null also yields `0xffffffff` (defensive, not
/// reachable with real nodes). Thiscall, one stack argument, no calls.
lf_checker_rt::export!(thiscall, rw_008df9f0(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x3c;
        const COUNT_OFF: u32 = 0x40;
        const NODE_VALUE: u32 = 4;
        const NODE_NEXT: u32 = 8;
        const MISSING: u32 = 0xffff_ffff;
        let count = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return MISSING;
        }
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let mut node =
            ((table + (key % count).wrapping_mul(4)) as *const u32).read_unaligned();
        loop {
            if node == 0 {
                return MISSING;
            }
            if (node as *const u32).read_unaligned() == key {
                let val_addr = node.wrapping_add(NODE_VALUE);
                if val_addr == 0 {
                    return MISSING;
                }
                return (val_addr as *const u32).read_unaligned();
            }
            node = ((node + NODE_NEXT) as *const u32).read_unaligned();
        }
    }
});
