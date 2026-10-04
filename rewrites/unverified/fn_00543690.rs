// original: 0x00543690 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_6, player_schema::LeaderboardInfo, 10>::vf7
/// Fetch one leaderboard column value by row index.
///
/// Calls the leaderboard-info callee (fastcall slot 0) with this board's
/// numeric id in ECX and a scratch info block in EDX. When the callee
/// reports failure (low byte clear) the result is NOT_FOUND. Otherwise the
/// info block's value-array pointer (at +0x10) is read and the row at
/// `index` returned. The object pointer in ECX is unused. Original is
/// thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_00543690(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADER_ID: u32 = 0xA6;
        const INFO_VALUES: u32 = 0x10;
        const INFO_CALLEE: u32 = 0;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADER_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let values = info[(INFO_VALUES / 4) as usize];
        (values as *const u32).add(index as usize).read_unaligned()
    }
});
