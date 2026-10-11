// original: 0x006118B0 physics_hash_bucket_insert

/// Pop one node from the free-node chain, hash the input key to a masked
/// bucket, link the node at that bucket's head, copy the input's two words,
/// and increment the table's occupied count. The table header stores its
/// bucket count at +0, bucket array at +8 and free-list head at +0xc; a node's
/// chain link is at +8. All hash stages use wrapping 32-bit arithmetic and
/// logical right shifts. This is a 32-bit thiscall with an input record pointer.
lf_checker_rt::export!(thiscall, rw_006118B0(table: u32, input: u32) -> u32 {
    unsafe {
        const BUCKET_COUNT: u32 = 0;
        const SIZE: u32 = 4;
        const BUCKETS: u32 = 8;
        const FREE_HEAD: u32 = 0x0c;
        const NODE_NEXT: u32 = 8;
        const INPUT_KEY: u32 = 0;
        const INPUT_VALUE: u32 = 4;

        let key = ((input + INPUT_KEY) as *const u32).read_unaligned();
        let mut hash = key.wrapping_mul(0x1001);
        hash ^= hash >> 22;
        hash = hash.wrapping_mul(17);
        hash ^= hash >> 9;
        hash = hash.wrapping_mul(0x401);
        hash ^= hash >> 2;
        hash = hash.wrapping_mul(0x81);
        hash ^= hash >> 12;
        if hash == 0 {
            hash = hash.wrapping_add(1);
        }

        let capacity = ((table + BUCKET_COUNT) as *const u32).read_unaligned();
        let bucket = hash & capacity.wrapping_sub(1);
        let bucket_array = ((table + BUCKETS) as *const u32).read_unaligned();
        let bucket_slot = bucket_array.wrapping_add(bucket.wrapping_mul(4));
        let node = ((table + FREE_HEAD) as *const u32).read_unaligned();
        let next_free = ((node + NODE_NEXT) as *const u32).read_unaligned();
        let prior_bucket_head = (bucket_slot as *const u32).read_unaligned();

        ((table + FREE_HEAD) as *mut u32).write_unaligned(next_free);
        (bucket_slot as *mut u32).write_unaligned(node);
        ((node + NODE_NEXT) as *mut u32).write_unaligned(prior_bucket_head);
        let value = ((input + INPUT_VALUE) as *const u32).read_unaligned();
        (node as *mut u32).write_unaligned(key);
        ((node + 4) as *mut u32).write_unaligned(value);
        let size = ((table + SIZE) as *const u32).read_unaligned();
        ((table + SIZE) as *mut u32).write_unaligned(size.wrapping_add(1));
        node
    }
});
