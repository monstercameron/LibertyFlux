// original: 0x00BBB190 forward_float_data_with_local_vectors

/// Forward six incoming scalar-float words and three integer words to the shared cdecl helper.
/// The first three float words and the next three float words are each assembled into a stack-local
/// three-word vector. Their pointer values differ from the original frame, so the contract skips those
/// two pointer comparisons and snapshots all six words at the call. The helper's return word is forwarded.
lf_checker_rt::export!(cdecl, rw_00bbb190(
    first_x_bits: u32, first_y_bits: u32, first_z_bits: u32,
    second_x_bits: u32, second_y_bits: u32, second_z_bits: u32,
    integer_word_0: u32, integer_word_1: u32, integer_word_2: u32
) -> u32 {
    unsafe {
        const SHARED_FORWARD_HELPER: u32 = 1;
        let first_vector = [first_x_bits, first_y_bits, first_z_bits];
        let second_vector = [second_x_bits, second_y_bits, second_z_bits];
        let first_vector_address = first_vector.as_ptr() as u32;
        let second_vector_address = second_vector.as_ptr() as u32;
        lf_checker_rt::callee_cdecl!(
            SHARED_FORWARD_HELPER, u32,
            first_vector_address, second_vector_address,
            integer_word_0, integer_word_1, integer_word_2
        )
    }
});
