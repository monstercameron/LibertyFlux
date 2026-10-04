// original: 0x0057d0c0 leaderboard_172_vf6_find
/// Position of `key` in board 172's id column (virtual slot 6).
///
/// Fetches the board's count and id column through the shared leaderboard
/// helper, then scans for the first entry equal to `key` and returns its
/// index, or -1 when the lookup fails, the count is not positive, or no
/// entry matches.
export!(thiscall, rw_0057d0c0(_this: u32, key: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x184, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let count = frame[3] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let col = frame[4];
        let mut i: i32 = 0;
        while i < count {
            let v = (col.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if v == key {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
