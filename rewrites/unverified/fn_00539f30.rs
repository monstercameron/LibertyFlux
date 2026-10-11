// original: 0x00539F30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopDrugFactory, player_schema::LeaderboardInfo, 10>::vf12
/// Use the selector-specific lookup helper to obtain an indexed key table and a candidate-key array. Return the first candidate position whose unsigned key equals the key selected by `lookup_index`; return `u32::MAX` when the helper declines, the selected key is the sentinel, the candidate count is zero, or no candidate matches. The receiver is not read.
lf_checker_rt::export!(thiscall, rw_00539f30(_receiver: u32, lookup_index: u32) -> u32 {
    const NO_MATCH: u32 = u32::MAX;
    const SELECTOR: u32 = 0x2;
    #[repr(C)]
    struct KeyLookup {
        _leading: u32,
        candidate_count: u32,
        candidates: *const u32,
        _middle: [u32; 2],
        keys_by_index: *const u32,
    }
    let mut output = KeyLookup {
        _leading: 0,
        candidate_count: 0,
        candidates: core::ptr::null(),
        _middle: [0; 2],
        keys_by_index: core::ptr::null(),
    };
    let output_address = &mut output as *mut KeyLookup as u32;
    let helper_result = lf_checker_rt::callee_fastcall!(1, u32, SELECTOR, output_address);
    if helper_result & 0xFF == 0 { return NO_MATCH; }
    let selected_key = unsafe { *output.keys_by_index.add(lookup_index as usize) };
    if selected_key == NO_MATCH || output.candidate_count == 0 { return NO_MATCH; }
    for position in 0..output.candidate_count {
        if unsafe { *output.candidates.add(position as usize) } == selected_key {
            return position;
        }
    }
    NO_MATCH
});
