// original: 0x008d6ae0 record_init
/// Initialise an 8-field record: three tag words to -1, the rest to zero.
///
/// `this` points at 0x20 bytes: words at +0x10/+0x14/+0x18 become all-ones,
/// words at +0/+4/+8/+0xC and the byte at +0x1C become zero.
export!(thiscall, rw_008d6ae0(this_: *mut u32) -> u32 {
    unsafe {
        // Tag words first, then the zeroed fields, in the original's order.
        *this_.byte_add(0x10) = 0xFFFF_FFFF;
        *this_.byte_add(0x14) = 0xFFFF_FFFF;
        *this_.byte_add(0x18) = 0xFFFF_FFFF;
        *this_.byte_add(0x04) = 0;
        *this_ = 0;
        *this_.byte_add(0x0C) = 0;
        *this_.byte_add(0x08) = 0;
        *(this_ as *mut u8).byte_add(0x1C) = 0;
        0
    }
});
