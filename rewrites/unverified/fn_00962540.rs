// original: 0x00962540 bucket_reclaim
/// Reclaim a bucket from the 12-entry allocatorbuddy table.
///
/// Hashes the seed byte at 0x120F298 modulo the bucket count at 0x11F6FFB,
/// walks forward while the state table at 0x11F6FF0 says the bucket is
/// busy, then for buckets below 12 clears the winning bucket's state and
/// its target byte through the pointer table at 0x11F6F7C. Returns the
/// pointer on the reclaim path, the last quotient otherwise.
export!(cdecl, rw_00962540() -> u32 {
    unsafe {
        let seed = *global::<u8>(0x120F298) as u32;
        let count = *global::<u8>(0x11F6FFB) as u32;
        let state = relocated(0x11F6FF0) as *const u8;
        let mut quot = (seed + 1) / count;
        let mut idx = (seed + 1) % count;
        loop {
            let s = *state.add(idx as usize);
            if s == 2 || s == 1 {
                break;
            }
            quot = (idx + 1) / count;
            idx = (idx + 1) % count;
            if *state.add(idx as usize) == 2 {
                break;
            }
        }
        if idx >= 0xB {
            return quot;
        }
        let target = *((relocated(0x11F6F7C) as *const u32).add(idx as usize));
        *((relocated(0x11F6FF0) as *mut u8).add(idx as usize)) = 0;
        *(target as *mut u8) = 0;
        target
    }
});
