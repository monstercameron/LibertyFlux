// original: 0x00593120 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_253, player_schema::LeaderboardInfo, 10>::vf13

/// Find `wanted` in the leaderboard key array and return the mapped value.
///
/// Calls the leaderboard-info callee for board `LEADERBOARD_ID` with a six-word
/// out struct; words `+3` (count, signed: `<= 0` means `NOT_FOUND`), `+4` (key
/// array) and `+5` (value array) are read afterwards. Returns the value at the
/// first matching key, or `NOT_FOUND` on callee failure (low byte zero), an
/// empty count, or no match. The original's `(an instruction of the original)` after a match is dead
/// (the index is always `>= 0`) and is not reproduced.
/// Original: stdcall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_00593120(wanted: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1d5;
        const INFO_CALLEE: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let count = info[3] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = info[4];
        let mut i = 0u32;
        loop {
            let k = (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if k == wanted {
                let values = info[5];
                return (values.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
