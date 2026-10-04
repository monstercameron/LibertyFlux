// original: 0x00582f00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_194, player_schema::LeaderboardInfo, 10>::vf13
#[repr(C)]
struct Col13 {
    _pad: [u32; 3],
    count: u32, // +0x0c
    keys: u32,   // +0x10
    vals: u32,   // +0x14
}

/// rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_194, player_schema::LeaderboardInfo, 10>::vf13
///
/// Looks up `key` in the leaderboard's key column and returns the value
/// from the parallel value column, or -1 (as u32) when the table is
/// unavailable, empty, or holds no such key.
lf_checker_rt::export!(stdcall, rw_00582f00(key: u32) -> u32 {
    const LEADERBOARD: u32 = 0x19a;
    let mut info = Col13 { _pad: [0; 3], count: 0, keys: 0, vals: 0 };
    let here = core::ptr::addr_of_mut!(info) as u32;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, here);
    if ok as u8 == 0 {
        return u32::MAX;
    }
    if (info.count as i32) <= 0 {
        return u32::MAX;
    }
    let keys = info.keys;
    let mut i = 0u32;
    while i < info.count {
        let k = unsafe { (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        if k == key {
            let vals = info.vals;
            return unsafe { (vals.wrapping_add(i.wrapping_mul(4)) as *const u32).read() };
        }
        i += 1;
    }
    u32::MAX
});
