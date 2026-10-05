// original: 0x005714a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_129, player_schema::LeaderboardInfo, 10>::vf6

/// Key search for the race-129 leaderboard table.
///
/// Calls the shared fetch routine with table id 0x14c and a 6-word scratch
/// descriptor; `COUNT` at `+0x0c` receives the entry count and `KEYS` at `+0x10`
/// a pointer to the key array. Returns the first index whose key equals `key`,
/// or -1 when the fetch fails (low answer byte clear), the signed count is not
/// positive, or no key matches. stdcall, one stack argument, pops 4.
/// Leaderboard-info fetch: `id` selects this race's table, `info` receives the
/// column descriptor the fetch fills in. Only the low byte of the answer is
/// tested (`(an instruction of the original)`); the upper bytes are ignored.
lf_checker_rt::export!(stdcall, rw_005714a0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x14c;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3; // descriptor offset +0x0c
        const KEYS_WORD: usize = 4; // descriptor offset +0x10
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let count = info[COUNT_WORD];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let keys = info[KEYS_WORD] as *const u32;
        let mut i: u32 = 0;
        while (i as i32) < (count as i32) {
            if keys.add(i as usize).read_unaligned() == key {
                return i;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
