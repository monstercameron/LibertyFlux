// original: 0x0053DA60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamMafya_BG, player_schema::LeaderboardInfo, 10>::vf6

/// Searches a parser-produced list for one 32-bit key and returns its first index or -1.
/// The fastcall parser receives byte size 0x36 in ECX and a local result record in EDX. The record's signed count is at byte offset 12 and its values pointer is at byte offset 16.
/// A false parser result or a count that is signed non-positive returns -1. The loop bound is signed, matching the original conditional branches.
/// The single stack argument is the key. A positive count with no match returns -1; no array bounds check is added beyond that count.
/// The checker-facing export uses the game's 32-bit thiscall convention; pointers and word arrays are 32-bit.
/// The parser helper is intercepted by the checker. Its ECX/EDX call shape and initialized output words are part of the contract.
lf_checker_rt::export!(thiscall, rw_fn_0053da60(_this: u32, key: u32) -> u32 {
    unsafe {
                const PARSER_CALLEE_ID: u32 = 1;
                const METADATA_SIZE: u32 = 0x36;
                const COUNT_WORD: usize = 3;
                const VALUES_POINTER_WORD: usize = 4;
                const WORD_BYTES: u32 = 4;

                let mut result_record = [0u32; 8];
                let result_record_address = result_record.as_mut_ptr() as usize as u32;
                let parser_ready = lf_checker_rt::callee_fastcall!(
                    PARSER_CALLEE_ID, u8, METADATA_SIZE, result_record_address
                );
                if parser_ready == 0 {
                    return u32::MAX;
                }
                let values_pointer = result_record[VALUES_POINTER_WORD];
                let count = result_record[COUNT_WORD] as i32;
                if count <= 0 {
                    return u32::MAX;
                }
                let mut index = 0i32;
                while index < count {
                    let slot = values_pointer.wrapping_add((index as u32).wrapping_mul(WORD_BYTES));
                    let value = (slot as *const u32).read_unaligned();
                    if value == key {
                        return index as u32;
                    }
                    index = index.wrapping_add(1);
                }
                u32::MAX
    }
});
