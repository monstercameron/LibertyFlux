// original: 0x00e45e50 string_field_copy_04
// string field copy at +4.
// Copies the NUL-terminated string src to this+4; a null src stores an empty
// string instead. Returns the address just past the copied terminator, or 0
// for a null src.
export!(thiscall, rw_00e45e50(this_obj: u32, src: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 4;
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
