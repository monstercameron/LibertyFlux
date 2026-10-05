// original: 0x00bebdd0 split_u24_be_123
/// Store the low 24 bits of a value as three big-endian bytes at +1..+3.
///
/// Writes bits 23-16 to byte 3, bits 15-8 to byte 2 and bits 7-0 to byte 1
/// of `this`. Returns the value shifted right by 8. Thiscall, one stack
/// argument.
export!(thiscall, rw_00bebdd0(this: u32, val: u32) -> u32 {
    unsafe {
        ((this + 3) as *mut u8).write((val >> 16) as u8);
        ((this + 2) as *mut u8).write((val >> 8) as u8);
        ((this + 1) as *mut u8).write(val as u8);
        val >> 8
    }
});
