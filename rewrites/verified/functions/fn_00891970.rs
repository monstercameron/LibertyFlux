// original: 0x00891970 aud_state_flag_update
/// Sets status flag bits from the state word at offset 6.
///
/// When the word equals 2, sets bit 2 of the flag byte at 0x39 and bit 4 of
/// the flag byte at 0x38; otherwise sets bit 3 of the byte at 0x38.
export!(thiscall, rw_00891970(this: *mut u8) -> () {
    unsafe {
        let state = *(this.add(6) as *const u16);
        if state == 2 {
            *(this.add(0x39)) |= 4;
            *(this.add(0x38)) |= 0x10;
        } else {
            *(this.add(0x38)) |= 8;
        }
    }
});
