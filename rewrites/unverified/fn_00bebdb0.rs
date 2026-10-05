// original: 0x00bebdb0 split_u24_be_567
/// Store the low 24 bits of a value as three big-endian bytes at +5..+7.
///
/// Writes bits 23-16 to byte 7, bits 15-8 to byte 6 and bits 7-0 to byte 5
/// of `this`; byte 31-24 are ignored. Returns the value shifted right by 8
/// (the original's `shr` leftover in EAX). Thiscall, one stack argument.
export!(thiscall, rw_00bebdb0(this: u32, val: u32) -> u32 {
    unsafe {
        ((this + 7) as *mut u8).write((val >> 16) as u8);
        ((this + 6) as *mut u8).write((val >> 8) as u8);
        ((this + 5) as *mut u8).write(val as u8);
        val >> 8
    }
});
