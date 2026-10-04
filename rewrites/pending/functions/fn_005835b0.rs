// original: 0x005835b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_195, player_schema::LeaderboardInfo, 10>::vf7
#[repr(C)]
struct CellArr {
    _pad: [u32; 4],
    arr: u32,    // +0x10
    _tail: u32,
}

/// rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_195, player_schema::LeaderboardInfo, 10>::vf7
///
/// Returns row `index` of the leaderboard's cell column, or -1 (as u32)
/// when the table is unavailable. No bounds check, like the original.
lf_checker_rt::export!(stdcall, rw_005835b0(index: u32) -> u32 {
    const LEADERBOARD: u32 = 0x19b;
    let mut info = CellArr { _pad: [0; 4], arr: 0, _tail: 0 };
    let here = core::ptr::addr_of_mut!(info) as u32;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, here);
    if ok as u8 == 0 {
        return u32::MAX;
    }
    unsafe { (info.arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read() }
});
