// original: 0x00962690 slot_array_clear
/// Clear the 1500-entry slot array at 0x11F7110 and the mark bytes.
///
/// For each 8-byte slot, clears a trailer block selected like
/// `rw_00962180` (a clear flag byte clears the slot pointer's own trailer,
/// a set flag clears the trailer of the block its head word points at),
/// then clears the slot itself. Also clears the 1500 mark bytes at
/// 0x11F6958. Returns the last slot index (0x5DB).
export!(cdecl, rw_00962690() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11F7110;
        const MARKS: u32 = 0x11F6958;
        const COUNT: u32 = 0x5DC;
        for idx in 0..COUNT {
            let row = (relocated(TABLE) as *mut u8).add((idx * 8) as usize);
            let flag = *(row.add(4));
            let ptr = *(row as *const u32);
            if flag == 0 {
                if ptr != 0 {
                    *((ptr as *mut u32).add(0xAC / 4)) = 0;
                    *((ptr as *mut u32).add(0xA8 / 4)) = 0;
                    *((ptr as *mut u32).add(0xA4 / 4)) = 0xFFFFFFFF;
                }
            } else if ptr != 0 {
                let head = *(ptr as *const u32);
                if head != 0 {
                    *((head as *mut u32).add(0xAC / 4)) = 0;
                    *((head as *mut u32).add(0xA8 / 4)) = 0;
                    *((head as *mut u32).add(0xA4 / 4)) = 0xFFFFFFFF;
                }
            }
            *(row as *mut u32) = 0;
            *(row.add(4)) = 0;
            *((relocated(MARKS) as *mut u8).add(idx as usize)) = 0;
        }
        COUNT - 1
    }
});
