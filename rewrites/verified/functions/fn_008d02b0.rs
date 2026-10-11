// original: 0x008D02B0 streaming_record_write_three_dwords

/// When the indexed validity byte is nonzero, write the three supplied dwords to record offsets 0x3630, 0x3634 and 0x3638. Sentinel index -90 or invalid status leaves the record unchanged.
lf_checker_rt::export!(cdecl, rw_008d02b0(index: u32, first: u32, second: u32, third: u32) -> () {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const STREAMING_VALIDITY_VA: u32 = 0x0011_7354C;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    unsafe {
        if index == NULL_INDEX { return; }
        let validity = lf_checker_rt::global::<u8>(STREAMING_VALIDITY_VA);
        if validity.add(index as usize).read() == 0 { return; }
        let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned() as *mut u8;
        (record.add(0x3630).cast::<u32>()).write_unaligned(first);
        (record.add(0x3634).cast::<u32>()).write_unaligned(second);
        (record.add(0x3638).cast::<u32>()).write_unaligned(third);
    }
});
