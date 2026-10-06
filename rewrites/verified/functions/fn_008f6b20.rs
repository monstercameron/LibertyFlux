// original: 0x008F6B20 Input_IsHotkeyActive

/// Report whether the hotkey is active, as a low byte (the upper 24 bits of
/// the result are incoming or intermediate garbage on several paths, so only
/// AL is meaningful): 1 when the first two id bytes xor above 0x7F while the
/// third xored with the first does not; otherwise, when the enable flag is
/// set, 1 when bit 0 of `W1 & (W1 ^ W2)` is set or the device query answers
/// nonzero. All byte comparisons are unsigned. Convention: cdecl, no stack
/// words.
lf_checker_rt::export!(cdecl, rw_008f6b20() -> u32 {
    unsafe {
        const ID0: u32 = 0x0118126C;
        const ID1: u32 = 0x0118126E;
        const ID2: u32 = 0x0118126F;
        const ENABLE: u32 = 0x0118198D;
        const WORD_B: u32 = 0x018B7A84;
        const WORD_A: u32 = 0x018B7A88;
        const DEVICE: u32 = 0x0118D110;
        const QUERY: u32 = 1;
        let b0 = lf_checker_rt::global::<u8>(ID0).read();
        if b0 ^ lf_checker_rt::global::<u8>(ID1).read() > 0x7F {
            if lf_checker_rt::global::<u8>(ID2).read() ^ b0 <= 0x7F {
                return 1;
            }
        }
        if lf_checker_rt::global::<u8>(ENABLE).read() == 0 {
            return 0;
        }
        let a = lf_checker_rt::global::<u32>(WORD_A).read_unaligned();
        let b = lf_checker_rt::global::<u32>(WORD_B).read_unaligned();
        if a & (a ^ b) & 1 != 0 {
            return 1;
        }
        let r: u32 =
            lf_checker_rt::callee_thiscall!(QUERY, u32, lf_checker_rt::relocated(DEVICE), 0x39, 1, 0);
        if r & 0xFF != 0 {
            1
        } else {
            0
        }
    }
});
