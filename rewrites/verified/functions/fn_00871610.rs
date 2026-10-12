// original: 0x00871610 write_dof_vector4_entry

/// Write one four-component value into the record selected by its key.
///
/// The receiver pointer and key are forwarded to the lookup helper with mode
/// `1`. If it returns null, return false. Otherwise copy four raw words from
/// the input vector to record offsets `+0x10..+0x1c`, clear bit `0x10` in the
/// record metadata byte at `+0x04`, and return true. The copy preserves every
/// component bit pattern, including floating-point payloads.
///
/// Original calling convention: thiscall with a key and vector pointer on the
/// stack; the method removes both stack arguments.
lf_checker_rt::export!(thiscall, rw_00871610(this: u32, key: u32, vector: u32) -> u32 {
    const LOOKUP_MODE: u32 = 1;
    const VECTOR_WORD_OFFSETS: [u32; 4] = [0x00, 0x04, 0x08, 0x0c];
    const RECORD_VECTOR_OFFSET: u32 = 0x10;
    const RECORD_FLAGS_OFFSET: u32 = 0x04;
    const VECTOR_VALID_BIT: u8 = 0x10;
    unsafe {
        let record = lf_checker_rt::callee_thiscall!(1, u32, this, LOOKUP_MODE, key);
        if record == 0 {
            return 0;
        }
        for offset in VECTOR_WORD_OFFSETS {
            let word = (vector.wrapping_add(offset) as *const u32).read_unaligned();
            (record.wrapping_add(RECORD_VECTOR_OFFSET + offset) as *mut u32)
                .write_unaligned(word);
        }
        let flags = (record.wrapping_add(RECORD_FLAGS_OFFSET) as *mut u8).read_unaligned();
        (record.wrapping_add(RECORD_FLAGS_OFFSET) as *mut u8)
            .write_unaligned(flags & !VECTOR_VALID_BIT);
        1
    }
});
