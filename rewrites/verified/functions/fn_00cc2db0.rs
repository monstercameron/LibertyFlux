// original: 0x00CC2DB0 replace_task_type_words_at_offset_4
/// Replace the task-type word with `new_value` in each of nine 0x20-byte
/// rows whose word at offset 4 equals `old_value`. Other row words are
/// preserved. The receiver is passed in ECX and both values are stack args.
lf_checker_rt::export!(thiscall, rw_00cc2db0(this: *mut u8, old_value: u32, new_value: u32) -> u32 {
    const ROW_COUNT: usize = 9;
    const ROW_STRIDE: usize = 0x20;
    const TASK_TYPE_OFFSET: usize = 4;
    for row in 0..ROW_COUNT {
        let field = unsafe { this.add(TASK_TYPE_OFFSET + row * ROW_STRIDE).cast::<u32>() };
        if unsafe { field.read_unaligned() } == old_value {
            unsafe { field.write_unaligned(new_value) };
        }
    }
    new_value
});
