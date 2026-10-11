// original: 0x0053DB00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamMafya_BG, player_schema::LeaderboardInfo, 10>::vf8

/// Maps the selected parser entry through a helper result to one of the values 0, 4, or 8.
/// The fastcall parser receives byte size 0x36 in ECX and a local result record in EDX; the entry-array pointer is at byte offset 20.
/// On parser success, the single stack argument selects an entry and the member helper receives that entry in ECX. Helper results 1 or 5 return 4, results 2 or 3 return 8, and every other result returns 0.
/// The local jump table has cases for decremented unsigned values 0 through 4; values outside that range return 0.
/// The checker-facing export uses the game's 32-bit thiscall convention; pointers and word arrays are 32-bit.
/// The parser helper is intercepted by the checker. Its ECX/EDX call shape and initialized output words are part of the contract.
lf_checker_rt::export!(thiscall, rw_fn_0053db00(_this: u32, index: u32) -> u32 {
    unsafe {
                const PARSER_CALLEE_ID: u32 = 1;
                const VALUE_CALLEE_ID: u32 = 2;
                const METADATA_SIZE: u32 = 0x36;
                const ENTRY_ARRAY_WORD: usize = 5;
                const WORD_BYTES: u32 = 4;
                const RETURN_SHORT: u32 = 4;
                const RETURN_LONG: u32 = 8;

                let mut result_record = [0u32; 8];
                let result_record_address = result_record.as_mut_ptr() as usize as u32;
                let parser_ready = lf_checker_rt::callee_fastcall!(
                    PARSER_CALLEE_ID, u8, METADATA_SIZE, result_record_address
                );
                if parser_ready == 0 {
                    return 0;
                }
                let entries = result_record[ENTRY_ARRAY_WORD];
                let entry_address = entries.wrapping_add(index.wrapping_mul(WORD_BYTES));
                let entry = (entry_address as *const u32).read_unaligned();
                let category = lf_checker_rt::callee_thiscall!(VALUE_CALLEE_ID, u32, entry);
                match category {
                    1 => 4,
                    5 => RETURN_SHORT,
                    2 | 3 => RETURN_LONG,
                    _ => 0,
                }
    }
});
