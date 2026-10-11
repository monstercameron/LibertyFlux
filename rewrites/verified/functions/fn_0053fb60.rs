// original: 0x0053FB60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopDrugFactory_BG_TIME, player_schema::LeaderboardInfo, 10>::vf13

/// Ask the selector-specific helper to fill a six-word record containing a signed candidate count, a key array, and a parallel result array. Return the result paired with the first key equal to `expected_key`; nonpositive counts, a declined helper, or no match return `u32::MAX`. The receiver is unused.
lf_checker_rt::export!(thiscall, rw_0053fb60(_this: u32, expected_key: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0x4D;
        const NO_MATCH: u32 = u32::MAX;
#[repr(C)]
    struct CandidateResults {
        _words_0_2: [u32; 3],
        candidate_count: i32,
        keys: u32,
        results: u32,
    }
    let mut info = CandidateResults { _words_0_2: [0; 3], candidate_count: 0,
        keys: 0, results: 0 };
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, &mut info as *mut _ as u32);
    if helper_result as u8 == 0 { return NO_MATCH; }
        let candidate_count = info.candidate_count;
    if candidate_count <= 0 { return NO_MATCH; }
        let keys = info.keys as *const u32;
        for position in 0..(candidate_count as usize) {
            if keys.wrapping_add(position).read_unaligned() == expected_key {
                let result = (info.results as *const u32).wrapping_add(position).read_unaligned();
                return result;
            }
        }
        NO_MATCH
    }
});
