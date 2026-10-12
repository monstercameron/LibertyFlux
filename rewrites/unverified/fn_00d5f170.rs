// original: 0x00D5F170 material_fx_set_material_pair_lookup

/// Update the values associated with every matching material pair.
///
/// `this` points to a fixed table containing 42 groups of 42 records. Each
/// record is 12 bytes: its key at offset 0, followed by two 32-bit values at
/// offsets 4 and 8. The signed key `-1` or any smaller key returns without
/// scanning; otherwise every record is compared, and all records whose key
/// matches receive the two supplied values. The method is void at its known
/// call site; its EAX residue is intentionally outside this proof.
lf_checker_rt::export!(thiscall, rw_material_fx_set_material_pair_lookup(
    this_ptr: u32,
    key: u32,
    first_value: u32,
    second_value: u32
) -> u32 {
    unsafe {
        const GROUP_COUNT: u32 = 42;
        const RECORDS_PER_GROUP: u32 = 42;
        const RECORD_STRIDE: u32 = 12;
        const FIRST_RECORD_OFFSET: u32 = 8;
        const KEY_OFFSET_FROM_RECORD: u32 = 4;
        const FIRST_VALUE_OFFSET_FROM_RECORD: u32 = 0;
        const SECOND_VALUE_OFFSET_FROM_RECORD: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        if (key as i32) <= -1 {
            return 0;
        }

        let record_count = GROUP_COUNT * RECORDS_PER_GROUP;
        let mut record = this_ptr.wrapping_add(FIRST_RECORD_OFFSET);
        for _ in 0..record_count {
            let record_key = read_u32(record.wrapping_sub(KEY_OFFSET_FROM_RECORD));
            if record_key == key {
                write_u32(record.wrapping_add(FIRST_VALUE_OFFSET_FROM_RECORD), first_value);
                write_u32(record.wrapping_add(SECOND_VALUE_OFFSET_FROM_RECORD), second_value);
            }
            record = record.wrapping_add(RECORD_STRIDE);
        }
        0
    }
});
