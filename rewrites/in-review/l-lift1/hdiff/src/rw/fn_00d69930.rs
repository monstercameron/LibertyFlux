// original: 0x00d69930 forward_byte_aux
// s16f16: forward a flag byte to the aux record's field (thiscall/1).
export!(thiscall, rw_s16f16(this: *const u8, flag: u32) -> () {
    unsafe {
        let aux = *((this.add(0x10)) as *const u32);
        if aux == 0 {
            return;
        }
        *((aux.wrapping_add(0x18)) as *mut u8) = (flag & 0xFF) as u8;
    }
});
