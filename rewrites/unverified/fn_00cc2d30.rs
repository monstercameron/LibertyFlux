// original: 0x00CC2D30 merge_duplicate_blend_rows
/// For each of nine 0x20-byte rows, merge matching later task codes into the
/// earlier row's weight. The later weight is the left operand and the earlier
/// weight the right operand; then the later row is marked consumed and its
/// six blend words are reset. A consumed code is `u32::MAX`.
lf_checker_rt::export!(thiscall, rw_00cc2d30(this: *mut u8) -> u32 {
    use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_set_ss};
    const ROW_COUNT: usize = 9;
    const ROW_STRIDE: usize = 0x20;
    const TASK_CODE_OFFSET: usize = 4;
    const WEIGHT_OFFSET: usize = 0x0C;
    const RESET_WORDS: usize = 6;
    const CONSUMED: u32 = u32::MAX;
    const FINAL_RESET_VALUE: u32 = 0x3F800000;
    for source_row in 0..ROW_COUNT - 1 {
        let source = unsafe { this.add(source_row * ROW_STRIDE) };
        let source_code = unsafe { source.add(TASK_CODE_OFFSET).cast::<u32>().read_unaligned() };
        if source_code == CONSUMED {
            continue;
        }
        for later_row in source_row + 1..ROW_COUNT {
            let later = unsafe { this.add(later_row * ROW_STRIDE) };
            let later_code = unsafe { later.add(TASK_CODE_OFFSET).cast::<u32>().read_unaligned() };
            if source_code != later_code {
                continue;
            }
            let source_weight = unsafe { source.add(WEIGHT_OFFSET).cast::<u32>().read_unaligned() };
            let later_weight = unsafe { later.add(WEIGHT_OFFSET).cast::<u32>().read_unaligned() };
            let left = core::hint::black_box(f32::from_bits(later_weight));
            let right = core::hint::black_box(f32::from_bits(source_weight));
            let combined = unsafe {
                _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(left), _mm_set_ss(right))).to_bits()
            };
            unsafe { source.add(WEIGHT_OFFSET).cast::<u32>().write_unaligned(combined) };
            unsafe { later.add(TASK_CODE_OFFSET).cast::<u32>().write_unaligned(CONSUMED) };
            for word in 0..RESET_WORDS {
                let value = if word == RESET_WORDS - 1 { FINAL_RESET_VALUE } else { 0 };
                unsafe {
                    later.add(WEIGHT_OFFSET + word * 4)
                        .cast::<u32>()
                        .write_unaligned(value);
                }
            }
        }
    }
    0
});
