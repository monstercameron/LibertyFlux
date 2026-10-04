// original: 0x008b75e0 meter_table_lookup_byte
/// Meter reading table lookup.
///
/// Samples the meter three times through the shared handle. The first sample
/// must be non-negative and agree with the second up to the live group's entry
/// count; the third sample then selects a byte from the group's 0x16-byte
/// entry array. Returns 0 whenever a sample is out of range or the group is
/// empty.
export!(cdecl, rw_008b75e0() -> u32 {
    unsafe {
        const HANDLE: u32 = 0x01160C0C;
        const LIVE_GROUP: u32 = 0x01160C40;
        const COUNT_TABLE: u32 = 0x019D33A4;
        const BASE_TABLE: u32 = 0x019D33A0;
        const ENTRY_LEN: u32 = 0x16;
        let handle = (relocated(HANDLE) as *const u32).read();
        let first: u32 = callee_cdecl!(2, u32, handle);
        if (first as i32) < 0 {
            return 0;
        }
        let second: u32 = callee_cdecl!(2, u32, handle);
        let group = (relocated(LIVE_GROUP) as *const u32).read();
        let row = group.wrapping_mul(3);
        let count_addr = relocated(COUNT_TABLE).wrapping_add(row.wrapping_mul(8));
        let count = (count_addr as *const u16).read() as u32;
        if (second as i32) > (count as i32) {
            return 0;
        }
        if (count as u16) == 0 {
            return 0;
        }
        let third: u32 = callee_cdecl!(2, u32, handle);
        let base_addr = relocated(BASE_TABLE).wrapping_add(row.wrapping_mul(8));
        let base = (base_addr as *const u32).read();
        let cell = base.wrapping_add(third.wrapping_mul(ENTRY_LEN));
        (cell as *const u8).read() as u32
    }
});
