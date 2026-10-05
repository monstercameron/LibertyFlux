// original: 0x005930b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_253, player_schema::LeaderboardInfo, 10>::vf12

/// Look up one leaderboard row by index, then find that row's key in the key array.
///
/// `index` selects a row. Calls the leaderboard-info callee for board
/// `LEADERBOARD_ID` with a six-word out struct; words `+1` (count), `+2` (key
/// array) and `+5` (row array) are read afterwards. Returns `NOT_FOUND` when
/// the callee reports failure (low byte zero), when the row key is `NOT_FOUND`,
/// when the count is zero, or when the key is absent; otherwise the unsigned
/// index of the first matching key. The scan bound is unsigned (`jb`), so a
/// huge count scans past the array exactly like the original.
/// Original: stdcall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_005930b0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d5;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let rows = info[5];
        let key = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[1];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = info[2];
        let mut i = 0u32;
        loop {
            let k = (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if k == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
