// original: 0x00570320 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_125, player_schema::LeaderboardInfo, 10>::vf6
use lf_checker_rt::{callee_fastcall, export};
/// Find `needle` in this leaderboard's id array, or -1.
///
/// Fetches the leaderboard descriptor, then scans its id array for
///
/// `needle` and returns the first matching index. Returns -1 when the
///
/// fetch fails, when the count is not positive, or when no entry matches.

export!(stdcall, rw_570320(needle: u32) -> u32 {
    const LEADERBOARD_ID: u32 = 0x148;
    const NOT_FOUND: u32 = 0xFFFF_FFFF;
    let mut query = [0u32; 5];
    let ok = callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32) as u8;
    if ok == 0 {
        return NOT_FOUND;
    }
    let count = query[3] as i32;
    if count <= 0 {
        return NOT_FOUND;
    }
    let ids = query[4];
    let mut i: i32 = 0;
    while i < count {
        let v = unsafe {
            (ids.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
        };
        if v == needle {
            return i as u32;
        }
        i += 1;
    }
    NOT_FOUND
});
