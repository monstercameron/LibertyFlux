// original: 0x00539FA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopDrugFactory, player_schema::LeaderboardInfo, 10>::vf13
/// Ask the selector-specific lookup helper for a candidate count and key array, then return the position of `expected_key`. The count is signed: zero and negative counts return `u32::MAX`; a positive count scans that many entries. The receiver is not read.
lf_checker_rt::export!(thiscall, rw_00539fa0(_receiver: u32, expected_key: u32) -> u32 {
    const NO_MATCH: u32 = u32::MAX;
    const SELECTOR: u32 = 0x2;
    #[repr(C)]
    struct CandidateList {
        _leading: [u32; 3],
        candidate_count: i32,
        candidates: *const u32,
        _unused_output: u32,
    }
    let mut output = CandidateList {
        _leading: [0; 3],
        candidate_count: 0,
        candidates: core::ptr::null(),
        _unused_output: 0,
    };
    let output_address = &mut output as *mut CandidateList as u32;
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, output_address);
    if helper_result & 0xFF == 0 || output.candidate_count <= 0 { return NO_MATCH; }
    for position in 0..output.candidate_count {
        if unsafe { *output.candidates.add(position as usize) } == expected_key {
            return position as u32;
        }
    }
    NO_MATCH
});
