// original: 0x005726c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_133, player_schema::LeaderboardInfo, 10>::vf8

/// Column width class for the race-133 leaderboard table.
///
/// Calls the shared fetch routine with table id 0x150 and a 6-word scratch
/// descriptor; `VALUES` at `+0x14` receives a pointer to the value array. The
/// value at `index` (unchecked, as in the original) is classified by a second
/// routine; its answer maps through a five-entry jump table to a width: 1 and 5
/// give 4, 2 and 3 give 8, 4 gives 0, anything else (including a fetch failure,
/// a classifier answer of -1, or 0 and 6 and above) gives 0. stdcall, one stack
/// argument, pops 4.
/// Leaderboard-info fetch: `id` selects this race's table, `info` receives the
/// column descriptor the fetch fills in. Only the low byte of the answer is
/// tested (`(an instruction of the original)`); the upper bytes are ignored.
lf_checker_rt::export!(stdcall, rw_005726c0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x150;
        const FETCH_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const VALUES_WORD: usize = 5; // descriptor offset +0x14
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return 0;
        }
        let values = info[VALUES_WORD] as *const u32;
        let v = values.add(index as usize).read_unaligned();
        let cls: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY_CALLEE, u32, v);
        match cls {
            1 | 5 => 4,
            2 | 3 => 8,
            _ => 0,
        }
    }
});
