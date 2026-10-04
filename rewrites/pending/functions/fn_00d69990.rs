// original: 0x00d69990 forward_byte_child
// s16f18: forward a flag byte to the child record's field (thiscall/1).
export!(thiscall, rw_s16f18(this: *const u8, flag: u32) -> () {
    unsafe {
        let child = *((this.add(4)) as *const u32);
        if child == 0 {
            return;
        }
        *((child + 0x18) as *mut u8) = (flag & 0xFF) as u8;
    }
});
