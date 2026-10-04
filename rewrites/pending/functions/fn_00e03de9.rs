// original: 0x00e03de9 table_key_lookup
/// Looks up a key in a 23-entry key/value table, returning the value or 0.
///
/// Scans the stride-8 table at 0xF0D838 linearly; on the first entry whose
/// key equals the argument, returns the paired value, otherwise 0.
export!(cdecl, rw_00e03de9(key: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0xF0D838;
        const COUNT: u32 = 23;
        let entries = global::<[u32; 46]>(TABLE);
        let mut i = 0u32;
        while i < COUNT {
            if (*entries)[(i * 2) as usize] == key {
                return (*entries)[(i * 2 + 1) as usize];
            }
            i += 1;
        }
        0
    }
});
