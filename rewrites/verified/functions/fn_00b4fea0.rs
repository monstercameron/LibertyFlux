// original: 0x00B4FEA0 ped_set_byte3a2

/// Store a flag byte into a ped extension block at offset 0x3A2.
///
/// Writes the low byte of `value` to `this + 0x3A2` and returns it in the low
/// byte of the result (upper bits are the caller's leftovers).
///
/// Original: 0x00B4FEA0 (thiscall, `this` in ecx, one stack word).
export!(thiscall, rw_00b4fea0(this: u32, value: u32) -> u32 {
    const FLAG_BYTE: u32 = 0x3A2;
    unsafe {
        ((this + FLAG_BYTE) as *mut u8).write(value as u8);
    }
    value & 0xFF
});
