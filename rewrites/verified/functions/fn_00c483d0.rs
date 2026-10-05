// original: 0x00c483d0 ccamscripted_set_bit6 (proposed)
/// Toggle flag bit 6 from `arg`, then release/retain the slot.
///
/// Flag bit 6 of `this + FLAGS` is set to whether `arg` is non-null
/// (the original's xor/mask/xor sequence). Then the old occupant of
/// the slot at `this + SLOT` (if any) is released (callee 1), and
/// `arg` is stored into the slot and retained (callee 2, called with
/// `arg` in ecx). Returns whatever the retain call returned.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c483d0(this: u32, arg: u32) -> u32 {
    const SLOT: u32 = 0x240;
    const FLAGS: u32 = 0x264;
    const PRESENT_BIT: u8 = 0x40;
    const RELEASE: u32 = 1;
    const RETAIN: u32 = 2;
    unsafe {
        let want: u8 = if arg != 0 { PRESENT_BIT } else { 0 };
        let cur = ((this + FLAGS) as *const u8).read();
        ((this + FLAGS) as *mut u8).write((cur & !PRESENT_BIT) | want);
        let slot = this + SLOT;
        let occupant = (slot as *const u32).read_unaligned();
        if occupant != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        (slot as *mut u32).write_unaligned(arg);
        lf_checker_rt::callee_thiscall!(RETAIN, u32, arg, slot)
    }
});
