// original: 0x005B5E30 record_table_write_nontagged
/// Finds the first record-table slot tagged 3 whose second word is not 5,
/// and stores the three given values into its data words. Does nothing when
/// no slot matches. EAX is incidental (entry garbage when empty), so callers
/// must ignore it.
export!(thiscall, rw_005B5E30(obj: *const u8, v0: u32, v1: u32, v2: u32) -> u32 {
    unsafe {
        let base = *(obj as *const u32) as *mut u32;
        let count = *(obj.add(4) as *const u16) as u32;
        let mut i: u32 = 0;
        while i < count {
            let slot = base.add((i as usize) * 4);
            if slot.add(0).read() == 3 && slot.add(1).read() != 5 {
                slot.add(1).write(v0);
                slot.add(2).write(v1);
                slot.add(3).write(v2);
                return 0;
            }
            i += 1;
        }
        0
    }
});
