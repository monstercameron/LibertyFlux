// original: 0x0053E0B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamVip_BG, player_schema::LeaderboardInfo, 10>::vf12

/// Uses a parser-produced query array to select a key, then searches a second array for that key and returns its first position or -1.
/// The fastcall parser receives byte size 0x24 in ECX and a local result record in EDX. The unsigned count, search-array pointer, and query-array pointer are at byte offsets 4, 8, and 20.
/// The single stack argument indexes the query array. A query value of 0xffffffff is the no-value sentinel. A zero count returns -1.
/// The search loop uses an unsigned count. It compares entries in order and returns the first matching position; otherwise it returns -1.
/// The checker-facing export uses the game's 32-bit thiscall convention; pointers and word arrays are 32-bit.
/// The parser helper is intercepted by the checker. Its ECX/EDX call shape and initialized output words are part of the contract.
lf_checker_rt::export!(thiscall, rw_fn_0053e0b0(_this: u32, index: u32) -> u32 {
    unsafe {
                const PARSER_CALLEE_ID: u32 = 1;
                const METADATA_SIZE: u32 = 0x24;
                const COUNT_WORD: usize = 1;
                const SEARCH_VALUES_WORD: usize = 2;
                const QUERY_VALUES_WORD: usize = 5;
                const WORD_BYTES: u32 = 4;
                const NO_QUERY: u32 = u32::MAX;

                let mut result_record = [0u32; 8];
                let result_record_address = result_record.as_mut_ptr() as usize as u32;
                let parser_ready = lf_checker_rt::callee_fastcall!(
                    PARSER_CALLEE_ID, u8, METADATA_SIZE, result_record_address
                );
                if parser_ready == 0 {
                    return u32::MAX;
                }
                let query_values = result_record[QUERY_VALUES_WORD];
                let query_slot = query_values.wrapping_add(index.wrapping_mul(WORD_BYTES));
                let query = (query_slot as *const u32).read_unaligned();
                if query == NO_QUERY {
                    return u32::MAX;
                }
                let search_values = result_record[SEARCH_VALUES_WORD];
                let count = result_record[COUNT_WORD];
                if count == 0 {
                    return u32::MAX;
                }
                let mut position = 0u32;
                while position < count {
                    let list_slot = search_values.wrapping_add(position.wrapping_mul(WORD_BYTES));
                    let value = (list_slot as *const u32).read_unaligned();
                    if value == query {
                        return position;
                    }
                    position = position.wrapping_add(1);
                }
                u32::MAX
    }
});
