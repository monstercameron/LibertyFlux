// original: 0x0053DF10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamVip, player_schema::LeaderboardInfo, 10>::vf7

/// Reads one word from a parser-produced array at the supplied index.
/// The fastcall parser receives byte size 0xb in ECX and a local result record in EDX; the array pointer is at byte offset 16.
/// A false parser result returns -1. A true result reads `array[index]` without a count or bounds check, as the original does.
/// The single stack argument is an unsigned array index; callers must supply a readable slot.
/// The checker-facing export uses the game's 32-bit thiscall convention; pointers and word arrays are 32-bit.
/// The parser helper is intercepted by the checker. Its ECX/EDX call shape and initialized output words are part of the contract.
lf_checker_rt::export!(thiscall, rw_fn_0053df10(_this: u32, index: u32) -> u32 {
    unsafe {
                const PARSER_CALLEE_ID: u32 = 1;
                const METADATA_SIZE: u32 = 0x0b;
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
                let selected_index = index;
                let slot = values_pointer.wrapping_add(selected_index.wrapping_mul(WORD_BYTES));
                (slot as *const u32).read_unaligned()
    }
});
