// original: 0x00583ac0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196, player_schema::LeaderboardInfo, 10>::vf9
#[repr(C)]
struct CellArr14 {
    _pad: [u32; 5],
    arr: u32,    // +0x14
}

/// rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196, player_schema::LeaderboardInfo, 10>::vf9
///
/// Ranks row `index` of the leaderboard's cell column: the kind
/// reported for the cell maps 1..=5 to 0, 1, 3, -1, 2, and anything
/// else (including an unavailable table) to -1.
lf_checker_rt::export!(stdcall, rw_00583ac0(index: u32) -> u32 {
    const LEADERBOARD: u32 = 0x19c;
    let mut info = CellArr14 { _pad: [0; 5], arr: 0 };
    let here = core::ptr::addr_of_mut!(info) as u32;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, here);
    if ok as u8 == 0 {
        return u32::MAX;
    }
    let cell = unsafe { (info.arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read() };
    let kind = lf_checker_rt::callee_thiscall!(2, u32, cell);
    if kind == u32::MAX {
        return u32::MAX;
    }
    match kind.wrapping_sub(1) {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => u32::MAX,
        4 => 2,
        _ => u32::MAX,
    }
});
