// original: 0x00524aa0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race52NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Finds one leaderboard row's key inside this leaderboard's key list.
///
/// Asks the registry for id LEADERBOARD_ID's lists (count at +4,
/// searched list at +8, indexed list at +20), takes `index`'s entry
/// from the indexed list and returns its position in the searched
/// list, or -1 on lookup failure, an empty marker (-1), an empty
/// list or a miss. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00524aa0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x41;
        const LOOKUP: u32 = 1;
        const COUNT: usize = 1;
        const SEARCH: usize = 2;
        const INDEXED: usize = 5;
        const EMPTY: u32 = 0xFFFF_FFFF;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = buf[COUNT];
        let arr2 = buf[SEARCH] as u32;
        let arr1 = buf[INDEXED] as u32;
        let want = (arr1.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if want == EMPTY {
            return NOT_FOUND;
        }
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            let v = (arr2.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == want {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
