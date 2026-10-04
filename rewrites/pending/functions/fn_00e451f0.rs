// original: 0x00e451f0 field_block_store_ext
// extended field block store.
// Stores eight words and two bytes from the arguments into the block at
// this+0x360..0x381. Returns the last word.
export!(thiscall, rw_00e451f0(
    this_obj: u32,
    flag0: u32,
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
    flag1: u32,
    w4: u32,
    w5: u32,
    w6: u32,
    w7: u32,
) -> u32 {
    unsafe {
        *((this_obj.wrapping_add(0x380)) as *mut u8) = (flag0 & 0xFF) as u8;
        *((this_obj.wrapping_add(0x370)) as *mut u32) = w0;
        *((this_obj.wrapping_add(0x374)) as *mut u32) = w1;
        *((this_obj.wrapping_add(0x378)) as *mut u32) = w2;
        *((this_obj.wrapping_add(0x37c)) as *mut u32) = w3;
        *((this_obj.wrapping_add(0x381)) as *mut u8) = (flag1 & 0xFF) as u8;
        *((this_obj.wrapping_add(0x360)) as *mut u32) = w4;
        *((this_obj.wrapping_add(0x364)) as *mut u32) = w5;
        *((this_obj.wrapping_add(0x368)) as *mut u32) = w6;
        *((this_obj.wrapping_add(0x36c)) as *mut u32) = w7;
        w7
    }
});
