// original: 0x009f0fe0 ped_slot_table_remove
/// Remove a record from the global slot table and count it out.
///
/// Marks the record itself with -1, then scans that record's row (row
/// `rec[1]`, 0x78 words) of the table at `0x12b41d8` for the record's own
/// address. A miss returns the scan-end address. A hit at index `i` clears
/// the table word, decrements counter `rec[1]` at `0x12b5858`, and returns
/// the row number.
export!(cdecl, rw_009f0fe0(rec: u32) -> u32 {
    unsafe {
        let row = *((rec + 4) as *const u32);
        *(rec as *mut u32) = 0xffffffff;
        let base = lf_checker_rt::relocated(0x12b41d8);
        let mut ptr = base.wrapping_add(row.wrapping_mul(0x1e0));
        let mut index: u32 = 0;
        while index < 0x78 {
            if *(ptr as *const u32) == rec {
                break;
            }
            index += 1;
            ptr = ptr.wrapping_add(4);
        }
        if index == 0x78 {
            return ptr;
        }
        let flat = index.wrapping_add(row.wrapping_mul(120));
        *(base.wrapping_add(flat.wrapping_mul(4)) as *mut u32) = 0;
        let counter = lf_checker_rt::relocated(0x12b5858).wrapping_add(row.wrapping_mul(4))
            as *mut u32;
        *counter = (*counter).wrapping_sub(1);
        row
    }
});
