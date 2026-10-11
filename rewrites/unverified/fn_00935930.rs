// original: 0x00935930 clear_network_record_flag

/// Clear the byte flag at offset `0x226` in the record selected by `record_index`.
/// The receiver stores the record-array pointer at `+0x144`; records are spaced
/// `0x22c` bytes apart. The index multiply and address addition use 32-bit
/// wrapping arithmetic. The original returns the array pointer in EAX and
/// removes its one stack argument (`thiscall`).
lf_checker_rt::export!(thiscall, rw_00935930(this: u32, record_index: u32) -> u32 {
    unsafe {
        const RECORDS_POINTER: u32 = 0x144;
        const RECORD_STRIDE: u32 = 0x22c;
        const FLAG_OFFSET: u32 = 0x226;

        let records = (this.wrapping_add(RECORDS_POINTER) as *const u32).read_unaligned();
        let record_offset = record_index
            .wrapping_mul(RECORD_STRIDE)
            .wrapping_add(FLAG_OFFSET);
        (records.wrapping_add(record_offset) as *mut u8).write(0);
        records
    }
});
