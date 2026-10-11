// original: 0x00BCC480 physics_prepare_extended_vector_process
/// Package seven scalar float words for a six-argument helper and forward its
/// result. The cdecl inputs are an integer token, seven float bit patterns and
/// one trailing word. An eight-word local record stores input words four,
/// five and six at offsets 0, 4 and 8, then words one, two and three at offsets
/// 16, 20 and 24; the remaining slots are zero. The helper receives literal
/// tag 1, the input token, pointers to the record suffix and base, then the
/// final float word and trailing word. The pointer values vary by stack frame,
/// so the contract skips them and snapshots four and eight words respectively.
lf_checker_rt::export!(
    cdecl,
    rw_00BCC480(
        input_token: u32,
        first_value_bits: u32,
        second_value_bits: u32,
        third_value_bits: u32,
        fourth_value_bits: u32,
        fifth_value_bits: u32,
        sixth_value_bits: u32,
        final_value_bits: u32,
        trailing_word: u32,
    ) -> u32 {
        let mut local_words = [0u32; 8];
        local_words[0] = fourth_value_bits;
        local_words[1] = fifth_value_bits;
        local_words[2] = sixth_value_bits;
        local_words[4] = first_value_bits;
        local_words[5] = second_value_bits;
        local_words[6] = third_value_bits;
        let local_base = local_words.as_mut_ptr() as usize as u32;
        let second_record = local_base.wrapping_add(4 * 4);
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            1u32,
            input_token,
            second_record,
            local_base,
            final_value_bits,
            trailing_word,
        )
    }
);
