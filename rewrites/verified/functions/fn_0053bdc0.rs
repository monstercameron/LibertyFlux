// original: 0x0053BDC0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamCarSteal_BG, player_schema::LeaderboardInfo, 10>::vf12

/// Look up a leaderboard row key by position, then find that key in the row list.
///
/// Queries the leaderboard metadata table TABLE_ID through the info callee,
/// which fills a six-word descriptor: word 1 is the row count, word 2 points
/// at the array of row keys, word 5 points at the position-to-key map. The
/// argument is a position; the function reads the key stored for it and
/// returns the index of that key in the row array, or NOT_FOUND (-1) when
/// the query fails, the position holds no key (-1), the table is empty, or
/// the key is not listed. The count comparison is unsigned, the scan linear.
///
/// Original: thiscall with one stack word; ECX (this) is unused.

lf_checker_rt::export!(thiscall, rw_0053bdc0(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_ID: u32 = 0x38;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, TABLE_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let key = (info[5] as *const u32).wrapping_add(index as usize).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[1];
        if count == 0 {
            return NOT_FOUND;
        }
        let items = info[2] as *const u32;
        let mut i = 0u32;
        loop {
            if items.wrapping_add(i as usize).read_unaligned() == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
