// original: 0x00c482a0 ccamscripted_set_24c (proposed)
/// Replace the slot at `this + SLOT`, releasing the old occupant.
///
/// When the slot holds a non-null pointer it is released first
/// (callee 1, called with the occupant in ecx and the slot address on
/// the stack). Then flag bit 5 of `this + FLAGS` records whether `arg`
/// is non-null, and `arg` is stored at `this + VALUE`. Returns the old
/// flag byte masked with `KEEP` in the low byte.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c482a0(this: u32, arg: u32) -> u32 {
    const SLOT: u32 = 0x244;
    const VALUE: u32 = 0x24c;
    const FLAGS: u32 = 0x264;
    const PRESENT_BIT: u8 = 0x20;
    const KEEP: u8 = 0xc7;
    const RELEASE: u32 = 1;
    unsafe {
        let slot = this + SLOT;
        let occupant = (slot as *const u32).read_unaligned();
        if occupant != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        let old = ((this + FLAGS) as *const u8).read();
        let mut bit: u8 = if arg != 0 { 1 } else { 0 };
        bit &= 1;
        bit <<= 5;
        let new = bit | (old & KEEP);
        debug_assert_eq!(bit & !PRESENT_BIT, 0);
        ((this + FLAGS) as *mut u8).write(new);
        ((this + VALUE) as *mut u32).write_unaligned(arg);
        (old & KEEP) as u32
    }
});
