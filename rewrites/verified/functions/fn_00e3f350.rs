// original: 0x00E3F350 net_info_header_copy_fields

/// Copy the first 28 bytes from the supplied source header to this header,
/// run its refresh helper, then forward the addresses of eleven text fields
/// at 60-byte intervals to the text writer. Return its final scripted result.
/// This is a thiscall with one source pointer; each text call is a thiscall
/// with the record index and source text pointer.
lf_checker_rt::export!(thiscall, rw_00e3f350(destination: u32, source: u32) -> u32 {
    const HEADER_COPY_BYTES: u32 = 28;
    const TEXT_POINTERS_OFFSET: u32 = 0x1C;
    const TEXT_POINTER_STRIDE: u32 = 0x3C;
    const TEXT_FIELD_COUNT: u32 = 11;

    unsafe {
        for word_index in 0..7 {
            let word = (source.wrapping_add(word_index * 4) as *const u32)
                .read_unaligned();
            (destination.wrapping_add(word_index * 4) as *mut u32)
                .write_unaligned(word);
        }
        let _refresh_result = lf_checker_rt::callee_thiscall!(1, u32, destination);
        let mut last_text_result = 0;
        let mut record_index = 0;
        while record_index < TEXT_FIELD_COUNT {
            let text_field = source
                .wrapping_add(TEXT_POINTERS_OFFSET)
                .wrapping_add(record_index.wrapping_mul(TEXT_POINTER_STRIDE));
            last_text_result = lf_checker_rt::callee_thiscall!(
                2, u32, destination, record_index, text_field
            );
            record_index = record_index.wrapping_add(1);
        }
        let _ = HEADER_COPY_BYTES;
        last_text_result
    }
});
