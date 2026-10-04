// original: 0x00b55d20 scan_global_table_by_nested_keys
/// Scan the shared pointer table for a record matching two nested u16 keys.
///
/// The table base and entry count come from fixed globals. The keys are read
/// per candidate from arg+8/+4 (+0x18, then +0x16 once the first matches) and
/// compared against the record's first two dwords; the record's third dword
/// is returned on the first full match, else 0.
export!(cdecl, rw_00b55d20(arg0: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0166d9b8;
        const COUNT: u32 = 0x0166d9bc;
        let count = (global::<u16>(COUNT)).read();
        if count == 0 {
            return 0;
        }
        let table = (global::<u32>(TABLE)).read();
        let mut i: u32 = 0;
        while i < count as u32 {
            let entry = ((table + i * 4) as *const u32).read();
            if entry != 0 {
                let outer = ((arg0 + 8) as *const u32).read();
                let keys = ((outer + 4) as *const u32).read();
                let key_a = ((keys + 0x18) as *const u16).read() as u32;
                if (entry as *const u32).read() == key_a {
                    let key_b = ((keys + 0x16) as *const u16).read() as u32;
                    if ((entry + 4) as *const u32).read() == key_b {
                        return ((entry + 8) as *const u32).read();
                    }
                }
            }
            i += 1;
        }
        0
    }
});
