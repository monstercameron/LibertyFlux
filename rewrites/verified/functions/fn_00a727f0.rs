// original: 0x00a727f0 has_both_state_bits (proposed)
/// True (1) when the looked-up state word has both bit 5 and bit 12 set.
///
/// `thiscall`, no stack words. Reads the key at `this+0x20`, resolves it
/// through the registry callee (one-word cdecl), reads the state word at
/// `+0x20` off the result, and tests the two bits. Returns 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00a727f0(this: u32) -> u8 {
    unsafe {
        const KEY_OFF: u32 = 0x20;
        const STATE_OFF: u32 = 0x20;
        const BIT_A: u32 = 1 << 5;
        const BIT_B: u32 = 1 << 12;
        let key = ((this + KEY_OFF) as *const u32).read_unaligned();
        let ent = lf_checker_rt::callee_cdecl!(1, u32, key);
        let w = ((ent + STATE_OFF) as *const u32).read_unaligned();
        (w & BIT_A != 0 && w & BIT_B != 0) as u8
    }
});
