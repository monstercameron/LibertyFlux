// original: 0x009b6e30 NativeImpl_DOES_VIEWPORT_EXIST
/// Scan the global object table for an entry whose key word matches.
///
/// Walks the first `count` slots of the table (both held in globals) and
/// compares the word at +0x53c of each entry against `key`. Returns 1 on the
/// first match (the entry is necessarily non-null there) and 0 when nothing
/// matches or the table is empty.
export!(stdcall, rw_009b6e30(key: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118D7F0;
        const COUNT: u32 = 0x0118D7F4;
        const KEY_OFF: u32 = 0x53C;
        let count = *global::<u16>(COUNT) as u32;
        if count == 0 {
            return 0;
        }
        let table = *global::<u32>(TABLE);
        let mut i = 0u32;
        loop {
            let entry = *((table.wrapping_add(i.wrapping_mul(4))) as *const u32);
            if *((entry.wrapping_add(KEY_OFF)) as *const u32) == key {
                return (entry != 0) as u32;
            }
            i += 1;
            if i >= count {
                return 0;
            }
        }
    }
});