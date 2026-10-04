// original: 0x008992a0 audio_hash_lookup
/// Looks up a name in a hash-bucketed string table.
///
/// Hashes the query's string pointer through the hash helper, picks the
/// bucket by dividing by the table size, then walks the chain comparing
/// NUL-terminated bytes. Returns a pointer past the matching node's string
/// pointer, or 0 when the table is empty, the bucket is empty, or nothing
/// matches.
export!(thiscall, rw_008992a0(this: u32, query: u32) -> u32 {
    unsafe {
        let count =
            core::ptr::read_unaligned(this.wrapping_add(4) as *const u16) as u32;
        if count == 0 {
            return 0;
        }
        let key = core::ptr::read_unaligned(query as *const u32);
        let h = callee_cdecl!(1, u32, key);
        let buckets = core::ptr::read_unaligned(this as *const u32);
        let mut node =
            core::ptr::read_unaligned(buckets.wrapping_add((h % count).wrapping_mul(4)) as *const u32);
        loop {
            if node == 0 {
                return 0;
            }
            let cand = core::ptr::read_unaligned(node as *const u32);
            let mut off = 0u32;
            let mut equal = false;
            loop {
                let a = core::ptr::read_unaligned(key.wrapping_add(off) as *const u8);
                let b = core::ptr::read_unaligned(cand.wrapping_add(off) as *const u8);
                if a != b {
                    break;
                }
                if a == 0 {
                    equal = true;
                    break;
                }
                off = off.wrapping_add(1);
            }
            if equal {
                return node.wrapping_add(4);
            }
            node = core::ptr::read_unaligned(node.wrapping_add(8) as *const u32);
        }
    }
});
