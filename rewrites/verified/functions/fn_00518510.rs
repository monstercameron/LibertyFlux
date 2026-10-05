// original: 0x00518510 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race6NoHolds, player_schema::LeaderboardInfo, 10>::vf9
/// Leaderboard class tag: classify the entry id stored at slot `index`.
///
/// Calls the board-info fetcher (fastcall: ECX = board id 0x63, EDX = out
/// struct) which reports success in AL and fills the id-table pointer at
/// `+20`. The table word at `index` goes in ECX to the classifier callee; its
/// answer maps through a five-way jump table (1->0, 2->1, 3->3, 4->-1, 5->2, anything else (including -1) -> -1). Fetcher failure maps
/// to the same default as an unlisted answer. Stdcall, one stack argument.
lf_checker_rt::export!(stdcall, rw_00518510(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x63;
        const TABLE_SLOT: usize = 5;
        let mut info = [0u32; 6];
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, BOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0xFFFF_FFFF;
        }
        let table = info[TABLE_SLOT];
        let v = (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, v);
        match r {
            1 => 0, 2 => 1, 3 => 3, 4 => 0xFFFF_FFFF, 5 => 2, _ => 0xFFFF_FFFF,
        }
    }
});
