// original: 0x00A76520 vehicle_global_id_index

/// Return the first index in the 133-word vehicle id table whose value equals
/// the cdecl argument. The table is read from the relocated global address;
/// the search is forward and returns `u32::MAX` when no word matches.
lf_checker_rt::export!(cdecl, rw_00a76520(search_value: u32) -> u32 {
    const VEHICLE_ID_TABLE: u32 = 0x0103D8A8;
    const TABLE_LENGTH: u32 = 0x85;

    unsafe {
        let entries = lf_checker_rt::global::<u32>(VEHICLE_ID_TABLE);
        let mut index = 0u32;
        while index < TABLE_LENGTH {
            if entries.add(index as usize).read_unaligned() == search_value {
                return index;
            }
            index += 1;
        }
        u32::MAX
    }
});
