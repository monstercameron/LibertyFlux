// original: 0x00572cf0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_135, player_schema::LeaderboardInfo, 10>::vf13

/// Mapped value lookup for the race-135 leaderboard table.
///
/// Calls the shared fetch routine with table id 0x15f and a 6-word scratch
/// descriptor; `COUNT` at `+0x0c` receives the entry count, `KEYS` at `+0x10` a
/// pointer to the key array and `MAPPED` at `+0x14` a pointer to the parallel
/// value array. Returns `mapped[i]` for the first index `i` whose key equals
/// `key`, or -1 when the fetch fails (low answer byte clear), the signed count
/// is not positive, or no key matches. The original also compares the found
/// index against -1 before indexing; that branch is dead (the index is always
/// in range) and is not reproduced. stdcall, one stack argument, pops 4.
/// Leaderboard-info fetch: `id` selects this race's table, `info` receives the
/// column descriptor the fetch fills in. Only the low byte of the answer is
/// tested (`(an instruction of the original)`); the upper bytes are ignored.
lf_checker_rt::export!(stdcall, rw_00572cf0(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x15f;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 3; // descriptor offset +0x0c
        const KEYS_WORD: usize = 4; // descriptor offset +0x10
        const MAPPED_WORD: usize = 5; // descriptor offset +0x14
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
                let mapped = info[MAPPED_WORD] as *const u32;
                return mapped.add(i as usize).read_unaligned();
            }
            i += 1;
        }
        NOT_FOUND
    }
});
