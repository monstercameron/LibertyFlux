// original: 0x00B4FFB0 CDummyPed::vf56

/// Store a flag byte into a dummy ped at offset 0x114.
///
/// Writes the low byte of `value` to `this + 0x114` and returns it in the low
/// byte of the result (upper bits are the caller's leftovers).
///
/// Original: 0x00B4FFB0 (thiscall, `this` in ecx, one stack word).
export!(thiscall, rw_00b4ffb0(this: u32, value: u32) -> u32 {
    const FLAG_BYTE: u32 = 0x114;
    unsafe {
        ((this + FLAG_BYTE) as *mut u8).write(value as u8);
    }
    value & 0xFF
});
