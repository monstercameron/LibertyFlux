// original: 0x00595B10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_262, player_schema::LeaderboardInfo, 10>::vf8
/// Ranked-leaderboard classified lookup (vf8 slot, race 262).
///
/// Looks up `table[index]` through the leaderboard-0x1da helper, classifies
/// the entry with the rank helper, and maps the rank to a size: ranks 1 and
/// 5 give 4, ranks 2 and 3 give 8, anything else gives 0. A failed helper
/// call also gives 0.
export!(stdcall, rw_00595b10(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1da;
        let mut out = [0u32; 6];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let table = out[5];
        let entry = core::ptr::read(table.wrapping_add(index.wrapping_mul(4)) as *const u32);
        let rank: u32 = callee_thiscall!(2, u32, entry);
        match rank {
            1 | 5 => 4,
            2 | 3 => 8,
            _ => 0,
        }
    }
});
