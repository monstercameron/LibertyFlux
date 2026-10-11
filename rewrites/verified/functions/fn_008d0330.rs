// original: 0x008D0330 streaming_record_write_indexed_float

/// For record index array_index, write the supplied dword to record offset 0x35C8 plus the element index. Return without writing for sentinel index -90 or a null record.
lf_checker_rt::export!(cdecl, rw_008d0330(index: u32, array_index: u32, value_bits: u32) -> () {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FIELD_OFFSET: usize = 0x35C8;
    unsafe {
        if index == NULL_INDEX { return; }
        let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        if record == 0 { return; }
        (record as *mut u8).add(FIELD_OFFSET + (array_index as usize * 4)).cast::<u32>().write_unaligned(value_bits);
    }
});
