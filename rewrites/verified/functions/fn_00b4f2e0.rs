// original: 0x00B4F2E0 ped_set_word3a2_and_byte3a4

/// Store a mode word and a sub-mode byte into a ped extension block.
///
/// Writes the constant 1 as a 16-bit word at `this + 0x3A2` and the low byte
/// of `value` at `this + 0x3A4`. Returns the stored byte in the low byte of
/// the result (the original only sets al, leaving the caller's upper bits).
///
/// Original: 0x00B4F2E0 (thiscall, `this` in ecx, one stack word).
export!(thiscall, rw_00b4f2e0(this: u32, value: u32) -> u32 {
    {
        const MODE_WORD: u32 = 0x3A2;
        const SUBMODE_BYTE: u32 = 0x3A4;
        unsafe {
            ((this + MODE_WORD) as *mut u16).write_unaligned(1);
            ((this + SUBMODE_BYTE) as *mut u8).write(value as u8);
        }
        value & 0xFF
    }
});
