// original: 0x00517db0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race5NoHolds, player_schema::LeaderboardInfo, 10>::vf13
/// Leaderboard joined fetch: find `key` in table A, return table B there.
///
/// Calls the board-info fetcher (fastcall: ECX = board id 0x62, EDX = out
/// struct) which reports success in AL and fills the signed count at `+12`,
/// the table-A pointer at `+16` and the table-B pointer at `+20`. On fetcher
/// failure, or when the count is not positive, returns -1. Otherwise scans
/// table A for `key` and returns table B at the first matching index, or -1
/// when absent. (The original re-tests the found index against -1; that
/// branch is dead since a found index is always in range.) Stdcall, one arg.
lf_checker_rt::export!(stdcall, rw_00517db0(key: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x62;
        const COUNT_SLOT: usize = 3;
        const TABLE_A_SLOT: usize = 4;
        const TABLE_B_SLOT: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, BOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0xFFFF_FFFF;
        }
        let count = info[COUNT_SLOT] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let table_a = info[TABLE_A_SLOT];
        let mut i = 0u32;
        let idx = loop {
            if rd32(table_a.wrapping_add(i.wrapping_mul(4))) == key {
                break i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return 0xFFFF_FFFF;
            }
        };
        rd32(info[TABLE_B_SLOT].wrapping_add(idx.wrapping_mul(4)))
    }
});
