// original: 0x00ABC8A0 input_ui_copy_30_word_record

/// Copy a fixed 30-word record from the source object to the destination.
///
/// The destination arrives in ECX and the source is the sole stack argument.
/// The record occupies 0x78 bytes. Words are read and written in ascending
/// order, preserving the original's forward-copy behavior when the ranges
/// overlap. The destination address is returned in EAX, and the callee removes
/// its four-byte stack argument. Fields are copied as raw bits so integer and
/// floating-point members retain their exact representations.
lf_checker_rt::export!(thiscall, rw_00abc8a0(destination: u32, source: u32) -> u32 {
    unsafe {
        const RECORD_BYTES: u32 = 0x78;
        const WORD_BYTES: u32 = 4;

        let mut byte_offset = 0u32;
        while byte_offset < RECORD_BYTES {
            let word = (source.wrapping_add(byte_offset) as *const u32).read_unaligned();
            (destination.wrapping_add(byte_offset) as *mut u32).write_unaligned(word);
            byte_offset += WORD_BYTES;
        }
        destination
    }
});
