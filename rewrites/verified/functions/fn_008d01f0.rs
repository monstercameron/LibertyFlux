// original: 0x008D01F0 streaming_record_has_flag_byte

/// Return whether the selected byte in the record flag area is nonzero; sentinel index -90 or a null record returns false.
lf_checker_rt::export!(cdecl, rw_008d01f0(index: u32, byte_index: u32) -> u8 {
    const STREAMING_RECORDS_VA: u32 = 0x0011_73560;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const FLAGS_OFFSET: usize = 0x3416;
    unsafe {
        if index == NULL_INDEX { return 0; }
        let records = lf_checker_rt::global::<u32>(STREAMING_RECORDS_VA);
        let record = records.add(index as usize).read_unaligned();
        if record == 0 { return 0; }
        u8::from((record as *const u8).add(FLAGS_OFFSET + byte_index as usize).read() != 0)
    }
});
