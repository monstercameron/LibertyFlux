// original: 0x00614270 streaming_component_word_exchange

/// Exchange six pairs of 32-bit component words in place. Each pair belongs
/// to corresponding fields in the two halves of a streaming component record.
/// Values move bit-for-bit, including floating-point encodings; the method
/// takes only the object pointer and has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00614270(this: u32) -> u32 {
    const COMPONENT_WORD_PAIRS: [(u32, u32); 6] = [
        (0x04, 0x10),
        (0x08, 0x20),
        (0x18, 0x24),
        (0x0c, 0x30),
        (0x1c, 0x34),
        (0x2c, 0x38),
    ];

    unsafe {
        for &(left_offset, right_offset) in &COMPONENT_WORD_PAIRS {
            let left = ((this + left_offset) as *const u32).read_unaligned();
            let right = ((this + right_offset) as *const u32).read_unaligned();
            ((this + left_offset) as *mut u32).write_unaligned(right);
            ((this + right_offset) as *mut u32).write_unaligned(left);
        }
    }
    0
});
