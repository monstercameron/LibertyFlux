// original: 0x005733a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_136, player_schema::LeaderboardInfo, 10>::vf7

/// Ranked-race leaderboard entry lookup: fetch this board's row table,
/// then return the row at `index`.
///
/// `LEADERBOARD_ID` selects the board (one instantiation per race); the
/// shared fetch helper fills a six-word stack descriptor whose word
/// `DESC_TABLE` points at the row table. `this` is ignored. Returns -1
/// when the fetch fails, otherwise the table word at `index` (an
/// out-of-range index reads past the table exactly as the original does).
///
/// Original: 0x005733a0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005733a0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x160;
        const DESC_TABLE: usize = 4;
        const FAILED: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut desc = [0u32; 6];
        desc[DESC_TABLE] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, desc.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return FAILED;
        }
        let table = desc[DESC_TABLE];
        rd32(table.wrapping_add(index.wrapping_mul(4)))
    }
});
