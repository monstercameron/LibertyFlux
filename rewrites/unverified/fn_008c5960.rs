// original: 0x008C5960 proposed
/// Search eight-byte records between signed inclusive bounds. The dword at
/// record offset +4 is compared as unsigned. A match returns the record
/// address; otherwise the function returns zero.
lf_checker_rt::export!(stdcall, rw_008c5960(low: u32, records: u32, key: u32, high: u32) -> u32 {
    unsafe {
        let mut low = low as i32;
        let mut high = high as i32;
        while low <= high {
            let sum = low.wrapping_add(high);
            let midpoint = sum.wrapping_add(i32::from(sum < 0)) >> 1;
            let record = records.wrapping_add((midpoint as u32).wrapping_mul(8));
            let record_key = (record.wrapping_add(4) as *const u32).read_unaligned();
            if key == record_key { return record; }
            if key > record_key { low = midpoint.wrapping_add(1); }
            else { high = midpoint.wrapping_sub(1); }
        }
        0
    }
});
