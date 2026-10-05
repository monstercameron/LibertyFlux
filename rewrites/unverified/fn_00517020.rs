// original: 0x00517020 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race2NoHolds, player_schema::LeaderboardInfo, 10>::vf12
/// Leaderboard reverse lookup: find `table_a[index]` inside table B.
///
/// Calls the board-info fetcher (fastcall: ECX = board id 0x5f, EDX = out
/// struct) which reports success in AL and fills the unsigned count at `+4`,
/// the table-B pointer at `+8` and the table-A pointer at `+20`. On fetcher
/// failure returns -1. The key is table A at `index`; a key of -1, a zero
/// count, or a failed linear scan of table B (unsigned bound) all return -1,
/// otherwise the scan returns the first matching index. Stdcall, one argument.
lf_checker_rt::export!(stdcall, rw_00517020(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x5f;
        const COUNT_SLOT: usize = 1;
        const TABLE_B_SLOT: usize = 2;
        const TABLE_A_SLOT: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, BOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0xFFFF_FFFF;
        }
        let key = rd32(info[TABLE_A_SLOT].wrapping_add(index.wrapping_mul(4)));
        if key == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = info[COUNT_SLOT];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let table_b = info[TABLE_B_SLOT];
        let mut i = 0u32;
        loop {
            if rd32(table_b.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return 0xFFFF_FFFF;
            }
        }
    }
});
