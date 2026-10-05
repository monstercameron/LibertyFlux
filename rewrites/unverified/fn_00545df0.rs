// original: 0x00545df0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_15, player_schema::LeaderboardInfo, 10>::vf7

/// Row id of one leaderboard entry by position, or -1 on failure.
///
/// Fetches the board's id table through the info callee (id `0xd2`, table
/// pointer written at frame offset `+0x10`) and returns `table[index]`.
/// Returns -1 when the callee reports failure (low byte zero). No bounds
/// check: a wild index faults, like the original. stdcall, one stack
/// argument; entry ECX ignored.
lf_checker_rt::export!(stdcall, rw_00545df0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xd2;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xff) == 0 {
            return NOT_FOUND;
        }
        ((info[4].wrapping_add(index.wrapping_mul(4))) as *const u32)
            .read_unaligned()
    }
});
