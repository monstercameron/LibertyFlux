// original: 0x00ABC3E0 input_ui_insert_before_end

/// Copy the source's first word into the word immediately before the end
/// pointer, then forward the range metadata to a helper.
///
/// The three cdecl arguments are source pointer, end pointer, and a helper
/// value. The routine saves the old word at `end - 4`, stores `source[0]`
/// there, and computes the signed element count as `(end - source - 4) >> 2`.
/// It calls the helper with source, zero, that count, the old end word, and
/// the helper value, returning the helper's EAX result. The caller cleans the
/// helper arguments and the function's own incoming arguments.
lf_checker_rt::export!(cdecl, rw_00abc3e0(source: u32, end: u32, helper_value: u32) -> u32 {
    unsafe {
        const WORD_BYTES: u32 = 4;

        let end_slot = end.wrapping_sub(WORD_BYTES);
        let previous_word = (end_slot as *const u32).read_unaligned();
        let first_source_word = (source as *const u32).read_unaligned();
        (end_slot as *mut u32).write_unaligned(first_source_word);
        let signed_count = end.wrapping_sub(source).wrapping_sub(WORD_BYTES) as i32;
        let element_count = (signed_count >> 2) as u32;
        lf_checker_rt::callee_cdecl!(1, u32, source, 0u32, element_count, previous_word, helper_value)
    }
});
