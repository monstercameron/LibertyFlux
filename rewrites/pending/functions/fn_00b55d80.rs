// original: 0x00b55d80 scan_global_table_by_direct_keys
/// Scan the shared pointer table for a record matching two u16 keys.
///
/// Same table walk as fn_00b55d20, but the keys come straight from the
/// argument (+0x18, then +0x16 once the first matches) instead of through a
/// nested object. Returns the record's third dword on the first full match,
/// else 0.
export!(cdecl, rw_00b55d80(arg0: u32) -> u32 {
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
                let key_a = ((arg0 + 0x18) as *const u16).read() as u32;
                if (entry as *const u32).read() == key_a {
                    let key_b = ((arg0 + 0x16) as *const u16).read() as u32;
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
