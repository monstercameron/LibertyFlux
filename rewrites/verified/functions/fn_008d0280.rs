// original: 0x008D0280 streaming_record_write_indexed_byte

/// For record index array_index, write the supplied byte to record offset 0x363C plus the element index. Return without writing for sentinel index -90 or a zero validity byte.
lf_checker_rt::export!(cdecl, rw_008d0280(index: u32, array_index: u32, value: u32) -> () {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FIELD_OFFSET: usize = 0x363C;
    unsafe {
        if index == NULL_INDEX { return; }
        let validity = lf_checker_rt::global::<u8>(STREAMING_VALIDITY_VA);
        if validity.add(index as usize).read() == 0 { return; }
let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        (record as *mut u8).add(FIELD_OFFSET + (array_index as usize)).cast::<u8>().write_unaligned(value as u8);
    }
});
