// original: 0x00e45e80 string_field_copy_40
// string field copy at +0x40.
// Same as 0x00e45e50 with the destination at this+0x40.
export!(thiscall, rw_00e45e80(this_obj: u32, src: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x40;
        if src == 0 {
            *((this_obj.wrapping_add(FIELD_OFF)) as *mut u8) = 0;
            return 0;
        }
        let mut from = src as *const u8;
        let mut to = this_obj.wrapping_add(FIELD_OFF) as *mut u8;
        loop {
            let b = *from;
            *to = b;
            from = from.add(1);
            to = to.add(1);
            if b == 0 {
                break;
            }
        }
        from as u32
    }
});
