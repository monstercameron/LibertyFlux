// original: 0x00a25270 camera_record_store_two_float_words

/// Copy two binary32 argument words into the camera record at offsets 0x22c
/// and 0x230, then set its ready byte at 0x234. Payload bits are preserved.
lf_checker_rt::export!(thiscall, rw_00a25270(this: u32, first_bits: u32, second_bits: u32) -> u32 {
    unsafe {
        (this.wrapping_add(0x22c) as *mut u32).write_unaligned(first_bits);
        (this.wrapping_add(0x234) as *mut u8).write(1);
        (this.wrapping_add(0x230) as *mut u32).write_unaligned(second_bits);
    }
    0
});
