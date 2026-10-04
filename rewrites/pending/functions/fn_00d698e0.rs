// original: 0x00d698e0 forward_byte_head
// s16f13: forward a flag byte to the head record's field (thiscall/1).
export!(thiscall, rw_s16f13(this: *const u8, flag: u32) -> () {
    unsafe {
        let head = *(this as *const u32);
        if head == 0 {
            return;
        }
        *((head.wrapping_add(0x18)) as *mut u8) = (flag & 0xFF) as u8;
    }
});
