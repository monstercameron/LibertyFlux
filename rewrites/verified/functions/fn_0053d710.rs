// original: 0x0053D710 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamMafya, player_schema::LeaderboardInfo, 10>::vf9

/// Maps the selected parser entry through a helper result to an index or -1.
/// The fastcall parser receives byte size 0x2c in ECX and a local result record in EDX; the entry-array pointer is at byte offset 20.
/// On parser success, the single stack argument selects an entry and the member helper receives that entry in ECX. Helper results 1, 2, 3, and 5 return 0, 1, 3, and 2 respectively; all other results return -1.
/// The comparisons treat the helper result and the decremented switch index as unsigned values.
/// The checker-facing export uses the game's 32-bit thiscall convention; pointers and word arrays are 32-bit.
/// The parser helper is intercepted by the checker. Its ECX/EDX call shape and initialized output words are part of the contract.
lf_checker_rt::export!(thiscall, rw_fn_0053d710(_this: u32, index: u32) -> u32 {
    unsafe {
                const PARSER_CALLEE_ID: u32 = 1;
                const VALUE_CALLEE_ID: u32 = 2;
                const METADATA_SIZE: u32 = 0x2c;
                const ENTRY_ARRAY_WORD: usize = 5;
                const WORD_BYTES: u32 = 4;
                const NO_MATCH: u32 = u32::MAX;

                let mut result_record = [0u32; 8];
                let result_record_address = result_record.as_mut_ptr() as usize as u32;
                let parser_ready = lf_checker_rt::callee_fastcall!(
                    PARSER_CALLEE_ID, u8, METADATA_SIZE, result_record_address
                );
                if parser_ready == 0 {
                    return NO_MATCH;
                }
                let entries = result_record[ENTRY_ARRAY_WORD];
                let entry_address = entries.wrapping_add(index.wrapping_mul(WORD_BYTES));
                let entry = (entry_address as *const u32).read_unaligned();
                let category = lf_checker_rt::callee_thiscall!(VALUE_CALLEE_ID, u32, entry);
                match category {
                    1 => 0,
                    2 => 1,
                    3 => 3,
                    5 => 2,
                    _ => NO_MATCH,
                }
    }
});
