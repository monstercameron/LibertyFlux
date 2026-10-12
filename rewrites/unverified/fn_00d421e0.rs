// original: 0x00D421E0 CTaskComplexJump::set_state

/// Replace the low six state bits in a jump task's flags word at +0x3c. The
/// previous upper bits are preserved, the supplied stack value is ORed into
/// the low word, and the updated word is returned through AX. ECX points to
/// the task and thiscall consumes one stack word.
lf_checker_rt::export!(thiscall, rw_D421E0(this: u32, state_bits: u32) -> u32 {
    const STATE_WORD: u32 = 0x3c;
    const PRESERVED_MASK: u16 = 0xffc0;
    unsafe {
        let word = (this + STATE_WORD) as *mut u16;
        let current = word.read_unaligned();
        let updated = (current & PRESERVED_MASK) | state_bits as u16;
        word.write_unaligned(updated);
        updated as u32
    }
});
