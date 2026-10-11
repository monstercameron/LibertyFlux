// original: 0x008D01C0 streaming_record_get_state_byte

/// Return the byte at record offset 0x35C5 when the indexed validity byte is nonzero; return zero for sentinel index -90 or invalid status.
lf_checker_rt::export!(cdecl, rw_008d01c0(index: u32) -> u8 {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FIELD_OFFSET: usize = 0x35C5;
    unsafe {
        if index == NULL_INDEX { return 0; }
        let validity = lf_checker_rt::global::<u8>(STREAMING_VALIDITY_VA);
        if validity.add(index as usize).read() == 0 { return 0; }
        let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        (record as *const u8).add(FIELD_OFFSET).read()
    }
});
