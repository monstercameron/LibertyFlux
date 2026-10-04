// original: 0x00d69910 forward_byte_slot
// s16f15: forward a flag byte to the slot record's field (thiscall/1).
export!(thiscall, rw_s16f15(this: *const u8, flag: u32) -> () {
    unsafe {
        let slot = *((this.add(8)) as *const u32);
        if slot == 0 {
            return;
        }
        *((slot.wrapping_add(0x18)) as *mut u8) = (flag & 0xFF) as u8;
    }
});
