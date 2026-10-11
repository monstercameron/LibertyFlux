// original: 0x00641E70 unnamed_00641E70
/// Copy the current thread's resource fields into its model-state record.
///
/// The stdcall argument points to the record. A global pointer identifies the
/// thread record; its byte at offset one selects a 96-byte row in the model
/// state. Eleven 32-bit fields and one byte are copied from the source fields
/// at offsets `0xa0..=0xfc` to corresponding row fields beginning at `0x100`.
/// The row index is an unsigned byte and address arithmetic wraps as 32-bit
/// pointer arithmetic. The routine has no meaningful return value.
lf_checker_rt::export!(stdcall, rw_00641e70(model_state: u32) -> u32 {
    const THREAD_RECORD_POINTER: u32 = 0x01BB6678;
    const THREAD_INDEX_BYTE: u32 = 1;
    const THREAD_ROW_STRIDE: u32 = 0x60;
    const SOURCE_A0: u32 = 0xa0;
    const SOURCE_A4: u32 = 0xa4;
    const SOURCE_A8: u32 = 0xa8;
    const SOURCE_AC: u32 = 0xac;
    const SOURCE_B0: u32 = 0xb0;
    const SOURCE_B4: u32 = 0xb4;
    const SOURCE_B8: u32 = 0xb8;
    const SOURCE_D0: u32 = 0xd0;
    const SOURCE_D4: u32 = 0xd4;
    const SOURCE_D8: u32 = 0xd8;
    const SOURCE_F0: u32 = 0xf0;
    const SOURCE_FC: u32 = 0xfc;
    const TARGET_100: u32 = 0x100;
    const TARGET_104: u32 = 0x104;
    const TARGET_108: u32 = 0x108;
    const TARGET_10C: u32 = 0x10c;
    const TARGET_110: u32 = 0x110;
    const TARGET_114: u32 = 0x114;
    const TARGET_118: u32 = 0x118;
    const TARGET_130: u32 = 0x130;
    const TARGET_134: u32 = 0x134;
    const TARGET_138: u32 = 0x138;
    const TARGET_150: u32 = 0x150;
    const TARGET_15C: u32 = 0x15c;

    unsafe {
        let thread_record = lf_checker_rt::global::<u32>(THREAD_RECORD_POINTER).read();
        let thread_index =
            (thread_record.wrapping_add(THREAD_INDEX_BYTE) as *const u8).read() as u32;
        let row_offset = thread_index.wrapping_mul(THREAD_ROW_STRIDE);
        macro_rules! copy_word {
            ($source:expr, $target:expr) => {{
                let value = (model_state.wrapping_add($source) as *const u32).read();
                (model_state.wrapping_add(row_offset).wrapping_add($target) as *mut u32)
                    .write(value);
            }};
        }
        copy_word!(SOURCE_A0, TARGET_100);
        copy_word!(SOURCE_A4, TARGET_104);
        copy_word!(SOURCE_A8, TARGET_108);
        copy_word!(SOURCE_AC, TARGET_10C);
        copy_word!(SOURCE_B0, TARGET_110);
        copy_word!(SOURCE_B4, TARGET_114);
        copy_word!(SOURCE_B8, TARGET_118);
        copy_word!(SOURCE_D0, TARGET_130);
        copy_word!(SOURCE_D4, TARGET_134);
        copy_word!(SOURCE_D8, TARGET_138);
        copy_word!(SOURCE_F0, TARGET_150);
        let byte_value = (model_state.wrapping_add(SOURCE_FC) as *const u8).read();
        (model_state.wrapping_add(row_offset).wrapping_add(TARGET_15C) as *mut u8)
            .write(byte_value);
    }
    0
});
