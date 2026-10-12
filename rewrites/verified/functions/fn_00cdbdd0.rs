// original: 0x00CDBDD0 CTaskSimpleNMScriptControl::vf21

/// Adds the matching script-key bit to the task's message flags.
///
/// The method scans the 29-entry image index table, loads each selected key
/// pointer from the image pointer table, and compares it with the NUL-terminated
/// text at the second stack argument's `+0x0c`. On a match it ORs the current
/// one-bit mask into the dword at `this + 0x2c`; the mask rotates left once per
/// table entry. EAX retains the last comparison result, or the last key pointer
/// when that key is empty. This x86 thiscall method takes two stack arguments
/// and pops eight bytes.
lf_checker_rt::export!(thiscall, rw_00cdbdd0(this: u32, _arg_one: u32, params: u32) -> u32 {
    unsafe {
        const INDEX_TABLE_START: u32 = 0x0105_1c48;
        const INDEX_TABLE_END: u32 = 0x0105_1cbc;
        const POINTER_TABLE_START: u32 = 0x0105_1cc4;
        const TEXT_OFFSET: u32 = 0x0c;
        const MESSAGE_FLAGS: u32 = 0x2c;

        let text = params.wrapping_add(TEXT_OFFSET);
        let mut table_entry = lf_checker_rt::global::<u32>(INDEX_TABLE_START);
        let table_end = lf_checker_rt::global::<u32>(INDEX_TABLE_END) as u32;
        let mut message_mask = 1u32;
        let mut last_result = 0u32;

        while (table_entry as u32) < table_end {
            let key_index = table_entry.read_unaligned();
            let key_slot = POINTER_TABLE_START.wrapping_add(key_index.wrapping_mul(4));
            let key_pointer = lf_checker_rt::global::<u32>(key_slot).read_unaligned();
            last_result = key_pointer;

            if (key_pointer as *const u8).read() != 0 {
                let mut byte_offset = 0u32;
                loop {
                    let key_byte = (key_pointer.wrapping_add(byte_offset) as *const u8).read();
                    let text_byte = (text.wrapping_add(byte_offset) as *const u8).read();
                    if key_byte != text_byte {
                        last_result = if key_byte < text_byte { u32::MAX } else { 1 };
                        break;
                    }
                    if key_byte == 0 {
                        last_result = 0;
                        let flags = this.wrapping_add(MESSAGE_FLAGS) as *mut u32;
                        flags.write_unaligned(flags.read_unaligned() | message_mask);
                        break;
                    }
                    byte_offset = byte_offset.wrapping_add(1);
                }
            }

            table_entry = table_entry.wrapping_add(1);
            message_mask = message_mask.rotate_left(1);
        }

        last_result
    }
});
