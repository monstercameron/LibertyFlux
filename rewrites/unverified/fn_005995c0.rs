// original: 0x005995C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_6, player_schema::LeaderboardInfo, 10>::vf13
/// Search the helper's signed-length key list for the requested 32-bit key and return the aligned value. A declined helper, nonpositive signed count, or missing key returns `u32::MAX`. The helper fills a local six-word result record with its count, key pointer and result pointer; the thiscall entry pops its one stack key.
lf_checker_rt::export!(thiscall, rw_005995c0(this: u32, requested_key: u32) -> u32 {
    const QUERY_SELECTOR: u32 = 0x00000158;
    const QUERY_CALLEE: u32 = 1;
    const QUERY_OUTPUT_WORDS: usize = 6;
    const COUNT_WORD: usize = 3;
    const KEYS_POINTER_WORD: usize = 4;
    const RESULTS_POINTER_WORD: usize = 5;
    const FAILURE: u32 = u32::MAX;

    let mut query_output = [0u32; QUERY_OUTPUT_WORDS];
    let query_ready = lf_checker_rt::callee_fastcall!(
        QUERY_CALLEE, u32, QUERY_SELECTOR, query_output.as_mut_ptr() as u32
    );
    if query_ready as u8 == 0 {
        return FAILURE;
    }

    let raw_count = query_output[COUNT_WORD];
    let entry_count = raw_count as i32;
    if entry_count <= 0 { return FAILURE; }
    let mut position = 0i32;
    let keys = query_output[KEYS_POINTER_WORD];
    let values = query_output[RESULTS_POINTER_WORD];
    while position < entry_count {
        let item_offset = (position as u32).wrapping_mul(4);
        let item_key = unsafe { (keys.wrapping_add(item_offset) as *const u32).read_unaligned() };
        if item_key == requested_key {
            return unsafe { (values.wrapping_add(item_offset) as *const u32).read_unaligned() };
        }
        position += 1;
    }
    FAILURE
});
