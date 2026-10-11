// original: 0x00BCC410 physics_prepare_vector_process
/// Package five scalar bit patterns and two boundary values for one helper,
/// then return the helper's 32-bit result. The cdecl inputs are an integer
/// token, five float words, and an integer flag. A local eight-word record
/// places the third and fourth float words at offsets 0 and 4, the exact
/// -100.0f sentinel at offsets 8 and 24, and the first two float words at
/// offsets 16 and 20; untouched words are zero. The helper receives that
/// record and its suffix by pointer, plus the token, fifth float word and
/// flag. Its return is forwarded unchanged. The contract snapshots both
/// pointer ranges and checks the remaining outgoing arguments and EAX.
lf_checker_rt::export!(
    cdecl,
    rw_00BCC410(
        input_token: u32,
        first_value_bits: u32,
        second_value_bits: u32,
        third_value_bits: u32,
        fourth_value_bits: u32,
        fifth_value_bits: u32,
        enabled_word: u32,
    ) -> u32 {
        const PROCESS_LIMIT_BITS: u32 = 0xc2c8_0000;
        let mut local_words = [0u32; 8];
        local_words[0] = third_value_bits;
        local_words[1] = fourth_value_bits;
        local_words[2] = PROCESS_LIMIT_BITS;
        local_words[4] = first_value_bits;
        local_words[5] = second_value_bits;
        local_words[6] = PROCESS_LIMIT_BITS;
        let local_base = local_words.as_mut_ptr() as usize as u32;
        let second_record = local_base.wrapping_add(4 * 4);
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            0u32,
            input_token,
            second_record,
            local_base,
            fifth_value_bits,
            enabled_word,
        )
    }
);
