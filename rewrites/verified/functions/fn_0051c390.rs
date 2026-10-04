// original: 0x0051c390 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race21NoHolds, player_schema::LeaderboardInfo, 10>::vf13

/// Find a value's row in one column and read the same row of another.
///
/// Asks the table helper (callee 1) for leaderboard 0x72: row count at
/// struct +0x0C (signed: not positive means empty), first-column base at
/// +0x10, second-column base at +0x14. Linear-searches `value` in the
/// first column and returns the second column's word at the found row.
/// Returns -1 when the helper fails, the count is not positive, or the
/// value is absent. (The original re-checks the found row against -1;
/// unreachable since rows are non-negative, so omitted.) stdcall.
lf_checker_rt::export!(stdcall, rw_0051c390(value: u32) -> u32 {
    unsafe {
        const LEADERBOARD: u32 = 0x72;
        const COUNT: usize = 3; // +0x0C, signed
        const COL1: usize = 4; // +0x10
        const COL2: usize = 5; // +0x14
        let mut info = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0xFFFF_FFFF;
        }
        let count = info[COUNT] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let base = info[COL1];
        let mut i = 0i32;
        loop {
            let w = (base.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if w == value {
                let col2 = info[COL2];
                return (col2.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            i = i.wrapping_add(1);
            if !(i < count) {
                return 0xFFFF_FFFF;
            }
        }
    }
});
