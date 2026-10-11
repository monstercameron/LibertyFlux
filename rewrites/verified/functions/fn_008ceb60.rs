// original: 0x008CEB60 streaming_record_get_field_34f8

/// Return the dword at offset 0x34F8 of a valid indexed streaming record. Index -90, a null record, or a zero validity byte returns zero.
lf_checker_rt::export!(cdecl, rw_008ceb60(index: u32) -> u32 {
    const TABLE_VA: u32 = 0x0011_73560;
    const NULL_INDEX: u32 = 0xFFFF_FFA6;
    const VALUE_FIELD: u32 = 0x34F8;
    unsafe {
        if index == NULL_INDEX {
            return 0;
        }
        let table = lf_checker_rt::global::<u32>(TABLE_VA);
        let status = lf_checker_rt::global::<u8>(0x0011_7354C);
        let record = table.add(index as usize).read_unaligned();
        if record == 0 {
            return 0;
        }
        if status.add(index as usize).read() == 0 {
            return 0;
        }
        (record.wrapping_add(VALUE_FIELD) as *const u32).read_unaligned()
    }
});
