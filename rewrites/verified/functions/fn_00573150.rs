// original: 0x00573150 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_136, player_schema::LeaderboardInfo, 10>::vf13

/// Ranked-race leaderboard value lookup: fetch this board's id
/// array, find `key` in it (a linear search over `count` entries),
/// and return the auxiliary word at the same position.
///
/// `LEADERBOARD_ID` selects the board; the descriptor's words hold the
/// id count (`DESC_COUNT`), the id array (`DESC_IDS`) and the auxiliary
/// table (`DESC_AUX`). `this` is ignored. Returns -1 when the fetch
/// fails, the count is not positive, or the key is not in the array
/// (the count comparison is signed). On a hit the original re-checks
/// the index against -1, which can never match; that check is kept.
/// Returns the auxiliary word, not the index (unlike vf6).
///
/// Original: 0x00573150 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00573150(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x160;
        const DESC_COUNT: usize = 3;
        const DESC_IDS: usize = 4;
        const DESC_AUX: usize = 5;
        const MISSING: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        desc[DESC_COUNT] = 0;
        desc[DESC_IDS] = 0;
        desc[DESC_AUX] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return MISSING;
        }
        let count = desc[DESC_COUNT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let ids = desc[DESC_IDS];
        let mut i = 0u32;
        loop {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == key {
                if i == MISSING {
                    return MISSING;
                }
                let aux = desc[DESC_AUX];
                return rd32(aux.wrapping_add(i.wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return MISSING;
            }
        }
    }
});
