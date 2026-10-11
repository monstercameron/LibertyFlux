// original: 0x0053FD50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopDrugFactory_BG_TIME, player_schema::LeaderboardInfo, 10>::vf6

/// Ask the selector-specific helper to fill a six-word record containing a signed candidate count and key array. Return the position of `expected_key`; nonpositive counts, a declined helper, or no match return `u32::MAX`. The receiver is unused.
lf_checker_rt::export!(thiscall, rw_0053fd50(_this: u32, expected_key: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0x4D;
        const NO_MATCH: u32 = u32::MAX;
#[repr(C)]
    struct CandidateKeys {
        _words_0_2: [u32; 3],
        candidate_count: i32,
        keys: u32,
        _word_5: u32,
    }
    let mut info = CandidateKeys { _words_0_2: [0; 3], candidate_count: 0,
        keys: 0, _word_5: 0 };
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, &mut info as *mut _ as u32);
    if helper_result as u8 == 0 { return NO_MATCH; }
        let candidate_count = info.candidate_count;
    if candidate_count <= 0 { return NO_MATCH; }
        let keys = info.keys as *const u32;
        for position in 0..(candidate_count as usize) {
            if keys.wrapping_add(position).read_unaligned() == expected_key {
                return position as u32;
            }
        }
        NO_MATCH
    }
});
