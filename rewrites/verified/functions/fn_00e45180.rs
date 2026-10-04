// original: 0x00e45180 field_block_store_scale
// field block store with scale.
// Stores four words, two bytes and one float from the arguments into the
// block at this+0x348..0x35c. Returns the third word with its low byte
// replaced by the second byte argument (register residue of the original).
export!(thiscall, rw_00e45180(
    this_obj: u32,
    flag0: u32,
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
    flag1: u32,
    scale_bits: u32,
) -> u32 {
    unsafe {
        *((this_obj.wrapping_add(0x358)) as *mut u8) = (flag0 & 0xFF) as u8;
        *((this_obj.wrapping_add(0x350)) as *mut u32) = w0;
        *((this_obj.wrapping_add(0x354)) as *mut u32) = w1;
        *((this_obj.wrapping_add(0x348)) as *mut u32) = w2;
        *((this_obj.wrapping_add(0x34c)) as *mut u32) = w3;
        *((this_obj.wrapping_add(0x359)) as *mut u8) = (flag1 & 0xFF) as u8;
        *((this_obj.wrapping_add(0x35c)) as *mut u32) = scale_bits;
        (w1 & 0xFFFF_FF00) | (flag1 & 0xFF)
    }
});
