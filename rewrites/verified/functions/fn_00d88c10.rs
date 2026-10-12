// original: 0x00D88C10 native_indexed_model_flag_bit6

/// Reads a signed 16-bit table index from byte offset 0x2e of the argument,
/// loads the corresponding pointer from the global model table, and returns
/// bit 6 of the 32-bit field at offset 0x94. The lookup uses four-byte table
/// entries and sign-extends negative indices before applying the index.
lf_checker_rt::export!(cdecl, rw_00d88c10(record: u32) -> u32 {
    const MODEL_TABLE_VA: u32 = 0x0129_5cd8;
    const INDEX_OFFSET: u32 = 0x2e;
    const MODEL_FLAGS_OFFSET: u32 = 0x94;
    const MODEL_FLAG_MASK: u32 = 1 << 6;

    let table_index = unsafe {
        (record.wrapping_add(INDEX_OFFSET) as *const i16).read_unaligned()
    };
    let model = unsafe {
        lf_checker_rt::global::<u32>(MODEL_TABLE_VA)
            .offset(table_index as isize)
            .read_volatile()
    };
    let model_flags = unsafe {
        (model.wrapping_add(MODEL_FLAGS_OFFSET) as *const u32).read_volatile()
    };

    u32::from(model_flags & MODEL_FLAG_MASK != 0)
});
