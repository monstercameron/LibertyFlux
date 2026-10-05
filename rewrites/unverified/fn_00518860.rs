// original: 0x00518860 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race7NoHolds, player_schema::LeaderboardInfo, 10>::vf6
/// Leaderboard index lookup: search this board's id table for `key`.
///
/// Calls the board-info fetcher (fastcall: ECX = board id 0x64, EDX = out
/// struct) which reports success in AL and fills count at `+12` and the id-table
/// pointer at `+16`. On fetcher failure, or when the signed count is not
/// positive, returns -1. Otherwise linearly scans the table and returns the
/// first index holding `key`, or -1 when absent. Stdcall, one stack argument.
lf_checker_rt::export!(stdcall, rw_00518860(key: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x64;
        const COUNT_SLOT: usize = 3;
        const TABLE_SLOT: usize = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 5];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, BOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0xFFFF_FFFF;
        }
        let count = info[COUNT_SLOT] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let table = info[TABLE_SLOT];
        let mut i = 0u32;
        loop {
            if rd32(table.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return 0xFFFF_FFFF;
            }
        }
    }
});
