// original: 0x0056ff20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_124, player_schema::LeaderboardInfo, 10>::vf7
use lf_checker_rt::{callee_fastcall, export};
/// Read one entry of this leaderboard's id array by index.
///
/// Fetches the leaderboard descriptor and returns the entry at `index`.
///
/// Returns -1 when the fetch fails.

export!(stdcall, rw_56ff20(index: u32) -> u32 {
    const LEADERBOARD_ID: u32 = 0x147;
    let mut query = [0u32; 5];
    let ok = callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32) as u8;
    if ok == 0 {
        return 0xFFFF_FFFF;
    }
    let base = query[4];
    unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read() }
});
