// original: 0x0053DCD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamVip, player_schema::LeaderboardInfo, 10>::vf13

/// Searches parser-produced keys and returns the corresponding mapped word, or -1 when no key matches.
/// The fastcall parser receives byte size 0xb in ECX and a local result record in EDX. The signed count, key-array pointer, and mapped-values pointer are at byte offsets 12, 16, and 20.
/// The single stack argument is compared with each key. A signed non-positive count returns -1; the signed loop returns the mapped value for the first match.
/// The checker-facing export uses the game's 32-bit thiscall convention; pointers and word arrays are 32-bit.
/// The parser helper is intercepted by the checker. Its ECX/EDX call shape and initialized output words are part of the contract.
lf_checker_rt::export!(thiscall, rw_fn_0053dcd0(_this: u32, key: u32) -> u32 {
    unsafe {
                const PARSER_CALLEE_ID: u32 = 1;
                const METADATA_SIZE: u32 = 0x0b;
                const COUNT_WORD: usize = 3;
                const KEYS_POINTER_WORD: usize = 4;
                const VALUES_POINTER_WORD: usize = 5;
                const WORD_BYTES: u32 = 4;

                let mut result_record = [0u32; 8];
                let result_record_address = result_record.as_mut_ptr() as usize as u32;
                let parser_ready = lf_checker_rt::callee_fastcall!(
                    PARSER_CALLEE_ID, u8, METADATA_SIZE, result_record_address
                );
                if parser_ready == 0 {
                    return u32::MAX;
                }
                let keys_pointer = result_record[KEYS_POINTER_WORD];
                let values_pointer = result_record[VALUES_POINTER_WORD];
                let count = result_record[COUNT_WORD] as i32;
                if count <= 0 {
                    return u32::MAX;
                }
                let mut position = 0i32;
                while position < count {
                    let position_index: u32 = position as u32;
                    let key_slot = keys_pointer.wrapping_add(position_index.wrapping_mul(WORD_BYTES));
                    let candidate = (key_slot as *const u32).read_unaligned();
                    if candidate == key {
                        let value_slot = values_pointer.wrapping_add(position_index.wrapping_mul(WORD_BYTES));
                        return (value_slot as *const u32).read_unaligned();
                    }
                    position = position.wrapping_add(1);
                }
                u32::MAX
    }
});
