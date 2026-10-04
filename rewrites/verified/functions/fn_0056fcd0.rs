// original: 0x0056fcd0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_124, player_schema::LeaderboardInfo, 10>::vf13
use lf_checker_rt::{callee_fastcall, export};
/// Look up the value paired with `needle`, or -1.
///
/// Fetches the descriptor, scans the id array for `needle` and returns the
///
/// entry at the same position in the value array. A failed fetch, a
///
/// non-positive count or no match yields -1.

export!(stdcall, rw_56fcd0(needle: u32) -> u32 {
    const LEADERBOARD_ID: u32 = 0x147;
    const NOT_FOUND: u32 = 0xFFFF_FFFF;
    let mut query = [0u32; 6];
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
            let vals = query[5];
            return unsafe {
                (vals.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
            };
        }
        i += 1;
    }
    NOT_FOUND
});
