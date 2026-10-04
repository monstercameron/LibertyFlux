// original: 0x00ab74d0 pair_table_index
/// Same search as pair_table_find but returns the entry index, or -1.

export!(thiscall, rw_00ab74d0(this: *const u8, key0: u32, key1: u32) -> u32 {
    unsafe {
        let count = *(this.add(4) as *const u16) as i32;
        if count <= 0 {
            return 0xFFFFFFFF;
        }
        let base = *(this as *const u32) as *const u8;
        let mut i = 0i32;
        while i < count {
            let e = base.add((i as usize) * 0x20);
            if *(e as *const u32) == key0 && *(e.add(4) as *const u32) == key1 {
                return i as u32;
            }
            i += 1;
        }
        0xFFFFFFFF
    }
});
