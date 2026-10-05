// original: 0x00571960 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_130, player_schema::LeaderboardInfo, 10>::vf7

/// Column value lookup for the race-130 leaderboard table.
///
/// Calls the shared fetch routine with table id 0x14d and a 5-word scratch
/// descriptor; field `VALUES` at `+0x10` receives a pointer to the column's
/// value array. Returns `values[index]`, or -1 when the fetch reports failure
/// (low byte of its answer clear). The index is used unchecked, exactly as in
/// the original. stdcall, one stack argument, pops 4.
/// Leaderboard-info fetch: `id` selects this race's table, `info` receives the
/// column descriptor the fetch fills in. Only the low byte of the answer is
/// tested (`(an instruction of the original)`); the upper bytes are ignored.
lf_checker_rt::export!(stdcall, rw_00571960(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x14d;
        const FETCH_CALLEE: u32 = 1;
        const VALUES_WORD: usize = 4; // descriptor offset +0x10
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return NOT_FOUND;
        }
        let values = info[VALUES_WORD] as *const u32;
        values.add(index as usize).read_unaligned()
    }
});
