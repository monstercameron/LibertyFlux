// original: 0x00593DD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_256, player_schema::LeaderboardInfo, 10>::vf12
/// Resolve the key selected by the stack index, then return its position in
/// the query's key list. The helper writes an unsigned count, a key-array
/// pointer and an index-to-key pointer into a six-word local record. A `-1`
/// key sentinel, a zero count, a missing key, or a declined helper request
/// returns `u32::MAX`. The search bound is unsigned, matching the original's
/// `jb` loop test. The thiscall entry pops its one stack index.
lf_checker_rt::export!(thiscall, rw_00593dd0(this: u32, index: u32) -> u32 {
    const QUERY_SELECTOR: u32 = 0x0000015B;
    const QUERY_CALLEE: u32 = 1;
    const QUERY_OUTPUT_WORDS: usize = 6;
    const COUNT_WORD: usize = 1;
    const KEYS_POINTER_WORD: usize = 2;
    const INDEX_TO_KEY_POINTER_WORD: usize = 5;
    const FAILURE: u32 = u32::MAX;

    let mut query_output = [0u32; QUERY_OUTPUT_WORDS];
    let query_ready = lf_checker_rt::callee_fastcall!(
        QUERY_CALLEE, u32, QUERY_SELECTOR, query_output.as_mut_ptr() as u32
    );
    if query_ready as u8 == 0 {
        return FAILURE;
    }

    let index_to_key = query_output[INDEX_TO_KEY_POINTER_WORD];
    let item_offset = index.wrapping_mul(4);
    let selected_key = unsafe { (index_to_key.wrapping_add(item_offset) as *const u32).read_unaligned() };
    if selected_key == FAILURE {
        return FAILURE;
    }
    let entry_count = query_output[COUNT_WORD];
    if entry_count == 0 {
        return FAILURE;
    }
    let keys = query_output[KEYS_POINTER_WORD];
    let search_limit = entry_count;
    let mut position = 0u32;
    while position < search_limit {
        let key_offset = position.wrapping_mul(4);
        let key = unsafe { (keys.wrapping_add(key_offset) as *const u32).read_unaligned() };
        if key == selected_key {
            return position;
        }
        position = position.wrapping_add(1);
    }
    FAILURE
});
