// original: 0x00A4DC90 vehicle_clear_flags (proposed)

/// Clears bits 3 and 4 of the flag byte and zeroes a state byte.
///
/// `mem[this + FLAGS] &= CLEAR_MASK` (0xE7: keeps bits 0-2 and 5-7) and
/// `mem[this + STATE]` (0x12F4) `= 0`. Returns nothing (`eax` is untouched,
/// so the contract compares no return channel).
///
/// Original: 0x00A4DC90 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4DC90(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x0F14;
        const STATE: u32 = 0x12F4;
        const CLEAR_MASK: u8 = 0xE7;
        let f = (this + FLAGS) as *mut u8;
        f.write(f.read() & CLEAR_MASK);
        ((this + STATE) as *mut u8).write(0);
        0
    }
});
