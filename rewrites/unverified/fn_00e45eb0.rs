// original: 0x00E45EB0 net_info_header_write_text_or_ellipsis

/// Write text at `this + record_index * 60 + 28`, including its nul byte.
/// A null text pointer writes the four-byte ellipsis representation there.
/// The thiscall takes the record index and text pointer and returns the text
/// cursor after its nul byte, or zero for the null-text case.
lf_checker_rt::export!(thiscall, rw_00e45eb0(this: u32, record_index: u32, text: u32) -> u32 {
    const RECORD_STRIDE: u32 = 60;
    const TEXT_OFFSET: u32 = 28;
    const ELLIPSIS: u32 = 0x002E2E2E;

    unsafe {
        let target = this
            .wrapping_add(record_index.wrapping_mul(RECORD_STRIDE))
            .wrapping_add(TEXT_OFFSET);
        if text == 0 {
            (target as *mut u32).write_unaligned(ELLIPSIS);
            0
        } else {
            let mut source_cursor = text;
            let mut destination_cursor = target;
            loop {
                let byte = (source_cursor as *const u8).read();
                (destination_cursor as *mut u8).write(byte);
                source_cursor = source_cursor.wrapping_add(1);
                destination_cursor = destination_cursor.wrapping_add(1);
                if byte == 0 {
                    break;
                }
            }
            source_cursor
        }
    }
});
