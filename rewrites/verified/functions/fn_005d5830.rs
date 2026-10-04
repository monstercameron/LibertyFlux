// original: 0x005d5830 table_find_zero_key
/// Search bucket zero of the hash table for the first node whose key word
/// is zero and return the payload address just past that key.
///
/// The bucket index comes out of a `0 % count` division, so it is always
/// zero (the divide is exact and fault-free for the guarded non-zero
/// count); the chain links live at node offset `+0x10`. Returns null when
/// the table is empty, the bucket is empty, or no node has a zero key.
export!(thiscall, rw_005d5830(this_ptr: u32) -> u32 {
    let count = unsafe { ((this_ptr + 4) as *const u16).read() };
    if count == 0 {
        return 0;
    }
    let table = unsafe { (this_ptr as *const u32).read() };
    let mut node = unsafe { (table as *const u32).read() };
    if node == 0 {
        return 0;
    }
    loop {
        if unsafe { (node as *const u32).read() } == 0 {
            return node.wrapping_add(4);
        }
        node = unsafe { ((node + 0x10) as *const u32).read() };
        if node == 0 {
            return 0;
        }
    }
});
