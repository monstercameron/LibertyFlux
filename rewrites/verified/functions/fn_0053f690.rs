// original: 0x0053F690 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_CoopBombBase_BG_TIME, player_schema::LeaderboardInfo, 10>::vf12

/// Ask the selector-specific helper to fill a six-word lookup record. Use its unsigned candidate count, candidate-key array, and position-to-key array to find the selected key in the candidate list. Return the first matching position, or `u32::MAX` when the helper declines, the selected key is the sentinel, the count is zero, or no candidate matches. The receiver is unused.
lf_checker_rt::export!(thiscall, rw_0053f690(_this: u32, lookup_index: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0x32;
        const NO_MATCH: u32 = u32::MAX;
#[repr(C)]
    struct KeyLookup {
        _word_0: u32,
        candidate_count: u32,
        candidates: u32,
        _words_3_4: [u32; 2],
        keys_by_index: u32,
    }
    let mut info = KeyLookup { _word_0: 0, candidate_count: 0, candidates: 0,
        _words_3_4: [0; 2], keys_by_index: 0 };
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, &mut info as *mut _ as u32);
    if helper_result as u8 == 0 { return NO_MATCH; }
        let selected_key = (info.keys_by_index as *const u32).wrapping_add(lookup_index as usize).read_unaligned();
        if selected_key == NO_MATCH { return NO_MATCH; }
        let candidate_count = info.candidate_count;
    if candidate_count == 0 { return NO_MATCH; }
        let candidates = info.candidates as *const u32;
        for position in 0..(candidate_count as usize) {
            if candidates.wrapping_add(position).read_unaligned() == selected_key { return position as u32; }
        }
        NO_MATCH
    }
});
