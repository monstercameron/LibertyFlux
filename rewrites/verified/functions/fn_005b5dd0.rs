// original: 0x005B5DD0 record_table_read_nontagged
/// Finds the first record-table slot tagged 3 whose second word is not 5,
/// and copies its three data words to the three out-pointers. Does nothing
/// when no slot matches. EAX is incidental (entry garbage when empty), so
/// callers must ignore it.
export!(thiscall, rw_005B5DD0(obj: *const u8, out0: *mut u32, out1: *mut u32, out2: *mut u32) -> u32 {
    unsafe {
        let base = *(obj as *const u32) as *const u32;
        let count = *(obj.add(4) as *const u16) as u32;
        let mut i: u32 = 0;
        while i < count {
            let slot = base.add((i as usize) * 4);
            if slot.add(0).read() == 3 && slot.add(1).read() != 5 {
                out0.write(slot.add(1).read());
                out1.write(slot.add(2).read());
                out2.write(slot.add(3).read());
                return 0;
            }
            i += 1;
        }
        0
    }
});
