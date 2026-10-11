// original: 0x00599C70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_7, player_schema::LeaderboardInfo, 10>::vf7
/// Read one 32-bit value from the result array selected by this instance's
/// query selector. The method takes one array index on the stack and returns
/// the selected value, or `u32::MAX` when the query helper declines the
/// request. The helper receives its selector in ECX and a local six-word
/// output record in EDX; the record's fifth word is the returned array
/// pointer. The original has a thiscall entry and pops its one stack word.
lf_checker_rt::export!(thiscall, rw_00599c70(this: u32, index: u32) -> u32 {
    const QUERY_SELECTOR: u32 = 0x00000159;
    const QUERY_CALLEE: u32 = 1;
    const QUERY_OUTPUT_WORDS: usize = 6;
    const VALUES_POINTER_WORD: usize = 4;
    const FAILURE: u32 = u32::MAX;

    let mut query_output = [0u32; QUERY_OUTPUT_WORDS];
    let query_ready = lf_checker_rt::callee_fastcall!(
        QUERY_CALLEE, u32, QUERY_SELECTOR, query_output.as_mut_ptr() as u32
    );
    if query_ready as u8 == 0 {
        return FAILURE;
    }
    let values = query_output[VALUES_POINTER_WORD];
    let value_offset = index.wrapping_mul(4);
    let stored_value = unsafe { (values.wrapping_add(value_offset) as *const u32).read_unaligned() };
    stored_value
});
