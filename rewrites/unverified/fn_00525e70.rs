// original: 0x00525e70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race56NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Finds a column value in this leaderboard's column list.
///
/// Asks the leaderboard registry (callee) for id LEADERBOARD_ID's column
/// list into a frame buffer (count at +12, array pointer at +16), then
/// returns the index of `value` in it, or -1 when the lookup fails,
/// the list is empty, or the value is absent. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00525e70(value: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x44;
        const LOOKUP: u32 = 1;
        const COUNT: usize = 3;
        const ARRAY: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP, u32, LEADERBOARD_ID, buf.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = buf[COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let arr = buf[ARRAY] as u32;
        let mut i = 0i32;
        while i < count {
            let v = (arr.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if v == value {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
