// original: 0x009b7690 rw_009b7690
/// Search the global pointer table for the first entry whose word at +0x53c
/// equals `want`. Returns the entry pointer, or null when the table is empty
/// or nothing matches.
export!(stdcall, rw_009b7690(want: u32) -> u32 {
    unsafe {
        const COUNT_G: u32 = 0x118D7F4;
        const TABLE_G: u32 = 0x118D7F0;
        const TAG_OFF: u32 = 0x53C;
        let count = *global::<u16>(COUNT_G) as u32;
        let table = *global::<u32>(TABLE_G) as *const u32;
        let mut i = 0u32;
        while i < count {
            let obj = *table.add(i as usize);
            if *((obj.wrapping_add(TAG_OFF)) as *const u32) == want {
                return obj;
            }
            i += 1;
        }
        0
    }
});
