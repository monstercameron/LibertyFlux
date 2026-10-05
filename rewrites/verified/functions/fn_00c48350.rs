// original: 0x00c48350 ccamscripted_set_vec1e0 (proposed)
/// Store a new 4-word vector from `arg`, releasing the old occupant.
///
/// When the slot at `this + SLOT` is occupied it is released first
/// (callee 1). When `this + VALUE` is non-null it is cleared. Then flag
/// bit 3 of `this + FLAGS` records whether `arg` is non-null, and the
/// four words at `arg` are copied to `this + DST`. Returns the last
/// word copied.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c48350(this: u32, arg: u32) -> u32 {
    const SLOT: u32 = 0x244;
    const VALUE: u32 = 0x24c;
    const FLAGS: u32 = 0x264;
    const DST: u32 = 0x1e0;
    const KEEP: u8 = 0xc7;
    const RELEASE: u32 = 1;
    unsafe {
        let slot = this + SLOT;
        let occupant = (slot as *const u32).read_unaligned();
        if occupant != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        if ((this + VALUE) as *const u32).read_unaligned() != 0 {
            ((this + VALUE) as *mut u32).write_unaligned(0);
        }
        let old = ((this + FLAGS) as *const u8).read();
        let mut bit: u8 = if arg != 0 { 1 } else { 0 };
        bit &= 1;
        bit <<= 3;
        ((this + FLAGS) as *mut u8).write(bit | (old & KEEP));
        let w0 = (arg as *const u32).read_unaligned();
        let w1 = ((arg + 4) as *const u32).read_unaligned();
        let w2 = ((arg + 8) as *const u32).read_unaligned();
        let w3 = ((arg + 12) as *const u32).read_unaligned();
        ((this + DST) as *mut u32).write_unaligned(w0);
        ((this + DST + 4) as *mut u32).write_unaligned(w1);
        ((this + DST + 8) as *mut u32).write_unaligned(w2);
        ((this + DST + 12) as *mut u32).write_unaligned(w3);
        w3
    }
});
