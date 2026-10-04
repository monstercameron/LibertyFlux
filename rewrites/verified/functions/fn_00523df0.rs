// original: 0x00523df0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race49NoHolds, player_schema::LeaderboardInfo, 10>::vf13

/// Maps a column value to its parallel-table entry for this leaderboard.
///
/// Asks the registry for id LEADERBOARD_ID's key list (count at +12,
/// keys at +16, values at +20), finds `value` in the keys and returns
/// the value at the same index, or -1 on lookup failure, empty list
/// or miss. stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00523df0(value: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x2d;
        const LOOKUP: u32 = 1;
        const COUNT: usize = 3;
        const KEYS: usize = 4;
        const VALS: usize = 5;
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
        let keys = buf[KEYS] as u32;
        let vals = buf[VALS] as u32;
        let mut i = 0i32;
        while i < count {
            let k = (keys.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if k == value {
                return (vals.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            }
            i += 1;
        }
        NOT_FOUND
    }
});
