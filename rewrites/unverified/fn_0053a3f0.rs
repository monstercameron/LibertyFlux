// original: 0x0053A3F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopDrugFactory_BG, player_schema::LeaderboardInfo, 10>::vf13
/// Ask the selector-specific lookup helper for a signed candidate count, a key array, and a parallel result array. Return the result value paired with the first key equal to `expected_key`; zero or negative counts and no match return `u32::MAX`. The receiver is not read.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_0053a3f0(_receiver: u32, expected_key: u32) -> u32 {
    const NO_MATCH: u32 = u32::MAX;
    const SELECTOR: u32 = 0x29;
    #[repr(C)]
    struct CandidateList {
        _leading: [u32; 3],
        candidate_count: i32,
        candidates: *const u32,
        results: *const u32,
    }
    let mut output = CandidateList {
        _leading: [0; 3],
        candidate_count: 0,
        candidates: core::ptr::null(),
        results: core::ptr::null(),
    };
    let output_address = &mut output as *mut CandidateList as u32;
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, output_address);
    if helper_result & 0xFF == 0 || output.candidate_count <= 0 { return NO_MATCH; }
    for position in 0..output.candidate_count {
        if unsafe { *output.candidates.add(position as usize) } == expected_key {
            let result_value = unsafe { *output.results.add(position as usize) };
            return result_value;
        }
    }
    NO_MATCH
}
