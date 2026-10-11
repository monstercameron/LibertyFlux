// original: 0x00615770 skyhat_config_selected_fields_copy

/// Copy selected fields from a one-based pointer table into a configuration
/// record. A zero selector preserves the destination field. The two selected
/// blocks contain four 32-bit words; the remaining selections copy one word.
/// The caller supplies a control record, an object whose pointer table is at
/// byte offset 0x14, and the destination record. These layouts are 32-bit.
lf_checker_rt::export!(stdcall, rw_00615770(control: u32, array_object: u32, destination: u32) -> u32 {
    const FIELD_MAP: [(u32, u32, u8); 9] = [
        (0x48, 0x30, 4),
        (0x4c, 0x44, 1),
        (0x54, 0x64, 1),
        (0x58, 0x68, 1),
        (0x5c, 0x6c, 1),
        (0x60, 0x70, 1),
        (0x78, 0xd8, 1),
        (0x70, 0xa0, 4),
        (0x74, 0xb0, 1),
    ];

    unsafe {
        for &(selector_offset, target_offset, word_count) in &FIELD_MAP {
            let selector = ((control + selector_offset) as *const u32).read_unaligned();
            if selector == 0 {
                continue;
            }

            let table = ((array_object + 0x14) as *const u32).read_unaligned();
            let entry = table.wrapping_add(selector.wrapping_mul(4).wrapping_sub(4));
            let source = (entry as *const u32).read_unaligned();

            if word_count == 4 {
                for chunk_offset in [0u32, 8] {
                    let block = ((source + chunk_offset) as *const u64).read_unaligned();
                    ((destination + target_offset + chunk_offset) as *mut u64)
                        .write_unaligned(block);
                }
            } else {
                let value = (source as *const u32).read_unaligned();
                ((destination + target_offset) as *mut u32).write_unaligned(value);
            }
        }
    }
    0
});
