// original: 0x00d698f0 forward_byte_linked
// s16f14: forward a status byte to the linked record's field (thiscall/1).
export!(thiscall, rw_s16f14(this: *const u8, flag: u32) -> () {
    unsafe {
        let linked = *((this.add(0xC)) as *const u32);
        if linked == 0 {
            return;
        }
        *((linked.wrapping_add(0xFC)) as *mut u8) = (flag & 0xFF) as u8;
    }
});
