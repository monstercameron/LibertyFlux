// original: 0x0056fb00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_123, player_schema::LeaderboardInfo, 10>::vf8
use lf_checker_rt::{callee_fastcall, callee_thiscall, export};
/// Classify one leaderboard entry: 4, 8 or 0.
///
/// Fetches the descriptor, reads the entry at `index`, asks the classifier
///
/// about it and maps the answer (1 to 5) to a width of 4, 8 or 0.
///
/// Anything outside 1 to 5, a failed fetch or a -1 answer yields 0.

export!(stdcall, rw_56fb00(index: u32) -> u32 {
    const LEADERBOARD_ID: u32 = 0x146;
    let mut query = [0u32; 6];
    let ok = callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32) as u8;
    if ok == 0 {
        return 0;
    }
    let base = query[5];
    let key = unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read() };
    let code: u32 = callee_thiscall!(2, u32, key);
    if code == 0xFFFF_FFFF {
        return 0;
    }
    match code.wrapping_sub(1) {
        0 => 4,
        1 | 2 => 8,
        4 => 4,
        _ => 0,
    }
});
