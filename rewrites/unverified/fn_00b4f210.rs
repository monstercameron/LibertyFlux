// original: 0x00B4F210 entity_check_model_flag_bit

/// Read the signed 16-bit model index at byte offset `0x2e`, fetch that
/// index's entry from the relocated model table, and test its category flag.
/// Indices 0 through 31 select the low flag word at `0x120`; indices 32
/// through 63 select the high flag word at `0x124`. Indices at or above 64
/// produce false after the table entry is read. The result is returned in AL.
lf_checker_rt::export!(thiscall, rw_b4f210(this: *const u8) -> u8 {
    const MODEL_INDEX_OFFSET: usize = 0x2e;
    const FLAG_WORD_LOW: usize = 0x120;
    const FLAG_WORD_HIGH: usize = 0x124;
    const MODEL_TABLE_VA: u32 = 0x01295cd8;

    unsafe {
        let index = (this.add(MODEL_INDEX_OFFSET) as *const i16).read_unaligned() as i32;
        let entry_va = MODEL_TABLE_VA.wrapping_add((index as u32).wrapping_mul(4));
        let entry_address = lf_checker_rt::relocated(entry_va);
        let model = (entry_address as *const u32).read_unaligned() as *const u8;
        let bit_mask = if index < 32 {
            1u32.wrapping_shl(index as u32)
        } else if index < 64 {
            1u32.wrapping_shl((index - 32) as u32)
        } else {
            0
        };
        let flag_word = if index < 32 { FLAG_WORD_LOW } else { FLAG_WORD_HIGH };
        if (model.add(flag_word) as *const u32).read_unaligned() & bit_mask != 0 { 1 } else { 0 }
    }
});

