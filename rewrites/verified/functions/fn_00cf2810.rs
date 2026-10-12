// original: 0x00CF2810 phmaterial_table_find

/// Find a case-sensitive key in the bucket selected by the table hash.
///
/// An empty table returns null without hashing. Otherwise the one-bucket
/// index is the unsigned hash remainder, and the function walks the linked
/// chain comparing NUL-terminated bytes. A match returns the address of the
/// node's value word (`node + 4`); an exhausted chain returns null. The
/// thiscall wrapper removes its key pointer argument.
lf_checker_rt::export!(thiscall, rw_fn_00cf2810(this: u32, key: u32) -> u32 {
    const BUCKETS_FIELD: u32 = 0;
    const BUCKET_COUNT_FIELD: u32 = 4;
    const NODE_KEY_FIELD: u32 = 0;
    const NODE_VALUE_FIELD: u32 = 4;
    const NODE_NEXT_FIELD: u32 = 8;
    const HASH_CALLEE: u32 = 1;

    let bucket_count = unsafe {
        (this.wrapping_add(BUCKET_COUNT_FIELD) as *const u16).read_unaligned()
    };
    if bucket_count == 0 {
        return 0;
    }

    let hash = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, key);
    let bucket_index = hash % u32::from(bucket_count);
    let buckets = unsafe {
        (this.wrapping_add(BUCKETS_FIELD) as *const u32).read_unaligned()
    };
    let mut node = unsafe {
        (buckets.wrapping_add(bucket_index.wrapping_mul(4)) as *const u32).read_unaligned()
    };

    while node != 0 {
        let stored_key = unsafe {
            (node.wrapping_add(NODE_KEY_FIELD) as *const u32).read_unaligned()
        };
        if c_string_equal(key, stored_key) {
            return node.wrapping_add(NODE_VALUE_FIELD);
        }
        node = unsafe {
            (node.wrapping_add(NODE_NEXT_FIELD) as *const u32).read_unaligned()
        };
    }
    0
});

#[inline(always)]
fn c_string_equal(mut left: u32, mut right: u32) -> bool {
    loop {
        let left_byte = unsafe { (left as *const u8).read() };
        let right_byte = unsafe { (right as *const u8).read() };
        if left_byte != right_byte {
            return false;
        }
        if left_byte == 0 {
            return true;
        }
        left = left.wrapping_add(1);
        right = right.wrapping_add(1);
    }
}
