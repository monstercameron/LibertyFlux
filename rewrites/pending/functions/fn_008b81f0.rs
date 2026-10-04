// original: 0x008b81f0 button_code_scan
/// Button scan by leading-byte code.
///
/// Scans the `group` entry list (valid range 0..=0x48) for the first 0x16-byte
/// entry whose leading byte equals `code`, and returns the signed 16-bit value
/// at offset 0x12 of that entry. Returns 0x7fffffff when the group is out of
/// range, the list is empty, or no entry matches. The comparison is a full
/// 32-bit compare, so codes above 0xff never match.
export!(cdecl, rw_008b81f0(group: u32, code: u32) -> u32 {
    unsafe {
        const COUNT_TABLE: u32 = 0x019D33A4;
        const BASE_TABLE: u32 = 0x019D33A0;
        const ENTRY_LEN: u32 = 0x16;
        const CODE_OFF: u32 = 0x12;
        const GROUPS: u32 = 0x48;
        const NONE: u32 = 0x7FFF_FFFF;
        if group > GROUPS {
            return NONE;
        }
        let row = group.wrapping_mul(3);
        let count_addr = relocated(COUNT_TABLE).wrapping_add(row.wrapping_mul(8));
        let count = (count_addr as *const u16).read() as u32;
        if (count as i32) <= 0 {
            return NONE;
        }
        let base_addr = relocated(BASE_TABLE).wrapping_add(row.wrapping_mul(8));
        let base = (base_addr as *const u32).read();
        let mut i: u32 = 0;
        let mut cursor = base;
        loop {
            let head = (cursor as *const u8).read() as u32;
            if code == head {
                let code_addr = base
                    .wrapping_add(i.wrapping_mul(ENTRY_LEN))
                    .wrapping_add(CODE_OFF);
                return (code_addr as *const i16).read() as i32 as u32;
            }
            i = i.wrapping_add(1);
            cursor = cursor.wrapping_add(ENTRY_LEN);
            if !((i as i32) < (count as i32)) {
                return NONE;
            }
        }
    }
});
