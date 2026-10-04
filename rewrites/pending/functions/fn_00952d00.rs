// original: 0x00952d00 copy_next_matching_block
/// Walk the global block list for the first data block whose words at 0xc
/// and 0x14 match the argument block's, then copy the *next* block's 32
/// bytes over the argument block (or the matching block's own bytes when it
/// is last) and return the address the bytes came from. Returns zero when
/// nothing matches or the list is empty.
export!(cdecl, rw_00952d00(arg: *mut u32) -> u32 {
    unsafe {
        let head = *global::<u32>(0x011ff028);
        if head == 0 {
            return 0;
        }
        let key1 = *arg.add(3);
        let key2 = *arg.add(5);
        let mut node = head;
        loop {
            let data = *((node + 4) as *const u32);
            let blk = data as *const u32;
            if *blk.add(3) == key1 && *blk.add(5) == key2 {
                let next = *((node + 8) as *const u32);
                let src = if next == 0 {
                    data
                } else {
                    *((next + 4) as *const u32)
                };
                let s = src as *const u32;
                let mut i = 0;
                while i < 8 {
                    *arg.add(i) = *s.add(i);
                    i += 1;
                }
                return src;
            }
            let next = *((node + 8) as *const u32);
            if next == 0 {
                return 0;
            }
            node = next;
        }
    }
});
