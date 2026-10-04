// original: 0x00583750 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196, player_schema::LeaderboardInfo, 10>::vf12
#[repr(C)]
struct Col12 {
    _pad0: u32,
    count: u32, // +0x04
    arr2: u32,   // +0x08
    _pad: [u32; 2],
    arr: u32,    // +0x14
}

/// rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_196, player_schema::LeaderboardInfo, 10>::vf12
///
/// Resolves row `index` of the leaderboard's first column to its key,
/// then returns the position of that key in the second column, or -1
/// (as u32) when the table is unavailable, the key is -1, or the
/// second column holds no such key.
lf_checker_rt::export!(stdcall, rw_00583750(index: u32) -> u32 {
    const LEADERBOARD: u32 = 0x19c;
    let mut info = Col12 { _pad0: 0, count: 0, arr2: 0, _pad: [0; 2], arr: 0 };
    let here = core::ptr::addr_of_mut!(info) as u32;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, here);
    if ok as u8 == 0 {
        return u32::MAX;
    }
    let key = unsafe { (info.arr.wrapping_add(index.wrapping_mul(4)) as *const u32).read() };
    if key == u32::MAX {
        return u32::MAX;
    }
    if info.count == 0 {
        return u32::MAX;
    }
    let arr2 = info.arr2;
    let mut i = 0u32;
    while i < info.count {
        let k = unsafe { (arr2.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if k == key {
            return i;
        }
        i += 1;
    }
    u32::MAX
});
