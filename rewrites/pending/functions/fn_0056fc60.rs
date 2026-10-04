// original: 0x0056fc60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_124, player_schema::LeaderboardInfo, 10>::vf12
use lf_checker_rt::{callee_fastcall, export};
/// Find the leaderboard entry selected by `index`, or -1.
///
/// Fetches the descriptor, reads the key at `index` from its value array,
///
/// then scans the id array for that key and returns the first matching
///
/// position. A failed fetch, a -1 key, an empty array or no match yields -1.

export!(stdcall, rw_56fc60(index: u32) -> u32 {
    const LEADERBOARD_ID: u32 = 0x147;
    const NOT_FOUND: u32 = 0xFFFF_FFFF;
    let mut query = [0u32; 6];
    let ok = callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32) as u8;
    if ok == 0 {
        return NOT_FOUND;
    }
    let base = query[5];
    let key = unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read() };
    if key == NOT_FOUND {
        return NOT_FOUND;
    }
    let count = query[1];
    if count == 0 {
        return NOT_FOUND;
    }
    let ids = query[2];
    let mut i: u32 = 0;
    while i < count {
        let v = unsafe { (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if v == key {
            return i;
        }
        i = i.wrapping_add(1);
    }
    NOT_FOUND
});
