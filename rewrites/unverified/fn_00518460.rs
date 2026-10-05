// original: 0x00518460 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race6NoHolds, player_schema::LeaderboardInfo, 10>::vf7
/// Leaderboard id fetch: return the id stored at slot `index`.
///
/// Calls the board-info fetcher (fastcall: ECX = board id 0x63, EDX = out
/// struct) which reports success in AL and fills the id-table pointer at
/// `+16`. On fetcher failure returns -1, otherwise the table word at `index`
/// (no bounds check in the original). Stdcall, one stack argument.
lf_checker_rt::export!(stdcall, rw_00518460(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x63;
        const TABLE_SLOT: usize = 4;
        let mut info = [0u32; 5];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, BOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0xFFFF_FFFF;
        }
        let table = info[TABLE_SLOT];
        (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
