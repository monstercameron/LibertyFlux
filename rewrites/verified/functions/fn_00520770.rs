// original: 0x00520770 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race36NoHolds, player_schema::LeaderboardInfo, 10>::vf7

/// Leaderboard item fetch: the entry at a position in this board's list.
///
/// Asks the board-info callee for leaderboard `0x79`'s entry list and
/// returns the item at `index` with no bounds check (a wild index faults,
/// as the original does), or all-ones when the lookup itself fails.
///
/// Original: 0x00520770 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00520770(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0x79;
        const INFO_CALLEE: u32 = 1;
        const INFO_ITEMS: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let items = info[INFO_ITEMS];
        rd32(items.wrapping_add(index.wrapping_mul(4)))
    }
});
