// original: 0x0057ced0 leaderboard_172_vf13_map
/// Mapped value for `key` in board 172's columns (virtual slot 13).
///
/// Fetches the board's count and two columns through the shared leaderboard
/// helper, scans the first column for the first entry equal to `key`, and
/// returns the second column's entry at that position, or -1 when the lookup
/// fails, the count is not positive, or no entry matches.
export!(thiscall, rw_0057ced0(_this: u32, key: u32) -> u32 {
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
        let col_a = frame[4];
        let col_b = frame[5];
        let mut i: i32 = 0;
        while i < count {
            let v = (col_a.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if v == key {
                return (col_b.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
