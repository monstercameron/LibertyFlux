// original: 0x00d5a600 ccam_market_init
/// Initialise a market camera: zero the state words, set the two default
/// float parameters, clear the trailing mode word and flag.
///
/// Zeroes the dwords at `+0x140`, `+0x144`, `+0x150`, `+0x154`, `+0x14c` and
/// `+0x158`, writes the default parameters `DEFAULT_A` (5.0) at `+0x148` and
/// `DEFAULT_B` (45.0) at `+0x15c`, clears the word at `+0x160` and the byte
/// at `+0x162`. Returns 1.
///
/// Original: thiscall, no stack arguments, returns 1 in `al`.
lf_checker_rt::export!(thiscall, rw_00d5a600 (this: u32) -> u32 {
    unsafe {
        const DEFAULT_A: u32 = 0x40A00000;
        const DEFAULT_B: u32 = 0x42340000;
        for off in [0x140u32, 0x144, 0x150, 0x154, 0x14c, 0x158] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + 0x148) as *mut u32).write_unaligned(DEFAULT_A);
        ((this + 0x15c) as *mut u32).write_unaligned(DEFAULT_B);
        ((this + 0x160) as *mut u16).write_unaligned(0);
        ((this + 0x162) as *mut u8).write(0);
        1
    }
});
