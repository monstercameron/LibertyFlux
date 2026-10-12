// original: 0x00CDBB80 CTaskSimpleNMScriptControl::vf22

/// Adds script-key flags to a NaturalMotion task object.
///
/// The method uses `this` from ECX and takes two stack arguments; the second
/// points to a record whose text begins at `+0x0c`. It walks 29 four-byte
/// indices from the image's script-key index list, uses each index to load a
/// string pointer from the image table, and compares nonempty keys byte for
/// byte with the record text. For every match it ORs the corresponding
/// rotating bit into the object's word at `+0x34`. Empty keys are skipped.
/// The final EAX value is the last nonempty string comparison result, or the
/// last loaded string pointer when the last table entry is empty. The image
/// tables and object offsets retain the original 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00cdbb80(this: u32, _unused: u32, params: u32) -> u32 {
    unsafe {
        const INDEX_LIST: u32 = 0x0105_1c48;
        const STRING_POINTERS: u32 = 0x0105_1cc4;
        const OBJECT_FLAGS: u32 = 0x34;
        const INPUT_TEXT: u32 = 0x0c;
        const INDEX_COUNT: u32 = 29;

        let text = params.wrapping_add(INPUT_TEXT);
        let mut index_offset = 0u32;
        let mut bit = 1u32;
        let mut result = 0u32;
        while index_offset < INDEX_COUNT * 4 {
            let entry_index = lf_checker_rt::global::<u32>(
                INDEX_LIST.wrapping_add(index_offset),
            )
            .read_unaligned();
            let pointer_slot = STRING_POINTERS.wrapping_add(entry_index.wrapping_mul(4));
            let key = lf_checker_rt::global::<u32>(pointer_slot).read_unaligned();
            result = key;

            if (key as *const u8).read() != 0 {
                let mut byte_offset = 0u32;
                let mut equal = true;
                loop {
                    let key_byte = (key.wrapping_add(byte_offset) as *const u8).read();
                    let text_byte = (text.wrapping_add(byte_offset) as *const u8).read();
                    if key_byte != text_byte {
                        equal = false;
                        break;
                    }
                    if key_byte == 0 {
                        break;
                    }
                    byte_offset = byte_offset.wrapping_add(1);
                }

                if equal {
                    let flags = this.wrapping_add(OBJECT_FLAGS) as *mut u32;
                    flags.write_unaligned(flags.read_unaligned() | bit);
                    result = 0;
                } else {
                    result = 1;
                }
            }

            bit = bit.rotate_left(1);
            index_offset = index_offset.wrapping_add(4);
        }
        result
    }
});
