// original: 0x0057ce60 leaderboard_172_vf12_rowpos
/// Position of board 172's row value in the short list (virtual slot 12).
///
/// Fetches the board's columns through the shared leaderboard helper, reads
/// the row value at `index` from the long column (-1 for a missing row ends
/// the search), then scans the short column for that value and returns its
/// position, or -1 when the lookup fails or nothing matches.
export!(thiscall, rw_0057ce60(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x184, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let col_a = frame[5];
        let v =
            (col_a.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if v == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = frame[1];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let col_b = frame[2];
        let mut i: u32 = 0;
        while i < count {
            let w = (col_b.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if w == v {
                return i;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
