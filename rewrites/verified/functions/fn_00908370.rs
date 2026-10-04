// original: 0x00908370 radar_scratch_clear
/// Clear the radar/blip scratch globals.
///
/// Zeroes two flag bytes and five related dwords. Takes no arguments and
/// returns nothing meaningful.
export!(cdecl, rw_00908370() -> u32 {
    unsafe {
        *global::<u8>(0x118F4BD) = 0;
        *global::<u8>(0x118F4BE) = 0;
        *global::<u32>(0x118F4C8) = 0;
        *global::<u32>(0x118F4CC) = 0;
        *global::<u32>(0x118F4D4) = 0;
        *global::<u32>(0x118F4D8) = 0;
        *global::<u32>(0x118F4D0) = 0;
        0
    }
});
