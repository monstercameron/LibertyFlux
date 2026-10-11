// original: 0x008D03E0 streaming_record_write_byte

/// For record index 0, write the supplied byte to record offset 0x35C4. Return without writing for sentinel index -90 or a null record.
lf_checker_rt::export!(cdecl, rw_008d03e0(index: u32, value: u32) -> () {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FIELD_OFFSET: usize = 0x35C4;
    unsafe {
        if index == NULL_INDEX { return; }
        let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        if record == 0 { return; }
        (record as *mut u8).add(FIELD_OFFSET).cast::<u8>().write_unaligned(value as u8);
    }
});
