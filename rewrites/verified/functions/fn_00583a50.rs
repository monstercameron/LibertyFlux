// original: 0x00583a50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196, player_schema::LeaderboardInfo, 10>::vf8
#[repr(C)]
struct CellArr14 {
    _pad: [u32; 5],
    arr: u32,    // +0x14
}

/// rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196, player_schema::LeaderboardInfo, 10>::vf8
///
/// Classifies row `index` of the leaderboard's cell column: the kind
/// reported for the cell maps 1..=5 to sizes 4, 8, 8, 0, 4, and anything
/// else (including an unavailable table) to 0.
lf_checker_rt::export!(stdcall, rw_00583a50(index: u32) -> u32 {
    const LEADERBOARD: u32 = 0x19c;
    let mut info = CellArr14 { _pad: [0; 5], arr: 0 };
    let here = core::ptr::addr_of_mut!(info) as u32;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, here);
    if ok as u8 == 0 {
        return 0;
    }
    let cell = unsafe { (info.arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read() };
    let kind = lf_checker_rt::callee_thiscall!(2, u32, cell);
    if kind == u32::MAX {
        return 0;
    }
    match kind.wrapping_sub(1) {
        0 => 4,
        1 => 8,
        2 => 8,
        3 => 0,
        4 => 4,
        _ => 0,
    }
});
