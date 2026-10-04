// original: 0x00952b30 find_block_by_tail_key
/// Walk the global block list and return the first data block whose last
/// word equals the key. When no block matches, the first block is returned.
/// A null list head faults reading near address zero, same as the original.
export!(cdecl, rw_00952b30(key: u32) -> u32 {
    unsafe {
        let head = *global::<u32>(0x011ff028);
        if head == 0 {
            return *((head + 4) as *const u32);
        }
        let mut node = head;
        loop {
            let data = *((node + 4) as *const u32);
            if *((data + 0x1c) as *const u32) == key {
                return data;
            }
            let next = *((node + 8) as *const u32);
            if next == 0 {
                return *((head + 4) as *const u32);
            }
            node = next;
        }
    }
});
