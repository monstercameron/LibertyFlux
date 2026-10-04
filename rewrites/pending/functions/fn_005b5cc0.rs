// original: 0x005B5CC0 record_table_find_tagged
/// Scans the record table for the first slot tagged (3,5) and returns its
/// payload word, or 0 when no slot matches.
export!(thiscall, rw_005B5CC0(obj: *const u8) -> u32 {
    unsafe {
        let base = *(obj as *const u32) as *const u32;
        let count = *(obj.add(4) as *const u16) as u32;
        let mut i: u32 = 0;
        while i < count {
            let slot = base.add((i as usize) * 4);
            if slot.add(0).read() == 3 && slot.add(1).read() == 5 {
                return slot.add(2).read();
            }
            i += 1;
        }
        0
    }
});
