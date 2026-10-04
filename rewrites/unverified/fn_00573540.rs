// original: 0x00573540 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_137, player_schema::LeaderboardInfo, 10>::vf12

/// Ranked-race leaderboard reverse lookup: fetch this board's row
/// table, read the row at `index`, and return its position in the
/// board's id array (a linear search over `count` entries).
///
/// `LEADERBOARD_ID` selects the board; the descriptor's words hold the
/// id count (`DESC_COUNT`), the id array (`DESC_IDS`) and the row table
/// (`DESC_TABLE`). `this` is ignored. Returns -1 when the fetch fails,
/// the row is the -1 sentinel, the count is zero, or the row is not in
/// the array (the count comparison is unsigned).
///
/// Original: 0x00573540 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00573540(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x161;
        const DESC_COUNT: usize = 1;
        const DESC_IDS: usize = 2;
        const DESC_TABLE: usize = 5;
        const MISSING: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        desc[DESC_COUNT] = 0;
        desc[DESC_IDS] = 0;
        desc[DESC_TABLE] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return MISSING;
        }
        let table = desc[DESC_TABLE];
        let want = rd32(table.wrapping_add(index.wrapping_mul(4)));
        if want == MISSING {
            return MISSING;
        }
        let count = desc[DESC_COUNT];
        if count == 0 {
            return MISSING;
        }
        let ids = desc[DESC_IDS];
        let mut i = 0u32;
        loop {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return MISSING;
            }
        }
    }
});
