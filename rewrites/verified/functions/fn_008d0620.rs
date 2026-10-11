// original: 0x008D0620 streaming_record_set_money

/// For record index 0, write the supplied dword to record offset 0x34F8. Return without writing for sentinel index -90 or a zero validity byte.
lf_checker_rt::export!(cdecl, rw_008d0620(index: u32, value: u32) -> () {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FIELD_OFFSET: usize = 0x34F8;
    unsafe {
        if index == NULL_INDEX { return; }
        let validity = lf_checker_rt::global::<u8>(STREAMING_VALIDITY_VA);
        if validity.add(index as usize).read() == 0 { return; }
let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        (record as *mut u8).add(FIELD_OFFSET).cast::<u32>().write_unaligned(value);
    }
});
