// original: 0x008e6d70 hash_bucket_insert
/// Insert `node` at the head of bucket `(tag & 0x1FF) + 1` of the table at
/// `this`, link it doubly, stamp the tag's low word at `node+0x10`, and bump
/// the count at `this+0xE04`. Returns the previous head.
export!(thiscall, rw_008e6d70(this: *mut u8, node: *mut u8, tag: u32) -> u32 {
    unsafe {
        let bucket =
            this.add(((tag & 0x1FF) + 1) as usize * 4) as *mut u32;
        let head = read_unaligned(bucket);
        write_unaligned(node as *mut u32, head);
        write_unaligned(node.add(4) as *mut u32, bucket as u32);
        if head != 0 {
            write_unaligned((head + 4) as *mut u32, node as u32);
        }
        write_unaligned(bucket, node as u32);
        write_unaligned(node.add(0x10) as *mut u16, tag as u16);
        let count = this.add(0xE04) as *mut u32;
        write_unaligned(count, read_unaligned(count).wrapping_add(1));
        head
    }
});

