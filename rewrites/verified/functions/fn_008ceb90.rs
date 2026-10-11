// original: 0x008CEB90 streaming_record_get_status_word

/// Return the dword at record offset 0x34F4; return zero for sentinel index -90, a null record, or a zero validity byte.
lf_checker_rt::export!(cdecl, rw_008ceb90(index: u32) -> u32 {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FIELD_OFFSET: u32 = 0x34F4;
    unsafe {
        if index == NULL_INDEX { return 0; }
        let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        if record == 0 { return 0; }
        let validity = lf_checker_rt::global::<u8>(STREAMING_VALIDITY_VA);
        if validity.add(index as usize).read() == 0 { return 0; }
        (record.wrapping_add(FIELD_OFFSET) as *const u32).read_unaligned()
    }
});
