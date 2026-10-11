// original: 0x0053A5E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopDrugFactory_BG, player_schema::LeaderboardInfo, 10>::vf6
/// Ask the selector-specific lookup helper for a signed candidate count and key array, then return the position of `expected_key`. Zero and negative counts return `u32::MAX`; otherwise the scan is bounded by the count. The receiver is not read.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_0053a5e0(_receiver: u32, expected_key: u32) -> u32 {
    const NO_MATCH: u32 = u32::MAX;
    const SELECTOR: u32 = 0x29;
    #[repr(C)]
    struct CandidateList {
        _leading: [u32; 3],
        candidate_count: i32,
        candidates: *const u32,

    }
    let mut output = CandidateList {
        _leading: [0; 3],
        candidate_count: 0,
        candidates: core::ptr::null(),

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
}
