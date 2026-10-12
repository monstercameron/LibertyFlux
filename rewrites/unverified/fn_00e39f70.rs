// original: 0x00E39F70 copy_text_then_replace_final_byte_with_image_suffix

/// Copy a nul-terminated byte string to the destination, then overwrite the
/// terminator and following bytes with the read-only suffix stored in the
/// original image. Empty strings begin the suffix at the destination. The
/// stdcall routine takes source then
/// destination and returns the suffix word with its final byte.
lf_checker_rt::export!(stdcall, rw_00e39f70(source: u32, destination: u32) -> u32 {
    const SUFFIX_ADDRESS: u32 = 0x00F143F0;

    unsafe {
        let destination_delta = destination.wrapping_sub(source);
        let mut source_cursor = source;
        loop {
            let byte = (source_cursor as *const u8).read();
            (destination_delta.wrapping_add(source_cursor) as *mut u8).write(byte);
            source_cursor = source_cursor.wrapping_add(1);
            if byte == 0 {
                break;
            }
        }

        let mut suffix_start = destination.wrapping_sub(1);
        loop {
            let next = suffix_start.wrapping_add(1);
            let byte = (next as *const u8).read();
            suffix_start = next;
            if byte == 0 {
                break;
            }
        }

        let suffix_address = lf_checker_rt::relocated(SUFFIX_ADDRESS);
        let suffix_word = (suffix_address as *const u32).read_unaligned();
        let suffix_byte = (suffix_address.wrapping_add(4) as *const u8).read();
        (suffix_start as *mut u32).write_unaligned(suffix_word);
        (suffix_start.wrapping_add(4) as *mut u8).write(suffix_byte);
        (suffix_word & 0xFFFF_FF00) | suffix_byte as u32
    }
});
