// original: 0x00B4F210 entity_check_model_flag_bit

/// Read a signed 16-bit model index from the object at byte offset `0x2e`,
/// use it to fetch a model pointer from a relocated global table, and test
/// the supplied unsigned bit index. Bit indices 0..31 address the low word;
/// 32..63 address the high word; 64 and above select neither. Both flag
/// words are read on every call. This thiscall method also takes one stack
/// argument and returns its boolean result in AL.
lf_checker_rt::export!(thiscall, rw_b4f210(this: *const u8, bit_index: u32) -> u8 {
    const MODEL_INDEX_OFFSET: usize = 0x2e;
    const FLAG_WORD_LOW: usize = 0x120;
    const FLAG_WORD_HIGH: usize = 0x124;
    const MODEL_TABLE_VA: u32 = 0x01295cd8;

    unsafe {
        let model_index = (this.add(MODEL_INDEX_OFFSET) as *const i16).read_unaligned() as i32;
        let entry_va = MODEL_TABLE_VA.wrapping_add((model_index as u32).wrapping_mul(4));
        let entry_address = lf_checker_rt::relocated(entry_va);
        let model = (entry_address as *const u32).read_unaligned() as *const u8;
        let bit_mask = 1u32.wrapping_shl(bit_index & 31);
        let low_mask = if bit_index < 32 { bit_mask } else { 0 };
        let high_mask = if bit_index >= 32 && bit_index < 64 { bit_mask } else { 0 };
        let low_flags = (model.add(FLAG_WORD_LOW) as *const u32).read_unaligned();
        let high_flags = (model.add(FLAG_WORD_HIGH) as *const u32).read_unaligned();
        let selected_flags = (low_mask & low_flags) | (high_mask & high_flags);
        if selected_flags != 0 { 1 } else { 0 }
    }
});

