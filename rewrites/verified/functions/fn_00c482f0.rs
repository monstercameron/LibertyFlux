// original: 0x00c482f0 ccamscripted_set_bit4 (proposed)
/// Record a new occupant: flag bit 4 plus a release/addref pair.
///
/// Flag bit 4 of `this + FLAGS` records whether `arg` is non-null,
/// and a non-null `this + VALUE` is cleared. Then the old slot
/// occupant (if any) is released (callee 1), and `arg` is stored into
/// the slot and retained (callee 2, called with `arg` in ecx); the
/// retain runs on every path. Returns whatever the retain call
/// returned.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c482f0(this: u32, arg: u32) -> u32 {
    const SLOT: u32 = 0x244;
    const VALUE: u32 = 0x24c;
    const FLAGS: u32 = 0x264;
    const KEEP: u8 = 0xc7;
    const RELEASE: u32 = 1;
    const RETAIN: u32 = 2;
    unsafe {
        let old = ((this + FLAGS) as *const u8).read();
        let mut bit: u8 = if arg != 0 { 1 } else { 0 };
        bit &= 1;
        bit <<= 4;
        ((this + FLAGS) as *mut u8).write(bit | (old & KEEP));
        if ((this + VALUE) as *const u32).read_unaligned() != 0 {
            ((this + VALUE) as *mut u32).write_unaligned(0);
        }
        let slot = this + SLOT;
        if (slot as *const u32).read_unaligned() != 0 {
            let occupant = (slot as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        (slot as *mut u32).write_unaligned(arg);
        lf_checker_rt::callee_thiscall!(RETAIN, u32, arg, slot)
    }
});
