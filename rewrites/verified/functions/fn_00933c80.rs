// original: 0x00933C80 detail_record_clear

/// Clear selected state fields in a detail record and return the same record
/// pointer. Four 32-bit fields, one 16-bit field, and four one-byte flags are
/// reset; the rest of the record is preserved. The object pointer is passed
/// in ECX under thiscall.
lf_checker_rt::export!(thiscall, rw_00933C80(record: u32) -> u32 {
    const STATE_WORDS: [u32; 4] = [0x154, 0x158, 0x15C, 0x160];
    const STATE_HALFWORD: u32 = 0x164;
    const FLAG_BYTES: [u32; 4] = [0x000, 0x104, 0x144, 0x167];

    unsafe {
        for offset in STATE_WORDS {
            (record.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (record.wrapping_add(STATE_HALFWORD) as *mut u16).write_unaligned(0);
        for offset in FLAG_BYTES {
            (record.wrapping_add(offset) as *mut u8).write(0);
        }
    }
    record
});
