// original: 0x00c492b0 ccamcinematic_swap_1f0 (proposed)
/// Swap the occupant of the slot at `this + SLOT` for `arg`.
///
/// The old occupant (if any) is released (callee 1) and the slot is
/// cleared. When `arg` is non-null it is stored into the slot and
/// retained (callee 2, called with `arg` in ecx). No value is returned.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c492b0(this: u32, arg: u32) -> u32 {
    const SLOT: u32 = 0x1f0;
    const RELEASE: u32 = 1;
    const RETAIN: u32 = 2;
    unsafe {
        let slot = this + SLOT;
        let occupant = (slot as *const u32).read_unaligned();
        if occupant != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        (slot as *mut u32).write_unaligned(0);
        if arg != 0 {
            (slot as *mut u32).write_unaligned(arg);
            lf_checker_rt::callee_thiscall!(RETAIN, u32, arg, slot);
        }
    }
    0
});
