// original: 0x00571240 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_129, player_schema::LeaderboardInfo, 10>::vf12

/// Index remap for the race-129 leaderboard table.
///
/// Calls the shared fetch routine with table id 0x14c and a 6-word scratch
/// descriptor; `COUNT` at `+0x04` receives the needle count, `NEEDLES` at `+0x08`
/// a pointer to the needle array and `VALUES` at `+0x14` a pointer to the value
/// array. Takes `values[index]` (unchecked, as in the original); a value of -1
/// means missing and returns -1, otherwise the first needle equal to it is
/// searched with an unsigned bound and its index returned, or -1 when absent.
/// A failed fetch (low answer byte clear) or a zero count also returns -1.
/// stdcall, one stack argument, pops 4.
/// Leaderboard-info fetch: `id` selects this race's table, `info` receives the
/// column descriptor the fetch fills in. Only the low byte of the answer is
/// tested (`(an instruction of the original)`); the upper bytes are ignored.
lf_checker_rt::export!(stdcall, rw_00571240(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x14c;
        const FETCH_CALLEE: u32 = 1;
        const COUNT_WORD: usize = 1; // descriptor offset +0x04
        const NEEDLES_WORD: usize = 2; // descriptor offset +0x08
        const VALUES_WORD: usize = 5; // descriptor offset +0x14
        const MISSING: u32 = 0xffff_ffff;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return MISSING;
        }
        let values = info[VALUES_WORD] as *const u32;
        let v = values.add(index as usize).read_unaligned();
        if v == MISSING {
            return MISSING;
        }
        let count = info[COUNT_WORD];
        if count == 0 {
            return MISSING;
        }
        let needles = info[NEEDLES_WORD] as *const u32;
        let mut i: u32 = 0;
        while i < count {
            if needles.add(i as usize).read_unaligned() == v {
                return i;
            }
            i += 1;
        }
        MISSING
    }
});
