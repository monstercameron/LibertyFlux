// original: 0x00CAB2F0 handler_state_clear (proposed)

/// Clear a handler state block and return its address.
///
/// Writes a zero dword at `+0x00` and zero bytes at `+0x04`, `+0x24` and
/// seven flag pairs: for each pair base in 0x3C, 0x74, 0xAC, 0xE4, 0x11C,
/// 0x154, 0x18C (step 0x38), the bytes at base and base+0x20 are cleared
/// (the original writes that flag range twice; the second pass is
/// redundant). Returns `this` in eax. No calls.
///
/// Original: 0x00CAB2F0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cab2f0(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const PAIR_FIRST: u32 = 0x3C;
        const PAIR_LAST: u32 = 0x18C;
        const PAIR_STEP: u32 = 0x38;
        const PAIR_SECOND: u32 = 0x20;
        (this.wrapping_add(HEAD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x04) as *mut u8).write(0);
        (this.wrapping_add(0x24) as *mut u8).write(0);
        let mut base = PAIR_FIRST;
        while base <= PAIR_LAST {
            (this.wrapping_add(base) as *mut u8).write(0);
            (this.wrapping_add(base + PAIR_SECOND) as *mut u8).write(0);
            base += PAIR_STEP;
        }
        this
    }
});
