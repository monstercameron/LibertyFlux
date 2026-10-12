// original: 0x0063AA40 blend_bits_by_mask

/// Blend four words bit by bit. `mask_words` is the implicit this pointer;
/// `output`, `left`, and `right` are the three stack arguments. For each
/// output word, a set bit in the corresponding mask selects the right input;
/// a clear bit selects the left input. Return `output`. The original reuses
/// an incoming stack argument slot for its four-word loop counter, so its
/// caller-stack scratch write is excluded from comparison; output, return,
/// registers, and stack cleanup remain checked.
lf_checker_rt::export!(thiscall, rw_0063aa40(
    mask_words: *const u32,
    output: *mut u32,
    left: *const u32,
    right: *const u32,
) -> *mut u32 {
    unsafe {
        const WORD_COUNT: usize = 4;
        for index in 0..WORD_COUNT {
            let mask = mask_words.add(index).read_unaligned();
            let left_word = left.add(index).read_unaligned();
            let right_word = right.add(index).read_unaligned();
            let blended = (left_word & !mask) | (right_word & mask);
            output.add(index).write_unaligned(blended);
        }
    }
    output
});
