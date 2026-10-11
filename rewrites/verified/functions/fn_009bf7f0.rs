// original: 0x009BF7F0 ped_task_record_lookup
/// Finds the first one of 32 fixed-stride task records whose 32-bit key at
/// byte offset 0x10 equals `key`. A zero key immediately returns null. On a
/// match it returns the record's relocated image address; otherwise it
/// returns zero. The table is global writable image data shared with the
/// checker, so reads use the runtime's relocated global pointer.
lf_checker_rt::export!(cdecl, rw_009bf7f0(key: u32) -> u32 {
    const TABLE_FIRST_KEY: u32 = 0x0129_2460;
    const TABLE_RECORDS_END: u32 = 0x0129_3E60;
    const TABLE_RECORD_BASE: u32 = 0x0129_2450;
    const RECORD_STRIDE: u32 = 0xD0;

    if key == 0 {
        return 0;
    }

    let mut key_address = TABLE_FIRST_KEY;
    let mut record_index = 0_u32;
    while key_address < TABLE_RECORDS_END {
        let stored_key = unsafe { lf_checker_rt::global::<u32>(key_address).read() };
        if stored_key == key {
            return lf_checker_rt::relocated(
                TABLE_RECORD_BASE.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE)),
            );
        }
        key_address = key_address.wrapping_add(RECORD_STRIDE);
        record_index = record_index.wrapping_add(1);
    }
    0
});
