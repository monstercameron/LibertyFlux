// original: 0x0057d790 leaderboard_174_vf13_map
/// Mapped value for `key` in board 174's columns (virtual slot 13).
///
/// Same behaviour as the board-172 slot: count/two-column lookup, scan of
/// the first column, second column's entry at the match, -1 otherwise.
export!(thiscall, rw_0057d790(_this: u32, key: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x186, frame.as_mut_ptr() as u32);
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
