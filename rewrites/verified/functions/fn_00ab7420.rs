// original: 0x00ab7420 pair_table_find
/// Linear search of `count` 0x20-byte entries for (key0,key1); returns the
/// entry address, or the shared empty-entry address when not found.

export!(thiscall, rw_00ab7420(this: *const u8, key0: u32, key1: u32) -> u32 {
    unsafe {
        let count = *(this.add(4) as *const u16) as i32;
        if count <= 0 {
            return relocated(0xEA5268);
        }
        let base = *(this as *const u32) as *const u8;
        let mut i = 0i32;
        while i < count {
            let e = base.add((i as usize) * 0x20);
            if *(e as *const u32) == key0 && *(e.add(4) as *const u32) == key1 {
                return e as u32;
            }
            i += 1;
        }
        relocated(0xEA5268)
    }
});
