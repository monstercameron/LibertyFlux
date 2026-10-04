// original: 0x00B4FFC0 CDummyPed::vf55

/// Store a word into a dummy ped at offset 0x110 and return it.
///
/// Original: 0x00B4FFC0 (thiscall, `this` in ecx, one stack word).
export!(thiscall, rw_00b4ffc0(this: u32, value: u32) -> u32 {
    const SLOT: u32 = 0x110;
    unsafe {
        ((this + SLOT) as *mut u32).write_unaligned(value);
    }
    value
});
